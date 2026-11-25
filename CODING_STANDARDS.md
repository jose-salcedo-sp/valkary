# Coding Standards

Concise, high-signal conventions for the `blog` repository (Leptos 0.8 + Axum + Tailwind CSS v4, statically exported for GitHub Pages at `/valkary`).

---

## 1. Architecture & Routing

### Routes
- `/` — **Portfolio Root**: Landing page featuring **Hero**, **About**, **Current Work**, and **Blog** sections separated by titles, tracked uppercase kickers, and whitespace rather than horizontal divider lines.
- `/blog` — **Blog Index**: Full index of series and posts.
- `/posts/:series/:slug` — **Post Reader**: Series navigation (`SeriesNav`), post header, table of contents (`TableOfContents`), rendered post body, and footer navigation.

### Base Path & Static Export Compatibility
- **GitHub Pages Base Path**: Production builds set `SITE_BASE=/valkary`, exposed in Rust as `pub const BASE_PATH: &str`.
- **Internal Links & Assets**: Every internal route href, `<A href=...>`, stylesheet, script, and static asset path **must** be prefixed with `BASE_PATH` via the base-path helper (`site_url` / `with_base`, e.g., `with_base("/blog")`, `with_base("/pkg/blog.css")`, `post_url(series, post)`). Never hardcode bare root-relative paths like `"/blog"` or `"/assets/..."` in markup.
- **Router Configuration**: `<Router base=BASE_PATH>` and `<HydrationScripts options root=BASE_PATH/>` must stay aligned with `BASE_PATH`.
- **Static Export (`src/main.rs --export`)**:
  - Invoking the binary with `--export` pre-renders static HTML into `site_root` (`target/site`) without starting a long-lived HTTP server.
  - Every route registered in `<Routes>` (`/` -> `index.html`, `/blog` -> `blog/index.html`, `/posts/:series/:slug` -> `posts/<series>/<slug>/index.html`, and `404.html`) must be rendered in `export_static_site` using `BASE_PATH`-prefixed request URIs.
  - Always emit `.nojekyll` at the root of `target/site` so GitHub Pages serves `/pkg` and underscore-prefixed WASM/JS assets cleanly.

---

## 2. Rust & Leptos Conventions

- **Idiomatic Rust (Edition 2024)**:
  - Keep domain types (`Series`, `Post`, `TocSection`, `SkillTab`) `Copy` / `'static` slices where possible so static post registries remain zero-allocation at startup.
  - Mark pure helpers with `#[must_use]`.
  - Prefer iterator chains (`filter_map`, `flat_map`, `collect_view()`) over manual mutation loops.
- **Component Organization (`src/components/`)**:
  - One focused module per component in `src/components/`, re-exported cleanly through `src/components/mod.rs` and `src/lib.rs`.
  - Accept optional styling overrides via `#[prop(optional, into)] class: Option<String>` merged onto a `BASE_CLASS` constant.
- **SSR + Hydrate Feature Parity**:
  - Default Cargo feature is `ssr` so `cargo test` and `cargo clippy` work out of the box; WASM builds use `--no-default-features --features hydrate`.
  - Gate browser-only APIs (`wasm_bindgen`, `window`, `sessionStorage`, `navigator.clipboard`) behind `#[cfg(feature = "hydrate")]` and provide clean no-op or fallback paths under `#[cfg(not(feature = "hydrate"))]`.
  - Ensure initial SSR output matches initial hydration state to prevent hydration mismatches.
- **No Unhandled `unwrap` in Runtime Code**:
  - Never call `.unwrap()` or `.expect()` in request handlers, components, routing, or WASM hydration paths.
  - Use `Option` / `Result` combinators (`unwrap_or`, `unwrap_or_default`, `unwrap_or_else`, `and_then`, `?`, or pattern matching with `let Some(...) = ... else`).
- **Verification & Test Coverage**:
  - Every change must pass `cargo test` and `cargo clippy` with zero warnings.
  - Maintain unit tests in `src/lib.rs` covering route/URL helpers (`BASE_PATH` / `with_base` / `site_url`, `post_url`), post lookup (`find_post`, `all_posts`), series neighbor resolution (`Series::neighbors`), and markdown/component rendering.

---

## 3. Frontend & Design Taste Standards

### Palette & Surface Hierarchy
- **Dark Zinc Palette**:
  - Page canvas: `bg-zinc-950` with `text-zinc-100 antialiased`.
  - Elevated surfaces / cards / code blocks: `bg-zinc-900/50` or `bg-zinc-900` with `border-zinc-800` (hover: `border-zinc-700`).
  - Body prose: `text-zinc-300`; secondary metadata / timestamps / muted labels: `text-zinc-400` or `text-zinc-500`.
  - Accent: subtle `red-400` / `red-300` for active indicators, inline code highlights, blockquotes, and interactive focus/hover accents.
  - Maintain WCAG contrast ratios (ink >= 7:1, muted ink >= 4.5:1 against background surfaces).

### Typography & Layout Geometry
- **Hierarchy**: Clear, single-scale progression (`h1` > `h2` > `h3` > body `P` > metadata/captions) paired with uppercase tracked kickers (`text-xs font-medium tracking-[8px] text-zinc-400 uppercase`) instead of horizontal rule dividers on `/`. Never set a heading smaller than the body copy beneath it.
- **Equal Content Edges**: All top-level sections (`Hero`, `About`, `Current Work`, `Blog`, blog index, and post articles) share a single centered container (`mx-auto max-w-4xl px-4 sm:px-6`) so left and right content edges align across the entire viewport.
- **Containment**: No horizontal overflow (`overflow-x`) at desktop (`1440px`), tablet (`768px`–`1024px`), or mobile (`390px` / `320px`). Wide code blocks and tables scroll internally (`overflow-x-auto`).

### Motion & Interaction
- **Purposeful Motion**: Keep transitions subtle and fast (`150ms`–`200ms` color/border/opacity transitions). Avoid gratuitous scroll-jacking or unrequested looping animations.
- **Reduced Motion**: Respect `prefers-reduced-motion: reduce` (`motion-reduce:transition-none`, `motion-reduce:animate-none`, and instant scroll behavior when reduced motion is active).

### Visual Discipline & Clean Copy
- **No Caption Clutter**: Never render meta-captions under images or UI blocks explaining how an asset was made, placeholder notes, or inline license boilerplate.
- **Anti-Slop Copywriting**:
  - Write direct, concrete, human prose.
  - Ban AI slop words (`delve`, `tapestry`, `testament to`, `realm`, `embark`, `beacon`, `multifaceted`, `paradigm`, `myriad`, `plethora`, `meticulous`, `intricate`, `elevate`, `unlock`, `unleash`, `empower`, `supercharge`, `seamless`, `effortless`, `cutting-edge`, `revolutionary`, `transformative`, `holistic`, `game-changer`, `synergy`, `leverage`, `reimagine`, `robust`, `unparalleled`). Note: literal domain usage of `harness` (e.g., "AI harness") is permitted when followed by concrete technical context rather than stock phrases like "harness the power of".
  - Ban stock filler phrases ("in today's fast-paced...", "let's dive in", "it's not just X, it's Y", "at your fingertips"), staged reveals ("The result?", "Here's the thing"), and exclamation marks in site copy.
  - Limit em dashes (`—`) to at most 2 per page of prose.

### Automated Design-Taste Verification (`~/.agents/skills/design-taste-frontend`)
When auditing rendered HTML or running the local server (with Node v22+ / v24+):
- **Copy Lint**: `node ~/.agents/skills/design-taste-frontend/scripts/copy-lint.mjs <html-or-md-file> --kind tech`
- **Visual & Layout Check**:
  - `node ~/.agents/skills/design-taste-frontend/scripts/consistency.mjs <url-or-file> --widths 1440,390 --profile .taste-skill/chrome-consistency --page tech`
  - `node ~/.agents/skills/design-taste-frontend/assets/layout/check.mjs --url <url-or-file> --sizes 1440x900,390x844 --profile .taste-skill/chrome-layout`
  - `node ~/.agents/skills/design-taste-frontend/scripts/shoot.mjs --url <url-or-file> --w 1440 --out .taste-skill/shots/1440.png --profile .taste-skill/chrome-shoot`
