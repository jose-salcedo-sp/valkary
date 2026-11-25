use crate::{Li, Link, P, Post, Section, TocSection, Ul};
use leptos::prelude::*;

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
