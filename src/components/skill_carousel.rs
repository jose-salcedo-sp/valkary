use crate::components::render_markdown;
use leptos::prelude::*;
use std::time::Duration;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen(
    inline_js = "export function copy_text(text) { navigator.clipboard.writeText(text); }"
)]
extern "C" {
    pub fn copy_text(text: &str);
}

#[cfg(not(feature = "hydrate"))]
pub fn copy_text(_text: &str) {}

#[derive(Clone, Copy)]
pub struct SkillTab {
    pub title: &'static str,
    pub src: &'static str,
}

#[component]
pub fn CopyButton(text: Signal<&'static str>) -> impl IntoView {
    let (copied, set_copied) = signal(false);

    let on_copy = move |_| {
        copy_text(text.get_untracked());
        set_copied.set(true);
        set_timeout(move || set_copied.set(false), Duration::from_millis(1500));
    };

    view! {
        <button
            type="button"
            aria-label="Copy markdown to clipboard"
            on:click=on_copy
            class="inline-flex cursor-pointer items-center gap-1.5 rounded px-2 py-1 font-mono text-xs text-zinc-400 transition hover:bg-zinc-800 hover:text-white"
        >
            {move || {
                if copied.get() {
                    view! {
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                            class="h-3.5 w-3.5 text-emerald-400"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M16.704 4.153a.75.75 0 0 1 .143 1.052l-8 10.5a.75.75 0 0 1-1.127.075l-4.5-4.5a.75.75 0 0 1 1.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 0 1 1.05-.143Z"
                                clip-rule="evenodd"
                            />
                        </svg>
                        <span class="text-emerald-400">"Copied"</span>
                    }
                        .into_any()
                } else {
                    view! {
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="h-3.5 w-3.5"
                        >
                            <rect width="14" height="14" x="8" y="8" rx="2" ry="2"/>
                            <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>
                        </svg>
                        <span>"Copy"</span>
                    }
                        .into_any()
                }
            }}
        </button>
    }
}

#[component]
pub fn SkillCarousel(
    skills: &'static [SkillTab],
    #[prop(default = true)] show_frontmatter: bool,
) -> impl IntoView {
    if skills.is_empty() {
        return ().into_any();
    }

    let len = skills.len();
    let (active, set_active) = signal(0usize);

    let prev = move |_| {
        set_active.update(|idx| {
            *idx = if *idx == 0 { len - 1 } else { *idx - 1 };
        });
    };
    let next = move |_| {
        set_active.update(|idx| {
            *idx = (*idx + 1) % len;
        });
    };

    let current_src =
        Signal::derive(move || skills.get(active.get()).copied().unwrap_or(skills[0]).src);

    view! {
        <div class="my-6 overflow-hidden rounded-lg border border-zinc-800 bg-zinc-900/50">
            <div class="flex items-center justify-between border-b border-zinc-800 bg-zinc-900 px-2">
                <div
                    role="tablist"
                    aria-label="Skills"
                    class="flex gap-1 overflow-x-auto py-1.5"
                >
                    {skills
                        .iter()
                        .enumerate()
                        .map(|(idx, skill)| {
                            let is_active = move || active.get() == idx;
                            view! {
                                <button
                                    type="button"
                                    role="tab"
                                    aria-selected=move || is_active().to_string()
                                    on:click=move |_| set_active.set(idx)
                                    class=move || {
                                        if is_active() {
                                            "shrink-0 cursor-pointer rounded px-3 py-1.5 font-mono text-xs font-semibold text-white bg-zinc-800 border-b-2 border-red-400"
                                        } else {
                                            "shrink-0 cursor-pointer rounded px-3 py-1.5 font-mono text-xs text-zinc-400 transition hover:bg-zinc-800/50 hover:text-zinc-200"
                                        }
                                    }
                                >
                                    {skill.title}
                                </button>
                            }
                        })
                        .collect_view()}
                </div>

                <div class="ml-2 flex shrink-0 items-center gap-1 border-l border-zinc-800 pl-2">
                    <CopyButton text=current_src/>
                    {(len > 1)
                        .then(|| {
                            view! {
                                <button
                                    type="button"
                                    aria-label="Previous skill"
                                    on:click=prev
                                    class="cursor-pointer rounded px-2 py-1 text-xs text-zinc-400 hover:bg-zinc-800 hover:text-white"
                                >
                                    "←"
                                </button>
                                <span class="font-mono text-xs text-zinc-500">
                                    {move || format!("{}/{len}", active.get() + 1)}
                                </span>
                                <button
                                    type="button"
                                    aria-label="Next skill"
                                    on:click=next
                                    class="cursor-pointer rounded px-2 py-1 text-xs text-zinc-400 hover:bg-zinc-800 hover:text-white"
                                >
                                    "→"
                                </button>
                            }
                        })}
                </div>
            </div>

            <div
                role="tabpanel"
                class="markdown p-6"
                inner_html=move || render_markdown(current_src.get(), show_frontmatter)
            />
        </div>
    }
    .into_any()
}
