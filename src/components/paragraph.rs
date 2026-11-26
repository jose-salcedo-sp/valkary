use leptos::prelude::*;

const BASE_CLASS: &str = "text-base sm:text-lg leading-relaxed text-zinc-300 [&_code]:rounded [&_code]:border [&_code]:border-red-400/25 [&_code]:bg-red-400/10 [&_code]:px-1.5 [&_code]:py-0.5 [&_code]:font-mono [&_code]:text-[0.85em] [&_code]:text-red-300";

#[component]
pub fn P(#[prop(optional, into)] class: Option<String>, children: Children) -> impl IntoView {
    let merged_class = match class {
        Some(extra) if !extra.is_empty() => format!("{BASE_CLASS} {extra}"),
        _ => BASE_CLASS.to_string(),
    };

    view! { <p class=merged_class>{children()}</p> }
}
