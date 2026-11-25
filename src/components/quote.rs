use leptos::prelude::*;

const BASE_CLASS: &str = "border-l-2 border-red-400 bg-zinc-900/40 py-2 pr-4 pl-3 sm:pl-4 text-base sm:text-lg italic leading-relaxed text-zinc-300 rounded-r-md";
const CITE_CLASS: &str = "mt-2 not-italic text-xs sm:text-sm text-zinc-500";

#[component]
pub fn Quote(
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] cite: Option<String>,
    children: Children,
) -> impl IntoView {
    let merged_class = match class {
        Some(extra) if !extra.is_empty() => format!("{BASE_CLASS} {extra}"),
        _ => BASE_CLASS.to_string(),
    };
    let footer = cite
        .filter(|c| !c.is_empty())
        .map(|c| view! { <footer class=CITE_CLASS>{c}</footer> });

    view! {
        <blockquote class=merged_class>
            {children()}
            {footer}
        </blockquote>
    }
}
