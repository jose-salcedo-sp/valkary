use leptos::prelude::*;

const UL_BASE_CLASS: &str = "list-disc list-inside space-y-2 sm:space-y-1.5 text-base sm:text-lg leading-relaxed text-zinc-300";
const LI_BASE_CLASS: &str = "leading-relaxed text-zinc-300";

#[component]
pub fn Ul(
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let merged_class = match class {
        Some(extra) if !extra.is_empty() => format!("{UL_BASE_CLASS} {extra}"),
        _ => UL_BASE_CLASS.to_string(),
    };

    view! { <ul class=merged_class>{children.map(|c| c())}</ul> }
}

#[component]
pub fn Li(
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let merged_class = match class {
        Some(extra) if !extra.is_empty() => format!("{LI_BASE_CLASS} {extra}"),
        _ => LI_BASE_CLASS.to_string(),
    };

    view! { <li class=merged_class>{children.map(|c| c())}</li> }
}
