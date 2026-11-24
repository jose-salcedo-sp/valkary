use crate::{Post, Series, post_url};
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn SeriesNav(series: Series, post: Post) -> impl IntoView {
    let (prev, next) = series.neighbors(post.slug);
    if prev.is_none() && next.is_none() {
        return ().into_any();
    }

    view! {
        <nav aria-label="Series navigation" class="flex items-center justify-between gap-4">
            <div>
                {prev.map(|p| {
                    let href = post_url(&series, &p);
                    view! {
                        <A href=href attr:class="text-sm text-zinc-400 hover:text-white">
                            {format!("← {}", p.title)}
                        </A>
                    }
                })}
            </div>
            <div>
                {next.map(|n| {
                    let href = post_url(&series, &n);
                    view! {
                        <A href=href attr:class="text-sm text-zinc-400 hover:text-white">
                            {format!("{} →", n.title)}
                        </A>
                    }
                })}
            </div>
        </nav>
    }
    .into_any()
}
