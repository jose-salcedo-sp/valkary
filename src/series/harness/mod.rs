pub mod posts;

use crate::{Series, SourceRepo};
pub use posts::{
    ARCHITECTURE_POST, FIRST_TOOL_CALL_POST, INTRO_POST, SKILLS_AND_CITATIONS_POST,
    WHY_AND_WHAT_POST,
};

pub const SERIES: Series = Series {
    slug: "building-and-understanding-a-harness",
    title: "Building and Understanding an AI Harness",
    summary: "This is my first post on this blog, I am an AI enthusiast and I want to try and shed some light into how you can understand what your AI harness (Cursor, DeepSeek, etc.) is doing.",
    source_repos: &[SourceRepo {
        name: "jose-salcedo-sp/harness",
        description: "The agent harness built throughout this series.",
        url: "https://github.com/jose-salcedo-sp/harness",
    }],
    posts: &[
        INTRO_POST,
        WHY_AND_WHAT_POST,
        ARCHITECTURE_POST,
        FIRST_TOOL_CALL_POST,
        SKILLS_AND_CITATIONS_POST,
    ],
};
