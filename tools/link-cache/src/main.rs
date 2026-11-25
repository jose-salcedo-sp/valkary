use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::{Path, PathBuf},
    process::ExitCode,
    sync::LazyLock,
    time::Duration,
};

use regex::Regex;
use reqwest::{
    blocking::Client,
    header::{CONTENT_LENGTH, CONTENT_TYPE},
};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use url::Url;
use walkdir::WalkDir;

const MAX_ICON_BYTES: usize = 1024 * 1024;
const USER_AGENT: &str = "jose-salcedo-blog-link-cache/1.0";
static LINK_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<Link\b[^>]*?\bhref\s*=\s*"([^"]+)""#).expect("Link component pattern is valid")
});

#[derive(Debug, Default, Deserialize, Serialize)]
struct Cache {
    pages: BTreeMap<String, CachedPage>,
    sites: BTreeMap<String, CachedSite>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CachedPage {
    title: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct CachedSite {
    icon_path: String,
}

#[derive(Debug)]
struct FetchedPage {
    title: String,
    icon_urls: Vec<Url>,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:#?}");
            ExitCode::FAILURE
        }
    }
}

#[eros::context("refresh the static Link metadata cache")]
fn run() -> eros::Result<()> {
    let refresh = env::args().skip(1).any(|argument| argument == "--refresh");
    let root = repository_root()?;
    let cache_path = root.join("tools/link-cache/cache.json");
    let metadata_path = root.join("src/components/link_metadata.rs");
    let favicon_dir = root.join("public/favicons");
    let urls = find_link_urls(&root.join("src"))?;
    let mut cache = if refresh {
        Cache::default()
    } else {
        read_cache(&cache_path)?
    };

    cache.pages.retain(|url, _| urls.contains(url));
    let active_origins = urls
        .iter()
        .map(|url| parse_url(url))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|url| origin(&url))
        .collect::<BTreeSet<_>>();
    cache
        .sites
        .retain(|site_origin, _| active_origins.contains(site_origin));

    fs::create_dir_all(&favicon_dir)?;
    let client = Client::builder()
        .user_agent(USER_AGENT)
        .connect_timeout(Duration::from_secs(8))
        .timeout(Duration::from_secs(20))
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()?;
    let mut fetched_pages = BTreeMap::new();

    for url in &urls {
        let parsed_url = parse_url(url)?;
        let site_origin = origin(&parsed_url);
        let needs_page = !cache.pages.contains_key(url);
        let needs_icon = !cache.sites.contains_key(&site_origin);

        if needs_page || needs_icon {
            eprintln!("Fetching {url}");
            fetched_pages.insert(url.clone(), fetch_page(&client, &parsed_url)?);
        }

        if needs_page {
            let fetched = fetched_pages
                .get(url)
                .expect("a missing cache entry always fetches its page");
            cache.pages.insert(
                url.clone(),
                CachedPage {
                    title: fetched.title.clone(),
                },
            );
        }

        if needs_icon {
            let fetched = fetched_pages
                .get(url)
                .expect("a missing site icon always fetches its page");
            let icon_path = cache_site_icon(
                &client,
                &parsed_url,
                &site_origin,
                &fetched.icon_urls,
                &favicon_dir,
            )?;
            cache.sites.insert(site_origin, CachedSite { icon_path });
        }
    }

    remove_unused_icons(&favicon_dir, &cache)?;
    write_cache(&cache_path, &cache)?;
    write_metadata_module(&metadata_path, &urls, &cache)?;
    eprintln!(
        "Cached {} links across {} sites",
        cache.pages.len(),
        cache.sites.len()
    );
    Ok(())
}

#[eros::context("locate the repository root")]
fn repository_root() -> eros::Result<PathBuf> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .ok_or_else(|| eros::error!("link-cache must remain under tools/link-cache"))
}

#[eros::context("scan Rust sources for Link components")]
fn find_link_urls(source_dir: &Path) -> eros::Result<BTreeSet<String>> {
    let mut urls = BTreeSet::new();

    for entry in WalkDir::new(source_dir) {
        let entry = entry?;
        if !entry.file_type().is_file()
            || entry.path().extension().and_then(|value| value.to_str()) != Some("rs")
        {
            continue;
        }

        let source = fs::read_to_string(entry.path())?;
        urls.extend(link_urls_in_source(&source));
    }

    Ok(urls)
}

fn link_urls_in_source(source: &str) -> BTreeSet<String> {
    LINK_PATTERN
        .captures_iter(source)
        .filter_map(|captures| captures.get(1))
        .map(|matched| matched.as_str().to_owned())
        .collect()
}

#[eros::context("read the existing link cache")]
fn read_cache(path: &Path) -> eros::Result<Cache> {
    if !path.exists() {
        return Ok(Cache::default());
    }

    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}

#[eros::context("parse a Link URL")]
fn parse_url(url: &str) -> eros::Result<Url> {
    let parsed = Url::parse(url)?;
    if !matches!(parsed.scheme(), "http" | "https") {
        return Err(eros::error!("Link URL must use http or https: {url}"));
    }
    Ok(parsed)
}

fn origin(url: &Url) -> String {
    url.origin().ascii_serialization()
}

#[eros::context("fetch page metadata")]
fn fetch_page(client: &Client, url: &Url) -> eros::Result<FetchedPage> {
    let response = client.get(url.clone()).send()?.error_for_status()?;
    let final_url = response.url().clone();
    let document = Html::parse_document(&response.text()?);
    let title_selector = Selector::parse("title").expect("title is a valid static selector");
    let meta_selector = Selector::parse(r#"meta[property="og:title"], meta[name="twitter:title"]"#)
        .expect("metadata title is a valid static selector");
    let icon_selector =
        Selector::parse("link[rel]").expect("favicon link is a valid static selector");

    let title = document
        .select(&title_selector)
        .find_map(|element| nonempty(element.text().collect::<Vec<_>>().join(" ")))
        .or_else(|| {
            document
                .select(&meta_selector)
                .find_map(|element| element.value().attr("content").and_then(nonempty))
        })
        .unwrap_or_else(|| {
            final_url
                .host_str()
                .unwrap_or(final_url.as_str())
                .to_owned()
        });
    let icon_urls = document
        .select(&icon_selector)
        .filter(|element| {
            element.value().attr("rel").is_some_and(|rel| {
                rel.split_ascii_whitespace().any(|value| {
                    value.eq_ignore_ascii_case("icon") || value.eq_ignore_ascii_case("shortcut")
                })
            })
        })
        .filter_map(|element| element.value().attr("href"))
        .filter_map(|href| final_url.join(href).ok())
        .collect();

    Ok(FetchedPage {
        title: collapse_whitespace(&title),
        icon_urls,
    })
}

fn nonempty(value: impl AsRef<str>) -> Option<String> {
    let value = value.as_ref().trim();
    (!value.is_empty()).then(|| value.to_owned())
}

fn collapse_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[eros::context("download and store a site favicon")]
fn cache_site_icon(
    client: &Client,
    page_url: &Url,
    site_origin: &str,
    declared_icons: &[Url],
    favicon_dir: &Path,
) -> eros::Result<String> {
    let fallback = page_url.join("/favicon.ico")?;
    let candidates = declared_icons
        .iter()
        .cloned()
        .chain(std::iter::once(fallback));

    for candidate in candidates {
        match download_icon(client, &candidate) {
            Ok((bytes, extension)) => {
                let filename = icon_filename(page_url, site_origin, extension);
                fs::write(favicon_dir.join(&filename), bytes)?;
                return Ok(format!("/favicons/{filename}"));
            }
            Err(error) => {
                eprintln!("Skipping favicon candidate {candidate}: {error:#}");
            }
        }
    }

    Err(eros::error!(
        "could not download a favicon for {site_origin}"
    ))
}

#[eros::context("download a favicon candidate")]
fn download_icon(client: &Client, url: &Url) -> eros::Result<(Vec<u8>, &'static str)> {
    let response = client.get(url.clone()).send()?.error_for_status()?;
    if response
        .headers()
        .get(CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<usize>().ok())
        .is_some_and(|length| length > MAX_ICON_BYTES)
    {
        return Err(eros::error!("favicon exceeds {MAX_ICON_BYTES} bytes"));
    }

    let content_type = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_owned();
    let bytes = response.bytes()?.to_vec();
    if bytes.is_empty() || bytes.len() > MAX_ICON_BYTES {
        return Err(eros::error!("favicon has an invalid size"));
    }

    Ok((bytes, icon_extension(&content_type, url)))
}

fn icon_extension(content_type: &str, url: &Url) -> &'static str {
    if content_type.contains("svg") {
        "svg"
    } else if content_type.contains("png") {
        "png"
    } else if content_type.contains("webp") {
        "webp"
    } else if content_type.contains("jpeg") {
        "jpg"
    } else if content_type.contains("gif") {
        "gif"
    } else if url.path().to_ascii_lowercase().ends_with(".svg") {
        "svg"
    } else if url.path().to_ascii_lowercase().ends_with(".png") {
        "png"
    } else {
        "ico"
    }
}

fn icon_filename(url: &Url, site_origin: &str, extension: &str) -> String {
    let host = url
        .host_str()
        .unwrap_or("site")
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '-'
            }
        })
        .collect::<String>();
    let hash = format!("{:x}", Sha256::digest(site_origin.as_bytes()));
    format!("{host}-{}.{}", &hash[..10], extension)
}

#[eros::context("remove favicons for links no longer present")]
fn remove_unused_icons(favicon_dir: &Path, cache: &Cache) -> eros::Result<()> {
    let active = cache
        .sites
        .values()
        .filter_map(|site| site.icon_path.rsplit('/').next())
        .collect::<BTreeSet<_>>();

    for entry in fs::read_dir(favicon_dir)? {
        let entry = entry?;
        let filename = entry.file_name();
        let filename = filename.to_string_lossy();
        if filename != "default.svg" && !active.contains(filename.as_ref()) {
            fs::remove_file(entry.path())?;
        }
    }
    Ok(())
}

#[eros::context("write the link cache manifest")]
fn write_cache(path: &Path, cache: &Cache) -> eros::Result<()> {
    let mut json = serde_json::to_string_pretty(cache)?;
    json.push('\n');
    fs::write(path, json)?;
    Ok(())
}

#[eros::context("generate the compiled Link metadata lookup")]
fn write_metadata_module(path: &Path, urls: &BTreeSet<String>, cache: &Cache) -> eros::Result<()> {
    let mut source = String::from(
        r#"// Generated by tools/link-cache. Do not edit by hand.

#[derive(Clone, Copy)]
pub(super) struct LinkMetadata {
    pub(super) title: &'static str,
    pub(super) icon_path: &'static str,
}

pub(super) fn find(href: &str) -> Option<LinkMetadata> {
    match href {
"#,
    );

    for url in urls {
        let page = cache
            .pages
            .get(url)
            .expect("every discovered Link has cached page metadata");
        let parsed = Url::parse(url).expect("cache keys were validated as URLs");
        let site = cache
            .sites
            .get(&origin(&parsed))
            .expect("every discovered Link has a cached site favicon");
        source.push_str(&format!(
            "        {url:?} => Some(LinkMetadata {{\n\
             \x20           title: {:?},\n\
             \x20           icon_path: {:?},\n\
             \x20       }}),\n",
            page.title, site.icon_path
        ));
    }

    source.push_str("        _ => None,\n    }\n}\n");
    fs::write(path, source)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_static_link_hrefs_and_deduplicates_them() {
        let source = r#"
            <Link href="https://example.com/one"/>
            <Link
                title="Example"
                href = "https://example.com/two"
            />
            <Link href="https://example.com/one"/>
            <Link href=dynamic_url/>
        "#;

        assert_eq!(
            link_urls_in_source(source),
            BTreeSet::from([
                "https://example.com/one".to_owned(),
                "https://example.com/two".to_owned(),
            ])
        );
    }

    #[test]
    fn ignores_similarly_named_components() {
        let source = r#"
            <LinkedImage href="https://example.com/image"/>
            <LinkCard href="https://example.com/card"/>
        "#;

        assert!(link_urls_in_source(source).is_empty());
    }
}
