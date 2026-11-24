pub mod components;
pub mod posts;

pub use components::{
    Markdown, P, Section, SeriesNav, SkillCarousel, SkillTab, Subtitle, TableOfContents,
};
pub use posts::{Post, SERIES, Series, TocSection, all_posts, find_post};

use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{A, Route, Router, Routes},
    hooks::use_params_map,
    path,
};

pub const BASE_PATH: &str = match option_env!("SITE_BASE") {
    Some(base) => base,
    None => "",
};

#[must_use]
pub fn site_url(path: &str) -> String {
    format!("{BASE_PATH}{path}")
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options root=BASE_PATH/>
                <MetaTags/>
                <Stylesheet id="leptos" href=site_url("/pkg/blog.css")/>
            </head>
            <body class="min-h-screen bg-zinc-950 text-zinc-100 antialiased">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Blog"/>
        <Router base=BASE_PATH>
            <div class="mx-auto max-w-2xl px-6 py-12">
                <header class="mb-12 border-b border-zinc-800 pb-6">
                    <nav class="flex items-center justify-between">
                        <A href=site_url("/") attr:class="text-xl font-semibold tracking-tight text-white hover:text-zinc-300">
                            "Pepe's Super Serious Blog"
                        </A>
                    </nav>
                </header>
                <main>
                    <Routes fallback=|| view! { <p class="text-zinc-400">"Page not found."</p> }>
                        <Route path=StaticSegment("") view=HomePage/>
                        <Route path=path!("posts/:slug") view=PostPage/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <section class="space-y-8">
            <h1 class="text-3xl font-bold tracking-tight text-white">"Is HMR Fast?"</h1>
            <ul class="space-y-6">
                {SERIES
                    .iter()
                    .filter_map(|series| series.posts.first().map(|first| (series, first)))
                    .map(|(series, first)| {
                        let href = site_url(&format!("/posts/{}", first.slug));
                        view! {
                            <li class="rounded-lg border border-zinc-800 bg-zinc-900/50 p-6 transition hover:border-zinc-700">
                                <article class="space-y-2">
                                    <h2 class="text-xl font-semibold text-white">
                                        <A href=href attr:class="hover:underline">{series.title}</A>
                                    </h2>
                                    <time class="block text-sm text-zinc-500">{first.date}</time>
                                    <p class="text-zinc-300">{series.summary}</p>
                                </article>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </section>
    }
}

#[component]
fn PostPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.read().get("slug").unwrap_or_default();

    move || match find_post(&slug()) {
        Some((series, post)) => view! {
            <article class="space-y-6">
                <Title text=post.title/>
                <SeriesNav series=series post=post/>
                <header class="space-y-2">
                    <h1 class="text-3xl font-bold tracking-tight text-white">{post.title}</h1>
                    <time class="block text-sm text-zinc-500">{post.date}</time>
                </header>
                <TableOfContents series=series post=post/>
                {(post.body)()}
                <div class="pt-6 space-y-6">
                    <SeriesNav series=series post=post/>
                    <p>
                        <A href=site_url("/") attr:class="text-sm text-zinc-400 hover:text-white">
                            "← Back to all posts"
                        </A>
                    </p>
                </div>
            </article>
        }
        .into_any(),
        None => view! {
            <section class="space-y-4">
                <h1 class="text-2xl font-bold text-white">"Post not found"</h1>
                <p>
                    <A href=site_url("/") attr:class="text-sm text-zinc-400 hover:text-white">
                        "← Back to all posts"
                    </A>
                </p>
            </section>
        }
        .into_any(),
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    leptos::mount::hydrate_body(App);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_known_post_by_slug() {
        let found = find_post("introduction");
        assert!(found.is_some());
        assert_eq!(found.map(|(_, p)| p.title), Some("Introduction"));
        assert!(find_post("skills").is_some());
        assert!(find_post("non-existent").is_none());
    }

    #[test]
    fn resolves_previous_and_next_in_series() {
        const PARTS: &[Post] = &[
            Post {
                slug: "part-1",
                title: "Part 1",
                date: "2026-11-24",
                summary: "",
                sections: &[],
                body: posts::harness::INTRO_POST.body,
            },
            Post {
                slug: "part-2",
                title: "Part 2",
                date: "2026-11-25",
                summary: "",
                sections: &[],
                body: posts::harness::INTRO_POST.body,
            },
            Post {
                slug: "part-3",
                title: "Part 3",
                date: "2026-11-26",
                summary: "",
                sections: &[],
                body: posts::harness::INTRO_POST.body,
            },
        ];
        let series = Series {
            slug: "harness",
            title: "Harness",
            summary: "",
            posts: PARTS,
        };

        let (prev_0, next_0) = series.neighbors("part-1");
        assert!(prev_0.is_none());
        assert_eq!(next_0.map(|p| p.slug), Some("part-2"));

        let (prev_1, next_1) = series.neighbors("part-2");
        assert_eq!(prev_1.map(|p| p.slug), Some("part-1"));
        assert_eq!(next_1.map(|p| p.slug), Some("part-3"));

        let (prev_2, next_2) = series.neighbors("part-3");
        assert_eq!(prev_2.map(|p| p.slug), Some("part-2"));
        assert!(next_2.is_none());
    }

    #[test]
    fn renders_skill_markdown_with_frontmatter() {
        let md = "---\nname: ponytail\ndescription: lazy mode\n---\n# Ponytail\n- rule 1\n";
        let html = components::render_markdown(md, true);
        assert!(html.contains("name: ponytail"));
        assert!(html.contains("<h1>Ponytail</h1>"));
        assert!(html.contains("<li>rule 1</li>"));
    }
}
