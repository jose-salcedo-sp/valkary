use crate::{ABOUT_PHRASES, HERO_PHRASES, SERIES, Typewriter, post_url, site_url};
use leptos::prelude::*;
use leptos_router::components::A;

const HEADER_BASE_CLASS: &str = "mb-8 pb-4 sm:mb-12 sm:pb-6";
const HERO_BASE_CLASS: &str = "relative flex min-h-[78vh] flex-col items-center justify-center overflow-visible py-16 text-center sm:min-h-[82vh] sm:py-24";
const ABOUT_BASE_CLASS: &str = "scroll-mt-20 space-y-8 pt-8 sm:space-y-10 sm:pt-12";
const PROJECTS_BASE_CLASS: &str = "scroll-mt-20 space-y-8 pt-8 sm:space-y-10 sm:pt-12";
const WRITING_BASE_CLASS: &str = "scroll-mt-20 space-y-8 pt-8 sm:space-y-10 sm:pt-12";
const BLOG_INDEX_BASE_CLASS: &str = "space-y-8 sm:space-y-10";
const SECTION_KICKER_CLASS: &str = "text-xs font-medium tracking-[8px] text-zinc-400 uppercase";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FeaturedProject {
    pub name: &'static str,
    pub stack: &'static str,
    pub url: &'static str,
    pub repo_label: &'static str,
    pub description: &'static str,
}

pub const FEATURED_PROJECTS: &[FeaturedProject] = &[
    FeaturedProject {
        name: "recall",
        stack: "Rust · Axum · pgvector",
        url: "https://github.com/jose-salcedo-sp/recall",
        repo_label: "github.com/jose-salcedo-sp/recall →",
        description: "Retrieval service with citation admission that gates memories through a System One classifier before generation. Combines hybrid BM25 + pgvector search with Reciprocal Rank Fusion, compile-time pipeline stages, claim verification, and full audit traces.",
    },
    FeaturedProject {
        name: "self-llm",
        stack: "Rust · Tauri · Postgres",
        url: "https://github.com/jose-salcedo-sp/self-llm",
        repo_label: "github.com/jose-salcedo-sp/self-llm →",
        description: "Local-first AI assistant engineered for strict data boundaries and self-hosted inference. Pairs a Rust backend and hybrid HNSW + full-text Postgres search with durable encrypted memory, policy-gated deep research, and evidence trace inspection.",
    },
    FeaturedProject {
        name: "final-server",
        stack: "C · Rust · TCP/UDP",
        url: "https://github.com/jose-salcedo-sp/final-server",
        repo_label: "github.com/jose-salcedo-sp/final-server →",
        description: "Distributed multi-tier messaging system backed by custom Rust TCP reverse-proxy load balancers with UDP heartbeat discovery, forking C logic middleware, and a MySQL data tier.",
    },
    FeaturedProject {
        name: "JavaRaytracer",
        stack: "Java · Computer Graphics · SIGGRAPH 2024",
        url: "https://github.com/jose-salcedo-sp/JavaRaytracer",
        repo_label: "github.com/jose-salcedo-sp/JavaRaytracer →",
        description: "Parallel 3D raytracing engine built from scratch without external graphics libraries, selected for presentation at ACM SIGGRAPH 2024. Implements recursive reflections, refraction, barycentric mesh intersections, and multi-threaded rendering.",
    },
];

const PILL_CLASS: &str = "inline-flex min-h-11 items-center justify-center rounded-full border border-[#242424] bg-zinc-950/90 px-6 py-2 text-xs font-medium tracking-widest text-zinc-400 uppercase transition duration-150 hover:-translate-y-0.5 hover:border-[#f43f5e]/40 hover:text-[#f43f5e] motion-reduce:transition-none motion-reduce:hover:translate-y-0";
const SOCIAL_ICON_CLASS: &str = "inline-flex min-h-11 min-w-11 items-center justify-center rounded-full text-zinc-400 transition duration-150 hover:bg-zinc-900 hover:text-[#f43f5e] motion-reduce:transition-none";

#[component]
pub fn SiteHeader(#[prop(optional, into)] class: Option<String>) -> impl IntoView {
    let merged_class = match class {
        Some(extra) if !extra.is_empty() => format!("{HEADER_BASE_CLASS} {extra}"),
        _ => HEADER_BASE_CLASS.to_string(),
    };

    view! {
        <header class=merged_class>
            <nav
                aria-label="Primary"
                class="flex flex-wrap items-center justify-between gap-3 sm:gap-4"
            >
                <div class="flex items-center gap-4 sm:gap-6">
                    <A
                        href=site_url("/")
                        attr:class="inline-flex min-h-11 items-center text-base font-semibold tracking-tight text-white transition duration-150 hover:text-[#f43f5e] sm:text-lg"
                    >
                        "Pepe Salcedo"
                    </A>
                    <div class="flex items-center gap-3 text-sm text-zinc-400 sm:gap-5">
                        <a
                            href=site_url("/#about")
                            class="inline-flex min-h-11 items-center transition duration-150 hover:text-white"
                        >
                            "About"
                        </a>
                        <a
                            href=site_url("/#projects")
                            class="inline-flex min-h-11 items-center transition duration-150 hover:text-white"
                        >
                            "Current Work"
                        </a>
                        <A
                            href=site_url("/blog")
                            attr:class="inline-flex min-h-11 items-center transition duration-150 hover:text-white"
                        >
                            "Blog"
                        </A>
                    </div>
                </div>

                <div class="flex items-center gap-1 sm:gap-1.5">
                    <a
                        href="https://github.com/jose-salcedo-sp"
                        target="_blank"
                        rel="noopener noreferrer"
                        aria-label="GitHub"
                        class=SOCIAL_ICON_CLASS
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                            aria-hidden="true"
                            class="h-5 w-5"
                        >
                            <path d="M12 .5C5.65.5.5 5.65.5 12A11.5 11.5 0 0 0 8.36 22.9c.58.1.79-.25.79-.56v-2c-3.2.7-3.88-1.36-3.88-1.36-.52-1.33-1.28-1.68-1.28-1.68-1.05-.71.08-.7.08-.7 1.16.08 1.77 1.2 1.77 1.2 1.03 1.76 2.7 1.25 3.36.96.1-.75.4-1.25.73-1.54-2.55-.29-5.24-1.28-5.24-5.69 0-1.26.45-2.28 1.19-3.09-.12-.29-.52-1.46.11-3.05 0 0 .97-.31 3.18 1.18a11.05 11.05 0 0 1 5.8 0c2.2-1.49 3.17-1.18 3.17-1.18.63 1.59.23 2.76.12 3.05.74.81 1.18 1.83 1.18 3.09 0 4.42-2.69 5.39-5.25 5.68.41.36.78 1.06.78 2.14v3.17c0 .31.21.67.8.56A11.5 11.5 0 0 0 23.5 12C23.5 5.65 18.35.5 12 .5Z"/>
                        </svg>
                    </a>
                    <a
                        href="https://www.linkedin.com/in/jose-salcedo-uribe/"
                        target="_blank"
                        rel="noopener noreferrer"
                        aria-label="LinkedIn"
                        class=SOCIAL_ICON_CLASS
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                            aria-hidden="true"
                            class="h-5 w-5"
                        >
                            <path d="M19 3a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14m-.5 15.5v-5.3a3.26 3.26 0 0 0-3.26-3.26c-.85 0-1.84.52-2.28 1.3v-1.11h-2.79v8.37h2.79v-4.93c0-.77.62-1.4 1.39-1.4a1.4 1.4 0 0 1 1.4 1.4v4.93h2.75M6.88 8.56a1.68 1.68 0 0 0 1.68-1.68c0-.93-.75-1.69-1.68-1.69a1.69 1.69 0 0 0-1.69 1.69c0 .93.76 1.68 1.69 1.68m1.39 9.94v-8.37H5.5v8.37h2.77Z"/>
                        </svg>
                    </a>
                    <a
                        href="mailto:pepoclesng@gmail.com"
                        aria-label="Contact via email"
                        class="inline-flex min-h-11 items-center gap-2 rounded-full px-3 text-zinc-400 transition duration-150 hover:bg-zinc-900 hover:text-[#f43f5e] motion-reduce:transition-none"
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="1.8"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            aria-hidden="true"
                            class="h-5 w-5"
                        >
                            <rect width="20" height="16" x="2" y="4" rx="2"/>
                            <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/>
                        </svg>
                        <span class="hidden text-xs font-medium tracking-wider uppercase md:inline-flex">
                            "Contact"
                        </span>
                    </a>
                </div>
            </nav>
        </header>
    }
}

#[component]
pub fn HeroBackground() -> impl IntoView {
    view! {
        <div
            aria-hidden="true"
            class="hero-rings-enter pointer-events-none absolute inset-0 flex items-center justify-center"
        >
            <div class="absolute h-40 w-40 rounded-full border border-[#333333] animate-ping motion-reduce:animate-none sm:h-50 sm:w-50"/>
            <div class="absolute h-55 w-55 rounded-full border border-[#333333] animate-ping motion-reduce:animate-none sm:h-75 sm:w-75"/>
            <div class="absolute h-70 w-70 rounded-full border border-[#333333] animate-ping motion-reduce:animate-none sm:h-106.25 sm:w-106.25"/>
            <div class="absolute h-82.5 w-82.5 rounded-full border border-[#f43f5e] animate-pulse motion-reduce:animate-none sm:h-137.5 sm:w-137.5"/>
            <div class="absolute h-97.5 w-97.5 rounded-full border border-[#333333] animate-ping motion-reduce:animate-none sm:h-156.25 sm:w-156.25"/>
        </div>
    }
}

#[component]
pub fn HeroSection(#[prop(optional, into)] class: Option<String>) -> impl IntoView {
    let merged_class = match class {
        Some(extra) if !extra.is_empty() => format!("{HERO_BASE_CLASS} {extra}"),
        _ => HERO_BASE_CLASS.to_string(),
    };

    view! {
        <section id="hero" class=merged_class>
            <HeroBackground/>

            <div class="relative z-10 flex flex-col items-center space-y-6 sm:space-y-8">
                <picture>
                    <source srcset=site_url("/profile_pic.webp") type="image/webp"/>
                    <img
                        src=site_url("/profile_pic.jpg")
                        alt="Pepe Salcedo"
                        width="144"
                        height="144"
                        decoding="async"
                        fetchpriority="high"
                        class="h-32 w-32 rounded-full object-cover ring-2 ring-zinc-800 sm:h-36 sm:w-36"
                    />
                </picture>

                <div class="space-y-3">
                    <p class="text-xs font-medium tracking-[10px] text-zinc-400 uppercase sm:text-sm">
                        "Software Engineer"
                    </p>
                    <h1 class="min-h-10 px-2 text-2xl font-semibold tracking-tight text-white sm:min-h-14 sm:px-6 sm:text-4xl md:text-5xl">
                        <Typewriter phrases=HERO_PHRASES/>
                    </h1>
                </div>

                <div class="z-20 flex flex-wrap items-center justify-center gap-2.5 pt-2 sm:gap-3">
                    <a href="#about" class=PILL_CLASS>
                        "About"
                    </a>
                    <a href="#projects" class=PILL_CLASS>
                        "Current Work"
                    </a>
                    <a href="#blog" class=PILL_CLASS>
                        "Blog"
                    </a>
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn AboutSection(#[prop(optional, into)] class: Option<String>) -> impl IntoView {
    let merged_class = match class {
        Some(extra) if !extra.is_empty() => format!("{ABOUT_BASE_CLASS} {extra}"),
        _ => ABOUT_BASE_CLASS.to_string(),
    };

    view! {
        <section id="about" class=merged_class>
            <div class="space-y-2">
                <p class=SECTION_KICKER_CLASS>"About me"</p>
                <h2 class="text-2xl font-bold tracking-tight text-white sm:text-3xl">
                    "Who am I?"
                </h2>
            </div>

            <div class="grid grid-cols-1 items-center gap-8 md:grid-cols-12 md:gap-10">
                <div class="flex justify-center md:col-span-5">
                    <picture class="w-full flex justify-center">
                        <source srcset=site_url("/about_pic.webp") type="image/webp"/>
                        <img
                            src=site_url("/about_pic.jpg")
                            alt="Pepe Salcedo"
                            width="1200"
                            height="1600"
                            loading="lazy"
                            decoding="async"
                            class="h-56 w-56 shrink-0 rounded-full border border-zinc-800 object-cover md:h-80 md:w-full md:rounded-xl"
                        />
                    </picture>
                </div>

                <div class="space-y-4 text-base leading-relaxed text-zinc-300 sm:text-lg md:col-span-7">
                    <p>
                        "I'm Pepe Salcedo, a software engineer in México with four years of industry experience. I started programming at age 11 with Ruby and gravitated toward Rust, C, systems programming, and local-first AI infrastructure. Today, I'm really keen on building agentic tooling, systems of provenance, and memory performantly — spanning citation-verified retrieval pipelines ("
                        <code class="rounded bg-zinc-900 px-1.5 py-0.5 font-mono text-sm text-red-300">
                            "recall"
                        </code>
                        "), self-hosted LLM assistants with hybrid vector and full-text search ("
                        <code class="rounded bg-zinc-900 px-1.5 py-0.5 font-mono text-sm text-red-300">
                            "self-llm"
                        </code>
                        "), distributed TCP/UDP servers and custom Rust load balancers ("
                        <code class="rounded bg-zinc-900 px-1.5 py-0.5 font-mono text-sm text-red-300">
                            "final-server"
                        </code>
                        "), and a multithreaded 3D raytracing engine ("
                        <code class="rounded bg-zinc-900 px-1.5 py-0.5 font-mono text-sm text-red-300">
                            "JavaRaytracer"
                        </code>
                        ", presented at ACM SIGGRAPH 2024)."
                    </p>
                    <p>
                        "I focus on "
                        <Typewriter phrases=ABOUT_PHRASES/>
                        " reliable systems from the ground up and learning in the process — whether that means gating retrieved context through a classifier before generation, writing custom reverse proxies over raw sockets, or tracing light rays across 3D scenes."
                    </p>
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn ProjectsSection(#[prop(optional, into)] class: Option<String>) -> impl IntoView {
    let merged_class = match class {
        Some(extra) if !extra.is_empty() => format!("{PROJECTS_BASE_CLASS} {extra}"),
        _ => PROJECTS_BASE_CLASS.to_string(),
    };

    view! {
        <section id="projects" class=merged_class>
            <div class="space-y-2">
                <p class=SECTION_KICKER_CLASS>"Current Work"</p>
                <h2 class="text-2xl font-bold tracking-tight text-white sm:text-3xl">
                    "Featured Systems & Research"
                </h2>
                <p class="text-sm leading-relaxed text-zinc-400 sm:text-base">
                    "Systems programming, retrieval infrastructure, distributed networking, and computer graphics."
                </p>
            </div>

            <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
                {FEATURED_PROJECTS
                    .iter()
                    .map(|project| {
                        view! {
                            <article class="flex flex-col justify-between rounded-xl border border-zinc-800 bg-zinc-900/50 p-5 transition duration-150 hover:border-zinc-700 sm:p-6">
                                <div class="space-y-2.5">
                                    <div class="flex flex-wrap items-center justify-between gap-2">
                                        <span class="font-mono text-xs text-red-400 uppercase">
                                            {project.stack}
                                        </span>
                                        <span class="font-mono text-xs text-zinc-400">
                                            "Open Source"
                                        </span>
                                    </div>
                                    <h3 class="text-lg font-semibold text-white">
                                        <a
                                            href=project.url
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            class="hover:text-red-300 hover:underline"
                                        >
                                            {project.name}
                                        </a>
                                    </h3>
                                    <p class="text-sm leading-relaxed text-zinc-300">
                                        {project.description}
                                    </p>
                                </div>
                                <div class="mt-4 pt-2">
                                    <a
                                        href=project.url
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="inline-flex min-h-11 items-center font-mono text-xs text-zinc-400 transition duration-150 hover:text-white"
                                    >
                                        {project.repo_label}
                                    </a>
                                </div>
                            </article>
                        }
                    })
                    .collect_view()}
            </div>

            <a
                href="https://github.com/jose-salcedo-sp/JavaRaytracer"
                target="_blank"
                rel="noopener noreferrer"
                class="block overflow-hidden rounded-xl"
            >
                <picture>
                    <source srcset=site_url("/java_ray_tracer.webp") type="image/webp"/>
                    <img
                        src=site_url("/java_ray_tracer.jpg")
                        alt="JavaRaytracer render exhibited at ACM SIGGRAPH 2024"
                        width="1920"
                        height="1080"
                        loading="lazy"
                        decoding="async"
                        class="w-full rounded-xl border border-zinc-800 object-cover transition duration-300 hover:scale-[1.01] motion-reduce:transition-none motion-reduce:hover:scale-100"
                    />
                </picture>
            </a>
        </section>
    }
}

#[component]
pub fn WritingSection(#[prop(optional, into)] class: Option<String>) -> impl IntoView {
    let merged_class = match class {
        Some(extra) if !extra.is_empty() => format!("{WRITING_BASE_CLASS} {extra}"),
        _ => WRITING_BASE_CLASS.to_string(),
    };

    view! {
        <section id="blog" class=merged_class>
            <div class="flex flex-wrap items-end justify-between gap-4">
                <div class="space-y-2">
                    <p class=SECTION_KICKER_CLASS>"Blog"</p>
                    <h2 class="text-2xl font-bold tracking-tight text-white sm:text-3xl">
                        "Technical Series & Notes"
                    </h2>
                    <p class="text-sm leading-relaxed text-zinc-400 sm:text-base">
                        "Long-form notes on agent harnesses, provenance, and systems architecture."
                    </p>
                </div>
                <A
                    href=site_url("/blog")
                    attr:class="inline-flex min-h-11 items-center text-sm font-medium text-red-400 transition duration-150 hover:text-red-300 hover:underline"
                >
                    "All blog posts →"
                </A>
            </div>

            <div class="space-y-6">
                {SERIES
                    .iter()
                    .map(|series| {
                        let first_href = series
                            .posts
                            .first()
                            .map(|first| post_url(series, first))
                            .unwrap_or_else(|| site_url("/blog"));

                        view! {
                            <article class="rounded-xl border border-zinc-800 bg-zinc-900/50 p-5 transition duration-150 hover:border-zinc-700 sm:p-6">
                                <div class="space-y-4">
                                    <div class="space-y-2">
                                        <div class="flex flex-wrap items-center justify-between gap-2">
                                            <span class="font-mono text-xs font-medium tracking-wider text-red-400 uppercase">
                                                "Featured Series"
                                            </span>
                                            {series.posts.first().map(|first| {
                                                view! {
                                                    <time class="text-xs text-zinc-400">{first.date}</time>
                                                }
                                            })}
                                        </div>

                                        <h3 class="text-lg font-semibold tracking-tight text-white sm:text-xl">
                                            <A href=first_href attr:class="hover:text-red-300 hover:underline">
                                                {series.title}
                                            </A>
                                        </h3>

                                        <p class="text-sm leading-relaxed text-zinc-300 sm:text-base">
                                            {series.summary}
                                        </p>
                                    </div>

                                    <div class="space-y-2 pt-2">
                                        <p class="text-xs font-medium tracking-[8px] text-zinc-400 uppercase">
                                            "Chapters"
                                        </p>
                                        <ol class="space-y-2 text-sm">
                                            {series
                                                .posts
                                                .iter()
                                                .enumerate()
                                                .map(|(idx, post)| {
                                                    let href = post_url(series, post);
                                                    let part_num = idx + 1;
                                                    view! {
                                                        <li>
                                                            <A
                                                                href=href
                                                                attr:class="group flex items-center justify-between gap-3 rounded-lg px-2 py-1.5 text-zinc-300 transition duration-150 hover:bg-zinc-800/60 hover:text-white"
                                                            >
                                                                <span class="truncate">
                                                                    <span class="mr-2 font-mono text-xs text-red-400">
                                                                        {format!("Part {part_num}.")}
                                                                    </span>
                                                                    <span class="font-medium group-hover:underline">
                                                                        {post.title}
                                                                    </span>
                                                                </span>
                                                                <time class="shrink-0 font-mono text-xs text-zinc-400">
                                                                    {post.date}
                                                                </time>
                                                            </A>
                                                        </li>
                                                    }
                                                })
                                                .collect_view()}
                                        </ol>
                                    </div>
                                </div>
                            </article>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}

#[component]
pub fn BlogIndex(#[prop(optional, into)] class: Option<String>) -> impl IntoView {
    let merged_class = match class {
        Some(extra) if !extra.is_empty() => format!("{BLOG_INDEX_BASE_CLASS} {extra}"),
        _ => BLOG_INDEX_BASE_CLASS.to_string(),
    };

    view! {
        <section class=merged_class>
            <header class="space-y-2">
                <p class=SECTION_KICKER_CLASS>"Blog"</p>
                <h1 class="text-2xl font-bold tracking-tight text-white sm:text-3xl">
                    "Technical Series & Notes"
                </h1>
                <p class="text-sm leading-relaxed text-zinc-300 sm:text-base">
                    "Deep dives on building Rust systems, AI agent harnesses, retrieval pipelines, and distributed infrastructure."
                </p>
            </header>

            <ul class="space-y-6">
                {SERIES
                    .iter()
                    .map(|series| {
                        let first_href = series
                            .posts
                            .first()
                            .map(|first| post_url(series, first))
                            .unwrap_or_else(|| site_url("/blog"));

                        view! {
                            <li class="rounded-xl border border-zinc-800 bg-zinc-900/50 p-5 transition duration-150 hover:border-zinc-700 sm:p-6">
                                <article class="space-y-4">
                                    <div class="space-y-2">
                                        <div class="flex flex-wrap items-center justify-between gap-2">
                                            <span class="font-mono text-xs font-medium tracking-wider text-red-400 uppercase">
                                                {format!("Series · {} parts", series.posts.len())}
                                            </span>
                                            {series.posts.first().map(|first| {
                                                view! {
                                                    <time class="text-xs text-zinc-400">{first.date}</time>
                                                }
                                            })}
                                        </div>

                                        <h2 class="text-lg font-semibold text-white sm:text-xl">
                                            <A href=first_href attr:class="hover:text-red-300 hover:underline">
                                                {series.title}
                                            </A>
                                        </h2>

                                        <p class="text-sm leading-relaxed text-zinc-300 sm:text-base">
                                            {series.summary}
                                        </p>
                                    </div>

                                    <div class="space-y-2 pt-2">
                                        <p class="text-xs font-medium tracking-[8px] text-zinc-400 uppercase">
                                            "Chapters"
                                        </p>
                                        <ol class="space-y-2 text-sm">
                                            {series
                                                .posts
                                                .iter()
                                                .enumerate()
                                                .map(|(idx, post)| {
                                                    let href = post_url(series, post);
                                                    let part_num = idx + 1;
                                                    view! {
                                                        <li>
                                                            <A
                                                                href=href
                                                                attr:class="group flex items-center justify-between gap-3 rounded-lg px-2 py-1.5 text-zinc-300 transition duration-150 hover:bg-zinc-800/60 hover:text-white"
                                                            >
                                                                <span class="truncate">
                                                                    <span class="mr-2 font-mono text-xs text-red-400">
                                                                        {format!("Part {part_num}.")}
                                                                    </span>
                                                                    <span class="font-medium group-hover:underline">
                                                                        {post.title}
                                                                    </span>
                                                                </span>
                                                                <time class="shrink-0 font-mono text-xs text-zinc-400">
                                                                    {post.date}
                                                                </time>
                                                            </A>
                                                        </li>
                                                    }
                                                })
                                                .collect_view()}
                                        </ol>
                                    </div>
                                </article>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </section>
    }
}
