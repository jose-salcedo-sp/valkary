use crate::{Li, P, Post, Section, TocSection, Ul};
use leptos::prelude::*;

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
