use leptos::prelude::*;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen(inline_js = "
let observerInitialized = false;

export function fit_inline_links() {
    if (typeof document === 'undefined') return;
    const links = document.querySelectorAll('a[data-inline-link]');
    for (const a of links) {
        const container = a.closest('p, li, blockquote, dd, td, th, div, section, article, main') || a.parentElement;
        if (!container) continue;
        const probe = a.previousElementSibling && a.previousElementSibling.previousElementSibling && a.previousElementSibling.previousElementSibling.hasAttribute('data-link-probe')
            ? a.previousElementSibling.previousElementSibling
            : (a.previousElementSibling && a.previousElementSibling.hasAttribute('data-link-probe') ? a.previousElementSibling : null);

        a.style.display = 'none';
        const cRect = container.getBoundingClientRect();
        const cs = window.getComputedStyle(container);
        const padRight = parseFloat(cs.paddingRight) || 0;
        const borderRight = parseFloat(cs.borderRightWidth) || 0;
        const padLeft = parseFloat(cs.paddingLeft) || 0;
        const borderLeft = parseFloat(cs.borderLeftWidth) || 0;

        const rightEdge = cRect.right - padRight - borderRight;
        const leftEdge = cRect.left + padLeft + borderLeft;
        const fullWidth = Math.max(0, Math.floor(rightEdge - leftEdge - 10));
        const probeLeft = probe ? probe.getBoundingClientRect().left : leftEdge;
        const remaining = Math.floor(rightEdge - probeLeft - 10);

        a.style.display = '';
        a.style.maxWidth = (remaining >= 64 ? remaining : fullWidth) + 'px';
    }

    if (!observerInitialized && typeof window !== 'undefined') {
        observerInitialized = true;
        window.addEventListener('resize', () => fit_inline_links(), { passive: true });
        if (typeof ResizeObserver !== 'undefined' && document.body) {
            const ro = new ResizeObserver(() => fit_inline_links());
            ro.observe(document.body);
        }
        if (document.fonts && document.fonts.ready) {
            document.fonts.ready.then(() => fit_inline_links()).catch(() => {});
        }
    }
}
")]
extern "C" {
    fn fit_inline_links();
}

#[must_use]
pub fn default_link_meta(href: &str) -> (String, String) {
    let trimmed = href
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches('/');
    (
        trimmed.to_string(),
        crate::with_base("/favicons/default.svg"),
    )
}

#[component]
pub fn Link(
    href: &'static str,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let cached = super::link_metadata::find(href);
    let (fallback_title, fallback_icon) = default_link_meta(href);
    let label = title.unwrap_or_else(|| {
        cached
            .map(|metadata| metadata.title.to_owned())
            .unwrap_or(fallback_title)
    });
    let icon = cached
        .map(|metadata| crate::with_base(metadata.icon_path))
        .unwrap_or(fallback_icon);

    #[cfg(feature = "hydrate")]
    {
        Effect::new(move |_| {
            fit_inline_links();
        });
    }

    const BASE_CLASS: &str = "mx-1 inline-flex min-w-0 max-w-full items-center gap-1.5 overflow-hidden rounded-md border border-zinc-700/80 bg-zinc-900 px-2 py-0.5 align-baseline text-xs sm:text-sm font-medium text-zinc-100 transition hover:border-red-400/60 hover:bg-zinc-800";
    let merged_class = match class {
        Some(extra) if !extra.is_empty() => format!("{BASE_CLASS} {extra}"),
        _ => BASE_CLASS.to_string(),
    };

    view! {
        <span data-link-probe="" aria-hidden="true" class="inline-block h-0 w-0 overflow-hidden align-baseline"></span>
        <wbr/>
        <a data-inline-link="" href=href target="_blank" rel="noopener noreferrer" class=merged_class>
            <img
                src=icon
                alt=""
                loading="lazy"
                class="inline-block h-3.5 w-3.5 shrink-0 rounded-xs object-contain"
            />
            <span class="min-w-0 truncate">{label}</span>
        </a>
    }
}
