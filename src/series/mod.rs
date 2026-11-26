pub mod harness;

use leptos::prelude::AnyView;

#[derive(Clone, Copy)]
pub struct TocSection {
    pub id: &'static str,
    pub title: &'static str,
}

#[derive(Clone, Copy)]
pub struct Post {
    pub slug: &'static str,
    pub title: &'static str,
    pub date: &'static str,
    pub summary: &'static str,
    pub sections: &'static [TocSection],
    pub body: fn() -> AnyView,
}

#[derive(Clone, Copy)]
pub struct SourceRepo {
    pub name: &'static str,
    pub description: &'static str,
    pub url: &'static str,
}

#[derive(Clone, Copy)]
pub struct Series {
    pub slug: &'static str,
    pub title: &'static str,
    pub summary: &'static str,
    pub source_repos: &'static [SourceRepo],
    pub posts: &'static [Post],
}

impl Series {
    #[must_use]
    pub fn neighbors(&self, slug: &str) -> (Option<Post>, Option<Post>) {
        let Some(idx) = self.posts.iter().position(|p| p.slug == slug) else {
            return (None, None);
        };
        let prev = idx.checked_sub(1).and_then(|i| self.posts.get(i).copied());
        let next = self.posts.get(idx + 1).copied();
        (prev, next)
    }
}

pub const SERIES: &[Series] = &[harness::SERIES];

#[must_use]
pub fn post_url(series: &Series, post: &Post) -> String {
    crate::site_url(&format!("/posts/{}/{}", series.slug, post.slug))
}

#[must_use]
pub fn all_posts() -> Vec<(Series, Post)> {
    SERIES
        .iter()
        .flat_map(|series| {
            series
                .posts
                .iter()
                .copied()
                .map(move |post| (*series, post))
        })
        .collect()
}

#[must_use]
pub fn find_post(series_slug: &str, post_slug: &str) -> Option<(Series, Post)> {
    let series = SERIES.iter().copied().find(|s| s.slug == series_slug)?;
    let post = series.posts.iter().copied().find(|p| p.slug == post_slug)?;
    Some((series, post))
}
