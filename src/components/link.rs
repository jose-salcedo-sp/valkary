use leptos::prelude::*;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen(inline_js = "
export function resolve_url_metadata(url, on_title, on_icon) {
    const cacheKey = 'link_meta:' + url;
    try {
        const cached = sessionStorage.getItem(cacheKey);
        if (cached) {
            const parsed = JSON.parse(cached);
            if (parsed.title) on_title(parsed.title);
            if (parsed.icon) on_icon(parsed.icon);
            return;
        }
    } catch (_) {}

    fetch('https://api.microlink.io/?url=' + encodeURIComponent(url))
        .then((r) => r.json())
        .then((res) => {
            if (res && res.status === 'success' && res.data) {
                const title = res.data.title || '';
                const icon = (res.data.logo && res.data.logo.url) || '';
                if (title) on_title(title);
                if (icon) on_icon(icon);
                try {
                    sessionStorage.setItem(cacheKey, JSON.stringify({ title, icon }));
                } catch (_) {}
            }
        })
        .catch(() => {});
}
")]
extern "C" {
    fn resolve_url_metadata(
        url: &str,
        on_title: &wasm_bindgen::closure::Closure<dyn Fn(String)>,
        on_icon: &wasm_bindgen::closure::Closure<dyn Fn(String)>,
    );
}

#[must_use]
pub fn default_link_meta(href: &str) -> (String, String) {
    let trimmed = href
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches('/');
    let domain = trimmed.split('/').next().unwrap_or(trimmed);
    let favicon = format!("https://www.google.com/s2/favicons?domain={domain}&sz=64");
    (trimmed.to_string(), favicon)
}

#[component]
pub fn Link(
    href: &'static str,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let (fallback_title, fallback_icon) = default_link_meta(href);
    let has_custom_title = title.is_some();
    let (label, set_label) = signal(title.unwrap_or(fallback_title));
    let (icon, set_icon) = signal(fallback_icon);

    #[cfg(feature = "hydrate")]
    {
        Effect::new(move |_| {
            let on_title = wasm_bindgen::closure::Closure::wrap(Box::new(move |resolved: String| {
                if !has_custom_title && !resolved.is_empty() {
                    set_label.set(resolved);
                }
            })
                as Box<dyn Fn(String)>);

            let on_icon = wasm_bindgen::closure::Closure::wrap(Box::new(move |resolved: String| {
                if !resolved.is_empty() {
                    set_icon.set(resolved);
                }
            })
                as Box<dyn Fn(String)>);

            resolve_url_metadata(href, &on_title, &on_icon);
            on_title.forget();
            on_icon.forget();
        });
    }
    #[cfg(not(feature = "hydrate"))]
    {
        let _ = (has_custom_title, &set_label, &set_icon);
    }

    const BASE_CLASS: &str = "mx-1 inline-flex max-w-full items-center gap-1.5 rounded-md border border-zinc-700/80 bg-zinc-900 px-2 py-0.5 align-baseline text-xs sm:text-sm font-medium text-zinc-100 transition hover:border-red-400/60 hover:bg-zinc-800";
    let merged_class = match class {
        Some(extra) if !extra.is_empty() => format!("{BASE_CLASS} {extra}"),
        _ => BASE_CLASS.to_string(),
    };

    view! {
        <a href=href target="_blank" rel="noopener noreferrer" class=merged_class>
            <img
                src=move || icon.get()
                alt=""
                loading="lazy"
                class="inline-block h-3.5 w-3.5 shrink-0 rounded-xs object-contain"
            />
            <span class="truncate">{move || label.get()}</span>
        </a>
    }
}
