use leptos::prelude::*;

#[component]
pub fn Subtitle(title: &'static str, id: &'static str) -> impl IntoView {
    view! {
        <h2 class="group text-2xl font-bold tracking-tight text-white" id=id>
            <a
                href=format!("#{id}")
                class="inline-block cursor-pointer underline decoration-transparent decoration-4 underline-offset-4 transition-colors duration-200 group-hover:decoration-red-400"
            >
                <span class="inline-block max-w-0 overflow-hidden align-bottom opacity-0 transition-all duration-200 group-hover:mr-1.5 group-hover:max-w-4 group-hover:opacity-100">
                    "#"
                </span>
                <span>{title}</span>
            </a>
        </h2>
    }
}
