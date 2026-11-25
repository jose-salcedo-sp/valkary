use leptos::prelude::*;
use std::{str::FromStr, sync::LazyLock};
use syntect::{
    easy::HighlightLines,
    highlighting::{
        Color, FontStyle, ScopeSelectors, StyleModifier, Theme, ThemeItem, ThemeSettings,
    },
    html::{IncludeBackground, styled_line_to_highlighted_html},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
static CODE_THEME: LazyLock<Theme> = LazyLock::new(site_theme);

#[component]
pub fn CodeExcerpt(code: &'static str) -> impl IntoView {
    let highlighted = highlight_rust(code);

    view! {
        <pre class="my-4 overflow-x-auto rounded-lg border border-zinc-800 bg-zinc-950 p-4 text-sm leading-relaxed">
            <code class="language-rust" inner_html=highlighted></code>
        </pre>
    }
}

fn highlight_rust(code: &str) -> String {
    let syntax = SYNTAX_SET
        .find_syntax_by_extension("rs")
        .expect("Syntect includes the Rust syntax");
    let mut highlighter = HighlightLines::new(syntax, &CODE_THEME);

    LinesWithEndings::from(code)
        .map(|line| {
            let regions = highlighter
                .highlight_line(line, &SYNTAX_SET)
                .expect("static Rust excerpts are valid UTF-8");
            styled_line_to_highlighted_html(&regions, IncludeBackground::No)
                .expect("Syntect can render highlighted Rust")
        })
        .collect()
}

fn site_theme() -> Theme {
    Theme {
        name: Some("Valkary".into()),
        author: Some("Pepe Salcedo".into()),
        settings: ThemeSettings {
            foreground: Some(rgb(0xd4d4d8)),
            background: Some(rgb(0x09090b)),
            caret: Some(rgb(0xfb7185)),
            selection: Some(rgb(0x3f3f46)),
            accent: Some(rgb(0xfb7185)),
            ..ThemeSettings::default()
        },
        scopes: vec![
            rule("comment", 0x71717a, Some(FontStyle::ITALIC)),
            rule("string", 0xfda4af, None),
            rule(
                "constant.numeric, constant.language, constant.character",
                0xfcd34d,
                None,
            ),
            rule("keyword, storage", 0xfb7185, None),
            rule(
                "entity.name.type, support.type, storage.type",
                0x67e8f9,
                None,
            ),
            rule("entity.name.function, support.function", 0xf4f4f5, None),
            rule("variable.language, variable.parameter", 0xc4b5fd, None),
            rule("entity.name.macro, support.macro", 0xd8b4fe, None),
            rule("punctuation, meta.brace", 0xa1a1aa, None),
        ],
    }
}

fn rule(scope: &str, foreground: u32, font_style: Option<FontStyle>) -> ThemeItem {
    ThemeItem {
        scope: ScopeSelectors::from_str(scope).expect("valid TextMate scope selector"),
        style: StyleModifier {
            foreground: Some(rgb(foreground)),
            font_style,
            ..StyleModifier::default()
        },
    }
}

const fn rgb(value: u32) -> Color {
    Color {
        r: ((value >> 16) & 0xff) as u8,
        g: ((value >> 8) & 0xff) as u8,
        b: (value & 0xff) as u8,
        a: 0xff,
    }
}
