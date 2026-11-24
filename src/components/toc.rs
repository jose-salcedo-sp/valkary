use crate::{Post, Series, site_url};
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn TableOfContents(series: Series, post: Post) -> impl IntoView {
    let has_multiple_posts = series.posts.len() > 1;
    let has_sections = !post.sections.is_empty();

    if !has_multiple_posts && !has_sections {
        return ().into_any();
    }

    view! {
        <nav
            aria-label="Table of contents"
            class="my-6 rounded-lg border border-zinc-800 bg-zinc-900/50 p-5"
        >
            <p class="text-xs font-semibold tracking-wider text-zinc-400 uppercase">
                {format!("Series · {}", series.title)}
            </p>

            <ol class="mt-3 space-y-2 text-sm">
                {series
                    .posts
                    .iter()
                    .enumerate()
                    .map(|(idx, part)| {
                        let is_current = part.slug == post.slug;
                        let part_num = idx + 1;
                        let href = site_url(&format!("/posts/{}", part.slug));
                        view! {
                            <li>
                                {if is_current {
                                    view! {
                                        <div class="font-semibold text-white">
                                            <span class="mr-2 text-red-400">
                                                {format!("Part {part_num}.")}
                                            </span>
                                            <span>{part.title}</span>
                                        </div>
                                        {(!part.sections.is_empty())
                                            .then(|| {
                                                view! {
                                                    <ul class="mt-2 ml-5 space-y-1 border-l border-zinc-800 pl-3 text-zinc-400">
                                                        {part
                                                            .sections
                                                            .iter()
                                                            .map(|sec| {
                                                                view! {
                                                                    <li>
                                                                        <a
                                                                            href=format!("#{}", sec.id)
                                                                            class="hover:text-white hover:underline"
                                                                        >
                                                                            {sec.title}
                                                                        </a>
                                                                    </li>
                                                                }
                                                            })
                                                            .collect_view()}
                                                    </ul>
                                                }
                                            })}
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <A
                                            href=href
                                            attr:class="text-zinc-400 transition-colors hover:text-white hover:underline"
                                        >
                                            <span class="mr-2 text-zinc-500">
                                                {format!("Part {part_num}.")}
                                            </span>
                                            <span>{part.title}</span>
                                        </A>
                                    }
                                        .into_any()
                                }}
                            </li>
                        }
                    })
                    .collect_view()}
            </ol>
        </nav>
    }
    .into_any()
}
