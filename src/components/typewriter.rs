use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypewriterPhase {
    Typing,
    Paused,
    Deleting,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhraseStyle {
    Plain,
    CodeTag,
    Accent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypewriterPhrase {
    pub text: &'static str,
    pub style: PhraseStyle,
}

impl TypewriterPhrase {
    #[must_use]
    pub const fn plain(text: &'static str) -> Self {
        Self {
            text,
            style: PhraseStyle::Plain,
        }
    }

    #[must_use]
    pub const fn code_tag(text: &'static str) -> Self {
        Self {
            text,
            style: PhraseStyle::CodeTag,
        }
    }

    #[must_use]
    pub const fn accent(text: &'static str) -> Self {
        Self {
            text,
            style: PhraseStyle::Accent,
        }
    }
}

pub const HERO_PHRASES: &[TypewriterPhrase] = &[
    TypewriterPhrase::plain("Welcome to my portfolio!"),
    TypewriterPhrase::plain("I'm Pepe Salcedo"),
    TypewriterPhrase::code_tag("Agentic_Systems"),
    TypewriterPhrase::code_tag("AI_&_Provenance"),
];

pub const ABOUT_PHRASES: &[TypewriterPhrase] = &[
    TypewriterPhrase::accent("architecting"),
    TypewriterPhrase::accent("engineering"),
    TypewriterPhrase::accent("building"),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypewriterState {
    pub phrase_index: usize,
    pub char_count: usize,
    pub phase: TypewriterPhase,
}

impl TypewriterState {
    #[must_use]
    pub fn initial(phrases: &[TypewriterPhrase]) -> Self {
        let char_count = phrases.first().map_or(0, |p| p.text.chars().count());
        Self {
            phrase_index: 0,
            char_count,
            phase: TypewriterPhase::Paused,
        }
    }

    #[must_use]
    pub fn step(self, phrases: &[TypewriterPhrase]) -> Self {
        if phrases.is_empty() {
            return self;
        }
        let target_len = phrases
            .get(self.phrase_index)
            .map_or(0, |p| p.text.chars().count());

        match self.phase {
            TypewriterPhase::Typing => {
                if self.char_count < target_len {
                    Self {
                        phrase_index: self.phrase_index,
                        char_count: self.char_count + 1,
                        phase: TypewriterPhase::Typing,
                    }
                } else {
                    Self {
                        phrase_index: self.phrase_index,
                        char_count: target_len,
                        phase: TypewriterPhase::Paused,
                    }
                }
            }
            TypewriterPhase::Paused => Self {
                phrase_index: self.phrase_index,
                char_count: self.char_count.saturating_sub(1),
                phase: TypewriterPhase::Deleting,
            },
            TypewriterPhase::Deleting => {
                if self.char_count > 0 {
                    Self {
                        phrase_index: self.phrase_index,
                        char_count: self.char_count - 1,
                        phase: TypewriterPhase::Deleting,
                    }
                } else {
                    let next_index = (self.phrase_index + 1) % phrases.len();
                    Self {
                        phrase_index: next_index,
                        char_count: 0,
                        phase: TypewriterPhase::Typing,
                    }
                }
            }
        }
    }

    #[must_use]
    pub const fn delay_ms(self, type_ms: u32, delete_ms: u32, pause_ms: u32) -> u32 {
        match self.phase {
            TypewriterPhase::Typing => type_ms,
            TypewriterPhase::Paused => pause_ms,
            TypewriterPhase::Deleting => delete_ms,
        }
    }

    #[must_use]
    pub fn current_phrase(self, phrases: &[TypewriterPhrase]) -> Option<&TypewriterPhrase> {
        phrases.get(self.phrase_index)
    }

    #[must_use]
    pub fn visible_text(self, phrases: &[TypewriterPhrase]) -> &'static str {
        let Some(phrase) = phrases.get(self.phrase_index) else {
            return "";
        };
        match phrase.text.char_indices().nth(self.char_count) {
            Some((byte_idx, _)) => &phrase.text[..byte_idx],
            None => phrase.text,
        }
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen(inline_js = "
export function schedule_typewriter_tick(delay_ms, on_tick) {
    if (typeof window === 'undefined') return;
    if (window.matchMedia && window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
        return;
    }
    window.setTimeout(() => {
        on_tick();
    }, delay_ms);
}
")]
extern "C" {
    fn schedule_typewriter_tick(delay_ms: u32, on_tick: &wasm_bindgen::closure::Closure<dyn Fn()>);
}

const BASE_CLASS: &str = "inline-flex items-baseline";

#[component]
pub fn Typewriter(
    phrases: &'static [TypewriterPhrase],
    #[prop(default = 80)] type_ms: u32,
    #[prop(default = 45)] delete_ms: u32,
    #[prop(default = 2000)] pause_ms: u32,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let (state, set_state) = signal(TypewriterState::initial(phrases));

    #[cfg(feature = "hydrate")]
    {
        Effect::new(move |_| {
            let current = state.get();
            let delay = current.delay_ms(type_ms, delete_ms, pause_ms);
            let cb = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                set_state.update(|s| *s = s.step(phrases));
            }) as Box<dyn Fn()>);
            schedule_typewriter_tick(delay, &cb);
            cb.forget();
        });
    }
    #[cfg(not(feature = "hydrate"))]
    {
        let _ = (type_ms, delete_ms, pause_ms, &set_state);
    }

    let merged_class = match class {
        Some(extra) if !extra.is_empty() => format!("{BASE_CLASS} {extra}"),
        _ => BASE_CLASS.to_string(),
    };

    view! {
        <span class=merged_class>
            {move || {
                let s = state.get();
                let text = s.visible_text(phrases);
                let style = s
                    .current_phrase(phrases)
                    .map_or(PhraseStyle::Plain, |p| p.style);

                match style {
                    PhraseStyle::Plain => view! { <span>{text}</span> }.into_any(),
                    PhraseStyle::CodeTag => view! {
                        <span>
                            <span class="text-zinc-400">"<"</span>
                            <span class="font-mono text-blue-300">{text}</span>
                            <span class="text-zinc-400">"/>"</span>
                        </span>
                    }
                        .into_any(),
                    PhraseStyle::Accent => view! { <span class="text-blue-300">{text}</span> }.into_any(),
                }
            }}
            <span
                aria-hidden="true"
                class="ml-0.5 inline-block font-light text-[#f43f5e] animate-pulse motion-reduce:animate-none"
            >
                "|"
            </span>
        </span>
    }
}
