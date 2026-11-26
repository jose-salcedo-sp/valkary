use crate::Series;
use leptos::prelude::*;

#[component]
pub fn SeriesSource(series: Series) -> impl IntoView {
    view! {
        <aside
            aria-labelledby="series-source-title"
            class="relative overflow-hidden rounded-xl border border-zinc-800 bg-zinc-900/60"
        >
            <div class="absolute inset-y-0 left-0 w-1 bg-red-400"/>
            <div class="absolute -right-12 -top-12 size-32 rounded-full bg-red-400/5 blur-2xl"/>

            <div class="relative space-y-2 p-5 sm:p-6">
                <p class="font-mono text-xs font-semibold uppercase tracking-widest text-red-300">
                    "Build along"
                </p>
                <h2 id="series-source-title" class="text-lg font-semibold text-white sm:text-xl">
                    "Source Code for this Series"
                </h2>
                <p class="max-w-2xl text-sm leading-relaxed text-zinc-400 sm:text-base">
                    "Follow the implementation, inspect every layer, and run the projects yourself."
                </p>
            </div>

            <ul class="relative divide-y divide-zinc-800 border-t border-zinc-800">
                {series.source_repos.iter().map(|repo| view! {
                    <li>
                        <a
                            href=repo.url
                            target="_blank"
                            rel="noopener noreferrer"
                            class="group/repo flex min-h-16 items-center gap-3 px-5 py-4 transition hover:bg-zinc-800/50 focus-visible:bg-zinc-800/50 focus-visible:outline-none sm:px-6"
                        >
                            <svg
                                aria-hidden="true"
                                viewBox="0 0 24 24"
                                class="size-5 shrink-0 fill-zinc-500 transition group-hover/repo:fill-red-300"
                            >
                                <path d="M12 .7a12 12 0 0 0-3.8 23.4c.6.1.8-.3.8-.6v-2.3c-3.3.7-4-1.4-4-1.4-.5-1.4-1.3-1.8-1.3-1.8-1.1-.7.1-.7.1-.7 1.2.1 1.8 1.2 1.8 1.2 1.1 1.8 2.8 1.3 3.5 1 .1-.8.4-1.3.8-1.6-2.7-.3-5.5-1.3-5.5-5.9 0-1.3.5-2.4 1.2-3.2-.1-.3-.5-1.5.1-3.2 0 0 1-.3 3.3 1.2a11.5 11.5 0 0 1 6 0c2.3-1.5 3.3-1.2 3.3-1.2.6 1.7.2 2.9.1 3.2.8.8 1.2 1.9 1.2 3.2 0 4.6-2.8 5.6-5.5 5.9.4.4.8 1.1.8 2.2v3.3c0 .3.2.7.8.6A12 12 0 0 0 12 .7Z"/>
                            </svg>

                            <span class="min-w-0 flex-1">
                                <span class="block truncate font-mono text-sm font-semibold text-zinc-100 transition group-hover/repo:text-red-300">
                                    {repo.name}
                                </span>
                                <span class="mt-0.5 block text-sm text-zinc-500">
                                    {repo.description}
                                </span>
                            </span>

                            <span
                                aria-hidden="true"
                                class="shrink-0 text-zinc-600 transition group-hover/repo:translate-x-0.5 group-hover/repo:text-red-300"
                            >
                                "↗"
                            </span>
                        </a>
                    </li>
                }).collect_view()}
            </ul>
        </aside>
    }
}
