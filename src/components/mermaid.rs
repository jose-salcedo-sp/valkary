use leptos::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use mermaid_rs_renderer::{LayoutConfig, RenderOptions, Theme, render_strict};

const BASE_CLASS: &str = "my-5 sm:my-6 overflow-x-auto rounded-xl border border-zinc-800 bg-zinc-900/50 p-4 sm:p-6 [&_svg]:mx-auto [&_svg]:h-auto [&_svg]:max-w-full";

include!(concat!(env!("OUT_DIR"), "/mermaid_cache.rs"));

#[component]
pub fn Mermaid(src: &'static str, #[prop(optional, into)] class: Option<String>) -> impl IntoView {
    let merged_class = match class {
        Some(extra) if !extra.is_empty() => format!("{BASE_CLASS} {extra}"),
        _ => BASE_CLASS.to_string(),
    };
    let rendered = render_mermaid(src);

    view! { <div class=merged_class inner_html=rendered/> }
}

/// Renders Mermaid diagram syntax to an SVG string styled for the blog's dark zinc + red-400 palette.
/// Uses build-time precomputed SVGs first (so WASM never invokes `std::time::Instant::now()`),
/// falling back to `mermaid_rs_renderer` on native targets and an HTML-escaped code block if invalid.
#[must_use]
pub fn render_mermaid(src: &str) -> String {
    if let Some(svg) = precomputed_mermaid_svg(src) {
        return svg.to_string();
    }
    #[cfg(not(target_arch = "wasm32"))]
    if let Ok(svg) = render_strict(src, blog_render_options()) {
        return svg;
    }
    format!(
        "<pre><code class=\"language-mermaid\">{}</code></pre>",
        escape_html(src)
    )
}

pub(crate) fn render_mermaid_container(src: &str) -> String {
    format!("<div class=\"{BASE_CLASS}\">{}</div>", render_mermaid(src))
}

#[cfg(not(target_arch = "wasm32"))]
fn blog_render_options() -> RenderOptions {
    let mut theme = Theme::dark();
    theme.font_family =
        "ui-sans-serif, system-ui, -apple-system, \"Segoe UI\", sans-serif".to_string();
    theme.font_size = 14.0;
    theme.background = "#09090b".to_string();
    theme.primary_color = "#18181b".to_string();
    theme.secondary_color = "#27272a".to_string();
    theme.tertiary_color = "#18181b".to_string();
    theme.primary_text_color = "#f4f4f5".to_string();
    theme.text_color = "#f4f4f5".to_string();
    theme.primary_border_color = "#3f3f46".to_string();
    theme.line_color = "#f87171".to_string();
    theme.edge_label_background = "#18181b".to_string();
    theme.cluster_background = "#18181b".to_string();
    theme.cluster_border = "#52525b".to_string();
    theme.sequence_actor_fill = "#18181b".to_string();
    theme.sequence_actor_border = "#52525b".to_string();
    theme.sequence_actor_line = "#52525b".to_string();
    theme.sequence_note_fill = "#27272a".to_string();
    theme.sequence_note_border = "#f87171".to_string();
    theme.sequence_activation_fill = "#27272a".to_string();
    theme.sequence_activation_border = "#f87171".to_string();
    theme.git_commit_label_color = "#f4f4f5".to_string();
    theme.git_commit_label_background = "#27272a".to_string();
    theme.git_tag_label_color = "#f4f4f5".to_string();
    theme.git_tag_label_background = "#18181b".to_string();
    theme.git_tag_label_border = "#f87171".to_string();
    theme.pie_title_text_color = "#f4f4f5".to_string();
    theme.pie_section_text_color = "#f4f4f5".to_string();
    theme.pie_legend_text_color = "#f4f4f5".to_string();
    theme.pie_stroke_color = "#09090b".to_string();
    theme.pie_outer_stroke_color = "#3f3f46".to_string();

    RenderOptions {
        theme,
        layout: LayoutConfig::default(),
    }
}

fn escape_html(src: &str) -> String {
    src.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
