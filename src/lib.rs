pub mod components;
pub mod series;

pub use components::{
    ABOUT_PHRASES, AboutSection, BlogIndex, CodeExcerpt, FEATURED_PROJECTS, FeaturedProject,
    HERO_PHRASES, HeroBackground, HeroSection, Li, Link, Markdown, Mermaid, P, PhraseStyle,
    ProjectsSection, Quote, Section, SeriesNav, SiteHeader, SkillCarousel, SkillTab, Subtitle,
    TableOfContents, Typewriter, TypewriterPhase, TypewriterPhrase, TypewriterState, Ul,
    WritingSection, render_markdown, render_mermaid,
};
pub use series::{Post, SERIES, Series, TocSection, all_posts, find_post, post_url};

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

#[must_use]
pub fn with_base(path: &str) -> String {
    site_url(path)
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" class="overflow-x-clip scroll-smooth motion-reduce:scroll-auto">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta
                    name="description"
                    content="Pepe Salcedo — Software engineer in México with four years of industry experience building performant agentic tooling, systems of provenance, memory, and Rust/C infrastructure."
                />
                <AutoReload options=options.clone()/>
                <HydrationScripts options root=BASE_PATH/>
                <MetaTags/>
                <link rel="icon" type="image/svg+xml" href=site_url("/favicon.svg")/>
                <Stylesheet id="leptos" href=site_url("/pkg/blog.css")/>
            </head>
            <body class="min-h-screen overflow-x-clip bg-zinc-950 text-zinc-100 antialiased">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Pepe Salcedo · Software Engineer"/>
        <Router base=BASE_PATH>
            <div class="mx-auto max-w-4xl px-4 py-8 sm:px-6 sm:py-12">
                <SiteHeader/>
                <main class="space-y-16 pb-12 sm:space-y-24 sm:pb-16">
                    <Routes fallback=|| view! { <p class="text-zinc-400">"Page not found."</p> }>
                        <Route path=StaticSegment("") view=HomePage/>
                        <Route path=StaticSegment("index.html") view=HomePage/>
                        <Route path=StaticSegment("blog") view=BlogIndexPage/>
                        <Route path=path!("blog/index.html") view=BlogIndexPage/>
                        <Route path=path!("posts/:series/:slug") view=PostPage/>
                        <Route path=path!("posts/:series/:slug/index.html") view=PostPage/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Title text="Pepe Salcedo · Software Engineer"/>
        <HeroSection/>
        <AboutSection/>
        <ProjectsSection/>
        <WritingSection/>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen(inline_js = "
export function scroll_to_top() {
    if (typeof window !== 'undefined') {
        window.scrollTo({ top: 0, left: 0, behavior: 'instant' });
    }
}
")]
extern "C" {
    fn scroll_to_top();
}

#[component]
fn BlogIndexPage() -> impl IntoView {
    #[cfg(feature = "hydrate")]
    Effect::new(move |_| {
        scroll_to_top();
    });

    view! {
        <Title text="Blog · Pepe Salcedo"/>
        <BlogIndex/>
    }
}

#[component]
fn PostPage() -> impl IntoView {
    let params = use_params_map();
    let series_slug = move || params.read().get("series").unwrap_or_default();
    let post_slug = move || params.read().get("slug").unwrap_or_default();

    #[cfg(feature = "hydrate")]
    Effect::new(move |_| {
        let _ = (series_slug(), post_slug());
        scroll_to_top();
    });

    move || {
        match find_post(&series_slug(), &post_slug()) {
        Some((series, post)) => view! {
            <article class="space-y-5 sm:space-y-6">
                <Title text=post.title/>
                <SeriesNav series=series post=post/>
                <header class="space-y-2">
                    <h1 class="text-2xl font-bold tracking-tight text-white sm:text-3xl">{post.title}</h1>
                    <time class="block text-xs text-zinc-500 sm:text-sm">{post.date}</time>
                </header>
                <TableOfContents series=series post=post/>
                {(post.body)()}
                <div class="pt-6 space-y-6">
                    <SeriesNav series=series post=post/>
                    <p>
                        <A href=site_url("/blog") attr:class="inline-flex min-h-11 items-center text-sm text-zinc-400 hover:text-white">
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
                    <A href=site_url("/blog") attr:class="inline-flex min-h-11 items-center text-sm text-zinc-400 hover:text-white">
                        "← Back to all posts"
                    </A>
                </p>
            </section>
        }
        .into_any(),
    }
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
    fn finds_known_post_by_series_and_slug() {
        let found = find_post("building-and-understanding-a-harness", "introduction");
        assert!(found.is_some());
        assert_eq!(found.map(|(_, p)| p.title), Some("Introduction"));
        assert!(
            find_post(
                "building-and-understanding-a-harness",
                "skills-and-citations"
            )
            .is_some()
        );
        assert!(find_post("building-and-understanding-a-harness", "non-existent").is_none());
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
                body: series::harness::INTRO_POST.body,
            },
            Post {
                slug: "part-2",
                title: "Part 2",
                date: "2026-11-25",
                summary: "",
                sections: &[],
                body: series::harness::INTRO_POST.body,
            },
            Post {
                slug: "part-3",
                title: "Part 3",
                date: "2026-11-26",
                summary: "",
                sections: &[],
                body: series::harness::INTRO_POST.body,
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
    fn site_url_and_post_url_prefix_base_path() {
        assert_eq!(site_url("/blog"), format!("{BASE_PATH}/blog"));
        assert_eq!(
            with_base("/profile_pic.jpg"),
            format!("{BASE_PATH}/profile_pic.jpg")
        );
        assert_eq!(
            with_base("/about_pic.jpg"),
            format!("{BASE_PATH}/about_pic.jpg")
        );

        let all = all_posts();
        let (series, first) = all[0];
        assert_eq!(
            post_url(&series, &first),
            format!("{BASE_PATH}/posts/building-and-understanding-a-harness/introduction")
        );
    }

    #[test]
    fn typewriter_state_machine_cycles_through_phrases() {
        const SAMPLE: &[TypewriterPhrase] = &[
            TypewriterPhrase::plain("Hi"),
            TypewriterPhrase::code_tag("Rust"),
        ];

        // Initial state starts on full first phrase in Paused mode for SSR / no-JS
        let mut state = TypewriterState::initial(SAMPLE);
        assert_eq!(state.phrase_index, 0);
        assert_eq!(state.phase, TypewriterPhase::Paused);
        assert_eq!(state.visible_text(SAMPLE), "Hi");
        assert_eq!(state.delay_ms(80, 45, 2000), 2000);

        // Step from Paused -> Deleting ("H")
        state = state.step(SAMPLE);
        assert_eq!(state.phase, TypewriterPhase::Deleting);
        assert_eq!(state.visible_text(SAMPLE), "H");
        assert_eq!(state.delay_ms(80, 45, 2000), 45);

        // Delete remaining char -> ""
        state = state.step(SAMPLE);
        assert_eq!(state.phase, TypewriterPhase::Deleting);
        assert_eq!(state.visible_text(SAMPLE), "");

        // Step at 0 chars advances to phrase 1 ("Rust") in Typing mode
        state = state.step(SAMPLE);
        assert_eq!(state.phrase_index, 1);
        assert_eq!(state.phase, TypewriterPhase::Typing);
        assert_eq!(state.visible_text(SAMPLE), "");
        assert_eq!(
            state.current_phrase(SAMPLE).map(|p| p.style),
            Some(PhraseStyle::CodeTag)
        );
        assert_eq!(state.delay_ms(80, 45, 2000), 80);

        // Type "R", "Ru", "Rus", "Rust"
        for expected in ["R", "Ru", "Rus", "Rust"] {
            state = state.step(SAMPLE);
            assert_eq!(state.visible_text(SAMPLE), expected);
            assert_eq!(state.phase, TypewriterPhase::Typing);
        }

        // Next step transitions to Paused on full "Rust"
        state = state.step(SAMPLE);
        assert_eq!(state.phase, TypewriterPhase::Paused);
        assert_eq!(state.visible_text(SAMPLE), "Rust");

        // Delete all 4 chars and wrap back to phrase 0
        for _ in 0..4 {
            state = state.step(SAMPLE);
        }
        assert_eq!(state.visible_text(SAMPLE), "");
        state = state.step(SAMPLE);
        assert_eq!(state.phrase_index, 0);
        assert_eq!(state.phase, TypewriterPhase::Typing);
    }
}
