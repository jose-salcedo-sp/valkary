use crate::{Li, Link, P, Post, Section, SkillCarousel, SkillTab, TocSection, Ul};
use leptos::prelude::*;

const RUST_SKILLS: &[SkillTab] = &[
    SkillTab {
        title: "rust-patterns",
        src: include_str!("../skills/rust-patterns.md"),
    },
    SkillTab {
        title: "rust-invalid-states",
        src: include_str!("../skills/rust-invalid-states.md"),
    },
    SkillTab {
        title: "rust-ownership",
        src: include_str!("../skills/rust-ownership.md"),
    },
    SkillTab {
        title: "rust-errors",
        src: include_str!("../skills/rust-errors.md"),
    },
    SkillTab {
        title: "rust-traits-generics",
        src: include_str!("../skills/rust-traits-generics.md"),
    },
    SkillTab {
        title: "rust-builders",
        src: include_str!("../skills/rust-builders.md"),
    },
    SkillTab {
        title: "rust-iterators",
        src: include_str!("../skills/rust-iterators.md"),
    },
    SkillTab {
        title: "rust-concurrency",
        src: include_str!("../skills/rust-concurrency.md"),
    },
    SkillTab {
        title: "rust-unsafe",
        src: include_str!("../skills/rust-unsafe.md"),
    },
    SkillTab {
        title: "rust-modules",
        src: include_str!("../skills/rust-modules.md"),
    },
    SkillTab {
        title: "rust-port-adapter-testing",
        src: include_str!("../skills/rust-port-adapter-testing.md"),
    },
];

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
                    <Link href="https://rust-lang.github.io/api-guidelines/" title="Rust API Guidelines" />
                </Li>
                <Li>
                    <Link href="https://www.gpui.rs/" title="GPUI" />
                </Li>
                <Li>
                    <Link href="https://github.com/longbridge/gpui-component" title="GPUI Component Kit" />
                </Li>
                <Li>
                    <Link href="https://github.com/tokio-rs/axum" title="Axum" />
                </Li>
                <Li>
                    <Link href="https://leptos.dev/" title="Leptos full-stack Rust framework (powering this blog)" />
                </Li>
                <Li>
                    <Link href="https://arxiv.org/html/2609.00006v1" title="Harness Engineering: Anatomy, Architecture, and Evolution of Coding Agents -- A Source-Code Study of Eleven Systems" />
                </Li>
                <Li>
                    <Link href="https://www.langchain.com/blog/the-anatomy-of-an-agent-harness" title="The Anatomy of an Agent Harness" />
                </Li>
                <Li>
                    <Link href="https://prismml.com/news/bonsai-2-family" title="Bonsai 2 family" />
                </Li>
                <Li>
                    <Link href="https://prismml.mintlify.app/models/bonsai-2-9b" title="Quickstart guide for Bonsai 2 9B" />
                </Li>
            </Ul>
        </Section>
    }
    .into_any()
}
