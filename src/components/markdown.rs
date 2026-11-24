use crate::components::skill_carousel::CopyButton;
use leptos::prelude::*;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd, html};

#[component]
pub fn Markdown(
    src: &'static str,
    #[prop(optional)] title: Option<&'static str>,
    #[prop(default = true)] show_frontmatter: bool,
) -> impl IntoView {
    let rendered = render_markdown(src, show_frontmatter);
    let src_signal = Signal::derive(move || src);

    view! {
        <div class="my-5 overflow-hidden rounded-lg border border-zinc-800 bg-zinc-900/50 sm:my-6">
            <div class="flex items-center justify-between gap-2 border-b border-zinc-800 bg-zinc-900 px-3 py-2 font-mono text-xs text-zinc-400 sm:px-4">
                <span class="truncate">{title.unwrap_or("SKILL.md")}</span>
                <CopyButton text=src_signal/>
            </div>
            <div class="markdown p-4 sm:p-6" inner_html=rendered/>
        </div>
    }
}

#[must_use]
pub fn render_markdown(src: &str, show_frontmatter: bool) -> String {
    let mut opts = Options::empty();
    opts.insert(
        Options::ENABLE_TABLES
            | Options::ENABLE_STRIKETHROUGH
            | Options::ENABLE_TASKLISTS
            | Options::ENABLE_YAML_STYLE_METADATA_BLOCKS,
    );

    let mut in_frontmatter = false;
    let parser = Parser::new_ext(src, opts).filter_map(|event| match event {
        Event::Start(Tag::MetadataBlock(_)) => {
            in_frontmatter = true;
            show_frontmatter.then_some(Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(
                "yaml".into(),
            ))))
        }
        Event::End(TagEnd::MetadataBlock(_)) => {
            in_frontmatter = false;
            show_frontmatter.then_some(Event::End(TagEnd::CodeBlock))
        }
        _ if in_frontmatter && !show_frontmatter => None,
        other => Some(other),
    });

    let mut out = String::new();
    html::push_html(&mut out, parser);
    out
}
