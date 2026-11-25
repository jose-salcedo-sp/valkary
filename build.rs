use mermaid_rs_renderer::{LayoutConfig, RenderOptions, Theme, render_strict};
use std::{
    collections::BTreeMap,
    env,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
};

fn main() {
    println!("cargo:rerun-if-changed=src");
    if env::var_os("LEPTOS_OUTPUT_NAME").is_none() {
        println!("cargo:rustc-env=LEPTOS_OUTPUT_NAME=blog");
    }

    let mut files = Vec::new();
    collect_files(Path::new("src"), &mut files);

    let options = blog_render_options();
    let mut cache = BTreeMap::new();

    for path in files {
        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };
        for diagram in extract_mermaid_sources(&content) {
            if !cache.contains_key(&diagram)
                && let Ok(svg) = render_strict(&diagram, options.clone())
            {
                cache.insert(diagram, svg);
            }
        }
    }

    let mut code = String::from(
        "#[must_use]\npub fn precomputed_mermaid_svg(src: &str) -> Option<&'static str> {\n    match src.trim() {\n",
    );
    for (src, svg) in &cache {
        let _ = writeln!(code, "        {:?} => Some({:?}),", src.trim(), svg);
    }
    code.push_str("        _ => None,\n    }\n}\n");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR must be set"));
    fs::write(out_dir.join("mermaid_cache.rs"), code).expect("failed to write mermaid_cache.rs");
}

fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, out);
        } else if matches!(path.extension().and_then(|e| e.to_str()), Some("rs" | "md")) {
            out.push(path);
        }
    }
}

fn is_mermaid_source(candidate: &str) -> bool {
    let first = candidate
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with("%%"))
        .unwrap_or("");
    const HEADERS: &[&str] = &[
        "flowchart",
        "graph",
        "sequenceDiagram",
        "classDiagram",
        "stateDiagram",
        "erDiagram",
        "pie",
        "gantt",
        "timeline",
        "journey",
        "mindmap",
        "gitGraph",
        "quadrantChart",
        "xychart",
        "sankey",
        "requirementDiagram",
        "C4Context",
        "kanban",
        "packet",
        "architecture",
        "block",
        "radar",
        "treemap",
        "zenuml",
    ];
    HEADERS.iter().any(|h| first.starts_with(h))
}

fn extract_mermaid_sources(content: &str) -> Vec<String> {
    let mut results = Vec::new();

    // 1. Extract raw string literals r#"..."# and r"..."
    let bytes = content.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let starts_raw_string = bytes[i] == b'r'
            && (i == 0 || !matches!(bytes[i - 1], b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_'));
        if starts_raw_string {
            let mut hashes = 0usize;
            let mut j = i + 1;
            while j < bytes.len() && bytes[j] == b'#' {
                hashes += 1;
                j += 1;
            }
            if j < bytes.len() && bytes[j] == b'"' {
                let start = j + 1;
                let mut k = start;
                let mut terminated = false;
                while k < bytes.len() {
                    if bytes[k] == b'"'
                        && k + hashes < bytes.len()
                        && bytes[k + 1..=k + hashes].iter().all(|&b| b == b'#')
                    {
                        if let Ok(s) = std::str::from_utf8(&bytes[start..k])
                            && is_mermaid_source(s)
                        {
                            results.push(s.to_string());
                        }
                        i = k + hashes + 1;
                        terminated = true;
                        break;
                    }
                    k += 1;
                }
                if !terminated {
                    i += 1;
                }
                continue;
            }
        }
        i += 1;
    }

    // 2. Extract ```mermaid ... ``` fenced code blocks
    let mut rest = content;
    while let Some(pos) = rest.find("```mermaid") {
        let after = &rest[pos + "```mermaid".len()..];
        let Some(newline) = after.find('\n') else {
            break;
        };
        let body_start = &after[newline + 1..];
        let Some(end) = body_start.find("```") else {
            break;
        };
        let diagram = &body_start[..end];
        if is_mermaid_source(diagram) {
            results.push(diagram.to_string());
        }
        rest = &body_start[end + 3..];
    }

    results
}

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
