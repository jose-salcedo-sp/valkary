use crate::{CodeExcerpt, Mermaid, P, Post, Section, TocSection, site_url};
use leptos::prelude::*;

const FIRST_TOOL_CALL_DIAGRAM: &str = r#"sequenceDiagram
    participant User
    participant Session
    participant AgentLoop
    participant Model as llama-server
    participant Tools as FsTools

    User->>Session: Submit message
    Session->>AgentLoop: Input Submit
    Note over AgentLoop: Idle to Inferring, round 1
    AgentLoop-->>Session: Emit UserMessage, Infer
    Session->>Model: POST chat/completions
    Model-->>Session: Stream ends with a read_file call
    Session->>AgentLoop: ModelChunk, ModelFinished
    Note over AgentLoop: Inferring to RunningTools
    AgentLoop-->>Session: Emit ToolCallStarted, RunTool
    Session->>Tools: read_file agent.rs
    Tools-->>Session: 871 lines of text
    Session->>AgentLoop: ToolFinished
    Note over AgentLoop: RunningTools to Inferring, round 2
    AgentLoop-->>Session: Emit ToolCallFinished, Infer
    Session->>Model: POST with the file in the history
    Model-->>Session: Streamed answer, no tool calls
    Session->>AgentLoop: ModelChunk, ModelFinished
    Note over AgentLoop: Inferring to Idle
    Session-->>User: TurnFinished"#;

const SUBMIT_EXCERPT: &str = r#"Input::Submit { text } => {
    history.push(Message::User(text));
    phase = Phase::Inferring { round: 1 };
    vec![Emit(UserMessage { ... }), Infer(request(...))]
}"#;

const REQUEST_EXCERPT: &str = r#"ChatRequest {
    model,
    messages: history.clone(),
    tools: vec![read_file_schema()],
    stream: true,
    ...
}"#;

const TOOL_CALL_EXCERPT: &str = r#"if !response.tool_calls.is_empty() {
    phase = Phase::RunningTools { pending: ... };
    return vec![Emit(ToolCallStarted { ... }), RunTool { ... }];
}"#;

const RUN_TOOL_EXCERPT: &str = r#"let request = ToolRequest::parse(name, arguments)?;
let path = resolve_inside_workspace(request.path)?;
let output = fs::read_to_string(path)?;
..."#;

const TOOL_RESULT_EXCERPT: &str = r#"history.push(Message::Tool { call_id, content: output, ... });
pending.remove(&call_id);

if pending.is_empty() {
    phase = Phase::Inferring { round: round + 1 };
    effects.push(Infer(request(...)));
}"#;

const FINISH_EXCERPT: &str = r#"if response.tool_calls.is_empty() {
    phase = Phase::Idle;
    return vec![Emit(TurnFinished)];
}"#;

const STEP_CLASS: &str = "font-semibold text-zinc-100";

pub const FIRST_TOOL_CALL_POST: Post = Post {
    slug: "first-tool-call",
    title: "Our First Tool Call",
    date: "2026-11-25",
    summary: "Following a read_file request through the agent loop, tool runtime, and back to the model.",
    sections: &[TocSection {
        id: "first-tool-call",
        title: "Our First Tool Call",
    }],
    body: FirstToolCall,
};

#[component]
fn FirstToolCall() -> AnyView {
    view! {
        <Section title="Our First Tool Call" id="first-tool-call">
            <P>"With the proof of concept wired up, I ran it against Bonsai 2 9B served by `llama-server` with a 32k context, and asked it to read the file that holds the agent loop itself. This is what came back:"</P>

            <picture>
                <source srcset=site_url("/post-imgs/harness/first-tool-call.webp") type="image/webp"/>
                <img
                    src=site_url("/post-imgs/harness/first-tool-call.png")
                    alt="The Harness chat window: the prompt \"read crates/harness-core/src/agent.rs\", one read_file call reporting 871 lines and 27 KB, then the model's summary of the file, with the context meter at 12k of 32k"
                    width="1728"
                    height="1085"
                    loading="lazy"
                    decoding="async"
                    class="my-5 h-auto w-full rounded-lg border border-zinc-800 sm:my-6"
                />
            </picture>

            <P>"One `read_file` call (871 lines, 27 KB), then an answer, and the context meter jumped to 12k of 32k. A lot happens between pressing Enter and that answer, so let's follow the request through the code. Two pieces do all the work. `AgentLoop` in `crates/harness-core/src/agent.rs` is a pure state machine: it takes an `Input` and returns a list of `Effect`s, and never does any IO itself. `Session` is the driver that performs those effects (calling the model, running tools) and feeds the results back in as new inputs."</P>

            <Mermaid src=FIRST_TOOL_CALL_DIAGRAM/>

            <P>
                <span class=STEP_CLASS>"1. The user message. "</span>
                "Pressing Enter parses the text box into a `UserText` (trimmed, never empty) and sends `Command::Submit` down a channel to the `Session`, which runs on a background executor so the window never blocks. The session turns the command into `Input::Submit` and hands it to the loop. The loop is `Idle`, so it appends a `Message::User` to the conversation and returns two effects: emit `UserMessage` so the window draws my bubble, and `Infer` with a fresh request. The phase moves to `Inferring` on round 1."
            </P>
            <CodeExcerpt code=SUBMIT_EXCERPT/>

            <P>
                <span class=STEP_CLASS>"2. The inference request. "</span>
                "A request is the model id, the system prompt, the entire conversation so far and the list of tools. Entire really means entire: the earlier \"Hello who are you?\" exchange goes along too. The tool list has a single entry, `read_file`, whose JSON schema takes one `path` string, and the system prompt tells the model when to use it: \"When the answer depends on a file, call the `read_file` tool with a path relative to the workspace, then answer from its contents.\" The OpenAI-compatible provider turns all of this into a `POST /v1/chat/completions` with `stream: true` and asks for token usage in the stream. It makes the HTTP call on its own OS thread and passes chunks back through a channel, so it needs no async runtime."
            </P>
            <CodeExcerpt code=REQUEST_EXCERPT/>

            <P>
                <span class=STEP_CLASS>"3. The model emits a read_file call. "</span>
                "`llama-server` streams the response back as server-sent events. Text is forwarded the moment it arrives, but a tool call comes in fragments (an id, a name, then the JSON arguments a few characters at a time), so the decoder stitches them together and only emits the finished `ToolCall` once the stream ends. The arguments stay raw JSON, `{\"path\":\"crates/harness-core/src/agent.rs\"}`, so they can be echoed back to the model later. In my run this first response was only a tool call: had the model written any text first, it would sit in a bubble above the `read_file` badge. The last event also carries token usage, which updates the context meter."
            </P>

            <P>
                <span class=STEP_CLASS>"4. The loop changes state. "</span>
                "When the stream closes, the session feeds `Input::ModelFinished`. The loop records the response as a `Message::Assistant` with empty text and one tool call, sees the call list is not empty, and returns `ToolCallStarted` (the badge with its spinner) plus a `RunTool` effect. The phase becomes `RunningTools`, holding that call's id in a `pending` list. Just like in `Inferring`, the loop answers new messages and model switches with a `Busy` error, which is why the window ignores Enter and greys out the model picker mid-turn."
            </P>
            <CodeExcerpt code=TOOL_CALL_EXCERPT/>

            <P>
                <span class=STEP_CLASS>"5. The tool runs. "</span>
                "Before anything touches the disk, `ToolRequest::parse` checks that the name is a tool we know and that the arguments are a JSON object with a non-empty `path`. Then `FsTools` joins the path onto the workspace root (`HARNESS_WORKSPACE`, or the directory the app was started from), resolves it, refuses anything that lands outside the root or is over 256 KB, and reads the file as UTF-8. `agent.rs` is 27,879 bytes, well under the limit. A failed check would not crash the turn either: the error becomes text like `error: path is empty` and goes to the model as the tool result."
            </P>
            <CodeExcerpt code=RUN_TOOL_EXCERPT/>

            <P>
                <span class=STEP_CLASS>"6. The result goes back to the model. "</span>
                "The file comes back as `Input::ToolFinished`. The loop removes the id from `pending`, appends a `Message::Tool` whose content is the whole file, and emits `ToolCallFinished`. The window counts the lines and bytes of that output for the badge, which is where \"871 lines · 27 KB\" comes from. With nothing left pending and round 2 under the limit, the loop returns another `Infer` effect and goes back to `Inferring`. That second request carries everything again: the system prompt, the hello exchange, my message, the assistant message with the tool call, and the tool message with all 871 lines. The model keeps no state between requests, so the harness resends the whole conversation every time."
            </P>
            <CodeExcerpt code=TOOL_RESULT_EXCERPT/>

            <P>
                <span class=STEP_CLASS>"7. The final answer. "</span>
                "Round 2 streams text, and every `TextDelta` goes straight to the window, which is why the answer types itself out as markdown. When the stream ends the loop records the reply, sees there are no tool calls this time, emits `TurnFinished`, and goes back to `Idle`."
            </P>
            <CodeExcerpt code=FINISH_EXCERPT/>

            <P>
                <span class=STEP_CLASS>"Were there any other tool calls? "</span>
                "No. The turn made exactly one tool call and two model requests: one that asked for the file and one that answered. The loop never decides to call a tool on its own. It ends the turn the first time a response comes back without tool calls, and the second response was plain text. Every `ToolCallStarted` adds a badge to the window, so a second `read_file` would have shown up as a second badge. There was also nothing to retry: the read succeeded, `read_file` returns the whole file in one go (there is no offset or line range to page through), and it is the only tool the model has. The model could have kept going if it wanted to. A turn allows up to 8 model responses (`DEFAULT_MAX_ROUNDS`), after which it fails with \"Stopped after 8 model calls without a final answer.\" And if one response had asked for several files at once, the loop would wait for every pending call before asking the model again, with the session running them one at a time."
            </P>

            <P>
                <span class=STEP_CLASS>"Why did the context jump? "</span>
                "The meter shows the server's own count: the `prompt_tokens` plus `completion_tokens` that `llama-server` reports for the latest response. The harness only falls back to guessing four characters per token when a server reports nothing. Before the read, the prompt was the system prompt, the tool schema, the hello exchange and a one-line message. The second prompt also holds the whole file: 27,879 characters, roughly 7,000 tokens even by that four-character guess, and source code with all its symbols and indentation usually tokenizes denser than prose. Add the tokens of the answer and you land at 12k. Usage arrives in the last event of each stream, so the meter jumps when the answer finishes, not when the file is read. It does not come back down either. The file is now part of the conversation and every later request resends it, so even a \"thanks\" starts at over 12k tokens. Nothing trims or summarizes old messages yet. That is the memory layer's job, and we will get there."
            </P>
        </Section>
    }
    .into_any()
}
