use crate::Subtitle;
use leptos::prelude::*;

const BASE_CLASS: &str = "space-y-4 leading-relaxed text-zinc-300";

#[component]
pub fn Section(
    title: &'static str,
    id: &'static str,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let merged_class = match class {
        Some(extra) if !extra.is_empty() => format!("{BASE_CLASS} {extra}"),
        _ => BASE_CLASS.to_string(),
    };

    view! {
        <section class=merged_class>
            <Subtitle title=title id=id/>
            {children.map(|c| c())}
        </section>
    }
}
