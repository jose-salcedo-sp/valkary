use crate::{Li, Link, P, Post, Section, Series, SkillCarousel, SkillTab, TocSection, Ul};
use leptos::prelude::*;

const RUST_SKILLS: &[SkillTab] = &[
    SkillTab {
        title: "rust-patterns",
        src: include_str!("./skills/rust-patterns.md"),
    },
    SkillTab {
        title: "rust-invalid-states",
        src: include_str!("./skills/rust-invalid-states.md"),
    },
    SkillTab {
        title: "rust-ownership",
        src: include_str!("./skills/rust-ownership.md"),
    },
    SkillTab {
        title: "rust-errors",
        src: include_str!("./skills/rust-errors.md"),
    },
    SkillTab {
        title: "rust-traits-generics",
        src: include_str!("./skills/rust-traits-generics.md"),
    },
    SkillTab {
        title: "rust-builders",
        src: include_str!("./skills/rust-builders.md"),
    },
    SkillTab {
        title: "rust-iterators",
        src: include_str!("./skills/rust-iterators.md"),
    },
    SkillTab {
        title: "rust-concurrency",
        src: include_str!("./skills/rust-concurrency.md"),
    },
    SkillTab {
        title: "rust-unsafe",
        src: include_str!("./skills/rust-unsafe.md"),
    },
    SkillTab {
        title: "rust-modules",
        src: include_str!("./skills/rust-modules.md"),
    },
    SkillTab {
        title: "rust-port-adapter-testing",
        src: include_str!("./skills/rust-port-adapter-testing.md"),
    },
];

pub const SERIES: Series = Series {
    slug: "building-and-understanding-a-harness",
    title: "Building and Understanding an AI Harness",
    summary: "This is my first post on this blog, I am an AI enthusiast and I want to try and shed some light into how you can understand what your AI harness (Cursor, DeepSeek, etc.) is doing.",
    posts: &[INTRO_POST, WHY_AND_WHAT_POST, SKILLS_AND_CITATIONS_POST],
};

pub const INTRO_POST: Post = Post {
    slug: "introduction",
    title: "Introduction",
    date: "2026-11-24",
    summary: "This is my first post on this blog, I am an AI enthusiast and I want to try and shed some light into how you can understand what your AI harness (Cursor, DeepSeek, etc.) is doing.",
    sections: &[
        TocSection {
            id: "objectives",
            title: "Objectives",
        },
        TocSection {
            id: "rules",
            title: "Rules",
        },
    ],
    body: Introduction,
};

pub const SKILLS_AND_CITATIONS_POST: Post = Post {
    slug: "skills-and-citations",
    title: "Skills and Citations",
    date: "2026-11-24",
    summary: "Skills and citations used for this project",
    sections: &[
        TocSection {
            id: "skills",
            title: "Skills",
        },
        TocSection {
            id: "citations",
            title: "Citations",
        },
    ],
    body: Skills,
};

#[component]
fn Introduction() -> AnyView {
    view! {
        <div class="space-y-6 leading-relaxed text-zinc-300">
            <div class="space-y-4">
                <P>"I have been working using Cursor for most of my proffessional life (not really that long but still). I've become greatly interested in how these tools work, how do they interface with LLMs, what the hell a tool call is, skills, rules, etc. And I thought the best way to do this is to build one myself."</P>
                <P>"The purpose of this post (and probably series) is to delve deep into implementing a personal AI harness. The troubles I run into and explaining how each individual core component works or connects with an actual LLM."</P>
                <P>"So in the spirit of actually learning and transparency I will set myself up with some objectives and rules for the project."</P>
            </div>

            <Section title="Objectives" id="objectives">
                <Ul>
                    <Li>"Build a UI interface (preferrably pretty lol) to interact with my harness"</Li>
                    <Li>"Implementing a chat bot with conversation history and context (a cache we'll call agent memory)"</Li>
                    <Li>"Implementing a tool calling system and traces so we know what the hell is happening under the hood"</Li>
                    <Li>"Implementing agents and subagents with their own memory and rules (we'll call this personality)"</Li>
                    <Li>"Add skill and rule calling capabilities to the harness"</Li>
                    <Li>"Try to be performant and efficient with the use of memory and resources"</Li>
                    <Li>"Making something useful even if not for production use, but at least something really showcases what can be done"</Li>
                </Ul>
            </Section>

            <Section title="Rules" id="rules">
                <Ul>
                    <Li>"All of the code must be written in Rust"</Li>
                    <Li>"The blogpost are human written and not AI generated (unless it is to paraphrase or improve my writing)"</Li>
                    <Li>"The code will be written with AI in the most part but I will review it and keep it up to standard so I know what is actually going on while I implement the harness from scratch"</Li>
                    <Li>"Listing all my sources (including personal skills, notes, external blogposts and technical documentation) so I can reference them later"</Li>
                    <Li>"Using images and diagrams to help explain the code and concepts"</Li>
                </Ul>
            </Section>
        </div>
    }
    .into_any()
}

#[component]
fn Skills() -> AnyView {
    view! {
        <Section title="Skills" id="skills">
            <P>"These are the skills used for this project. Nothing is specific to a harness and they are highly opinionated to my specific flavor of rust development but I think they will be useful for anyone writting agentic idiomatic Rust that is made by machines but readable for humans (and vice versa)"</P>

            <SkillCarousel skills=RUST_SKILLS/>
        </Section>

        <Section title="Citations" id="citations">
            <P>"Core references, libraries, and documentation used across this series:"</P>
            <Ul>
                <Li>
                    "Rust API Guidelines:"
                    <Link href="https://rust-lang.github.io/api-guidelines/"/>
                </Li>
                <Li>
                    "GPUI (Zed's GPU-accelerated UI framework):"
                    <Link href="https://www.gpui.rs/"/>
                </Li>
                <Li>
                    "GPUI Component Kit:"
                    <Link href="https://github.com/longbridge/gpui-component"/>
                </Li>
                <Li>
                    "Axum web framework:"
                    <Link href="https://github.com/tokio-rs/axum"/>
                </Li>
                <Li>
                    "Leptos full-stack Rust framework (powering this blog):"
                    <Link href="https://leptos.dev/"/>
                </Li>
            </Ul>
        </Section>
    }
    .into_any()
}

pub const WHY_AND_WHAT_POST: Post = Post {
    slug: "why-what-and-how",
    title: "Why? What? and How?",
    date: "2026-11-24",
    summary: "Why? What? How? I am building",
    sections: &[
        TocSection {
            id: "why",
            title: "Why",
        },
        TocSection {
            id: "what",
            title: "What",
        },
        TocSection {
            id: "how",
            title: "How",
        },
        TocSection {
            id: "tech-stack",
            title: "Tech Stack",
        },
    ],
    body: Overview,
};

#[component]
fn Overview() -> AnyView {
    view! {
        <Section title="Why?" id="why">
            <P>"The first question when starting a project like this is 'Why would we even want to build all this ourselves when there are much better and ready to use tools out there?'"</P>
            <P>"And the answer comes in two parts; because we can and because (I don't know if many people feel the same way) but since I started doing agentic development some of the tools I use in my day to day life and which I have become quite proficient at working with; have become some sort of mythological creatures to me. They are there, they work (most of the time) but I don't have a clue of where stuff starts and ends. What a tool call actually works? How does an LLM even know which tools are available to it even to begin with? Can an LLM browse the web natively? (the answer is not really but we'll get to that) is it safe to let these man-made all knowing machines to roam around my PC freely and do as they please? (also no lol)"</P>
        </Section>

        <Section title="What?" id="what">
            <P>"So let's start thinking about what it is we're actually building. Strip away the hype and an AI harness is really just three pieces working together around a stateless LLM:"</P>
            <Ul>
                <Li>"The UI (the cockpit): where you chat, attach files, trigger skills and rules, and inspect live thinking traces and tool calls instead of staring at a black box"</Li>
                <Li>"The Agent Loop & Memory: the state machine that builds the prompt, injects rules and skills, manages conversation history/context windows, and decides when the model is done or needs another turn"</Li>
                <Li>"The Tool & Subagent Runtime: the execution layer that discovers available tools, runs shell commands or headless browsers, spawns isolated subagents with their own personalities, and feeds the results back to the model"</Li>
            </Ul>
            <P>"I won't get too much into the user interface design and implementation since that is the least interesting part of this project. I've been working on UI for most of my life and going into the design ideas and why I chose red over blue on a button doesn't get us any closer to understanding how a `Build me an automated trading platform that makes $1M MRR MAKE NO MISTAKES` prompt gets us a half-baked website that loses you your only $10 left in your bank account."</P>
        </Section>

        <Section title="How?" id="how">
            <P>
                "I will be writing a new `Part` in this series alongside each layer of the harness as I build the UI and backend server in public. All of the code will be open to clone, fork, star, contribute to, or poke around in the harness lives at"
                <Link href="https://github.com/jose-salcedo-sp/harness"/>
                "and the source for this Leptos + Tailwind blog lives at"
                <Link href="https://github.com/jose-salcedo-sp/valkary"/>
                "."
            </P>
        </Section>

        <Section title="Tech Stack" id="tech-stack">
            <P>
                "Keeping up with Rule #1 (everything in Rust, no exceptions), I'm skipping Electron and web wrappers entirely. Here is what we're rolling with:"
            </P>
            <Ul>
                <Li>
                    "GPUI"
                    <Link href="https://www.gpui.rs/"/>
                    "+ GPUI Kit"
                    <Link href="https://github.com/longbridge/gpui-component"/>
                    "for the desktop UI: Zed's GPU-accelerated UI framework paired with a GPUI component kit so we get clean, snappy native visuals out of the box without spending three weeks hand-rolling a text input or a scroll bar from scratch"
                </Li>
                <Li>
                    "Axum"
                    <Link href="https://github.com/tokio-rs/axum"/>
                    "for the backend server: handles the heavy lifting, streaming LLM responses, orchestrating the agent loop, managing conversation memory and traces, and executing tools and subagents"
                </Li>
                <Li>
                    "Tokio"
                    <Link href="https://tokio.rs/"/>
                    "+ Reqwest"
                    <Link href="https://github.com/seanmonstar/reqwest"/>
                    "+ Serde"
                    <Link href="https://serde.rs/"/>
                    "under the hood: async runtime, streaming HTTP calls to LLM providers, and typed JSON tool-call schemas so the compiler yells at us before the LLM does"
                </Li>
            </Ul>
        </Section>
    }
    .into_any()
}
