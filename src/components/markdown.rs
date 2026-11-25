use crate::components::{mermaid::render_mermaid_container, skill_carousel::CopyButton};
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
    let mut mermaid_buf: Option<String> = None;
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
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref lang)))
            if lang.split_whitespace().next() == Some("mermaid") =>
        {
            mermaid_buf = Some(String::new());
            None
        }
        Event::Text(text) if mermaid_buf.is_some() => {
            if let Some(buf) = &mut mermaid_buf {
                buf.push_str(&text);
            }
            None
        }
        Event::End(TagEnd::CodeBlock) if mermaid_buf.is_some() => {
            let diagram = mermaid_buf.take().unwrap_or_default();
            Some(Event::Html(render_mermaid_container(&diagram).into()))
        }
        other => Some(other),
    });

    let mut out = String::new();
    html::push_html(&mut out, parser);
    out
}
