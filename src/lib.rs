use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{A, Route, Router, Routes},
    hooks::use_params_map,
    path,
};

#[derive(Clone, Copy)]
pub struct Post {
    pub slug: &'static str,
    pub title: &'static str,
    pub date: &'static str,
    pub summary: &'static str,
    pub body: fn() -> AnyView,
}

pub const BASE_PATH: &str = match option_env!("SITE_BASE") {
    Some(base) => base,
    None => "",
};

#[must_use]
pub fn site_url(path: &str) -> String {
    format!("{BASE_PATH}{path}")
}

// ponytail: static slice in-memory; swap for markdown files / SQLite when post count grows
pub const POSTS: &[Post] = &[Post {
    slug: "building-and-understanding-a-harness",
    title: "Building and Understanding an AI Harness",
    date: "2026-11-24",
    summary: "This is my first post on this blog, I am an AI enthusiast and I want to try and shed some light into how you can understand what your AI harness (Cursor, DeepSeek, etc.) is doing.",
    body: HarnessPost,
}];

#[component]
fn HarnessPost() -> AnyView {
    view! {
        <div class="space-y-4 leading-relaxed text-zinc-300">
            <p>"I have been working using Cursor for most of my proffessional life (not really that long but still). I've become greatly interested in how these tools work, how do they interface with LLMs, what the hell a tool call is, skills, rules, etc. And I thought the best way to do this is to build one myself."</p>
            <p>"The purpose of this post (and probably series) is to delve deep into implementing a personal AI harness. The troubles I run into and explaining how each individual core component works or connects with an actual LLM."</p>
            <p>"So in the spirit of actually learning and transparency I will set myself up with some objectives and rules for the project."</p>

            <h2 class="text-2xl font-bold tracking-tight text-white" id="objectives">"Objectives"</h2>
            <ul class="list-disc list-inside">
                <li>"Build a UI interface (preferrably pretty lol) to interact with my harness"</li>
                <li>"Implementing a chat bot with conversation history and context (a cache we'll call agent memory)"</li>
                <li>"Implementing a tool calling system and traces so we know what the hell is happening under the hood"</li>
                <li>"Implementing agents and subagents with their own memory and rules (we'll call this personality)"</li>
                <li>"Add skill and rule calling capabilities to the harness"</li>
                <li>"Try to be performant and efficient with the use of memory and resources"</li>
                <li>"Making something useful even if not for production use, but at least something really showcases what can be done"</li>
            </ul>

            <h2 class="text-2xl font-bold tracking-tight text-white" id="rules">"Rules"</h2>
            <ul class="list-disc list-inside">
                <li>"All of the code must be written in Rust"</li>
                <li>"The blogpost are human written and not AI generated (unless it is to paraphrase or improve my writing)"</li>
                <li>"The code will be written with AI in the most part but I will review it and keep it up to standard so I know what is actually going on while I implement the harness from scratch"</li>
                <li>"Listing all my sources (including personal skills, notes, external blogposts and technical documentation) so I can reference them later"</li>
                <li>"Using images and diagrams to help explain the code and concepts"</li>
            </ul>
        </div>
    }
    .into_any()
}

#[must_use]
pub fn find_post(slug: &str) -> Option<Post> {
    POSTS.iter().copied().find(|post| post.slug == slug)
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
                {POSTS
                    .iter()
                    .map(|post| {
                        let href = site_url(&format!("/posts/{}", post.slug));
                        view! {
                            <li class="rounded-lg border border-zinc-800 bg-zinc-900/50 p-6 transition hover:border-zinc-700">
                                <article class="space-y-2">
                                    <h2 class="text-xl font-semibold text-white">
                                        <A href=href attr:class="hover:underline">{post.title}</A>
                                    </h2>
                                    <time class="block text-sm text-zinc-500">{post.date}</time>
                                    <p class="text-zinc-300">{post.summary}</p>
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
        Some(post) => view! {
            <article class="space-y-4">
                <Title text=post.title/>
                <h1 class="text-3xl font-bold tracking-tight text-white">{post.title}</h1>
                <time class="block text-sm text-zinc-500">{post.date}</time>
                {(post.body)()}
                <p class="pt-4">
                    <A href=site_url("/") attr:class="text-sm text-zinc-400 hover:text-white">
                        "← Back to all posts"
                    </A>
                </p>
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
        let post = find_post("building-and-understanding-a-harness");
        assert!(post.is_some());
        assert_eq!(
            post.map(|p| p.title),
            Some("Building and Understanding an AI Harness")
        );
        assert!(find_post("non-existent").is_none());
    }
}
