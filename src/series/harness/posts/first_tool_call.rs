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
    },
    TocSection {
        id: "agent-loop",
        title: "The Agent Loop",
    },
    TocSection {
        id: "tool-runtime",
        title: "The Tool Runtime",
    }],
    body: FirstToolCall,
};

#[component]
fn FirstToolCall() -> AnyView {
    view! {
        <P>"Okay so this is the part where we get real technical really fast. This is no longer conceptual and we actually get into the code. I will be modeling the code (mainly the backend code) as a finite state machine with transitions between states and effects that are executed when the state changes."</P>

        <P>"While I won't get into every line of code (I don't want to bore you to death), I will try to explain and highlight the most important parts. Let's start with the most basic tool call: reading a file."</P>

        <Section title="Our First Tool Call" id="first-tool-call">
            <P>
                "With the proof of concept wired up, I ran it against Bonsai 2 9B served by "
                <code>"llama-server"</code>
                " with a 32k context, and asked it to read the file that holds the agent loop itself. This is what came back:"
            </P>

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

            <P>
                "One "
                <code>"read_file"</code>
                " call (871 lines, 27 KB), then an answer, and the context meter jumped to 12k of 32k. A lot happens between pressing Enter and that answer, so let's follow the request through the code. Two pieces do all the work. "
                <code>"AgentLoop"</code>
                " in "
                <code>"crates/harness-core/src/agent.rs"</code>
                " is a pure state machine: it takes an "
                <code>"Input"</code>
                " and returns a list of "
                <code>"Effect"</code>
                "s, and never does any IO itself. "
                <code>"Session"</code>
                " is the driver that performs those effects (calling the model, running tools) and feeds the results back in as new inputs."
            </P>

            <Mermaid src=FIRST_TOOL_CALL_DIAGRAM/>


        </Section>

        <Section title="The Agent Loop" id="agent-loop">
            <P>
                <span class=STEP_CLASS>"1. The user message. "</span>
                "Pressing Enter parses the text box into a "
                <code>"UserText"</code>
                " (trimmed, never empty) and sends "
                <code>"Command::Submit"</code>
                " down a channel to the "
                <code>"Session"</code>
                ", which runs on a background executor so the window never blocks. The session turns the command into "
                <code>"Input::Submit"</code>
                " and hands it to the loop. The loop is "
                <code>"Idle"</code>
                ", so it appends a "
                <code>"Message::User"</code>
                " to the conversation and returns two effects: emit "
                <code>"UserMessage"</code>
                " so the window draws my bubble, and "
                <code>"Infer"</code>
                " with a fresh request. The phase moves to "
                <code>"Inferring"</code>
                " on round 1."
            </P>
            <CodeExcerpt code=SUBMIT_EXCERPT/>

            <P>
                <span class=STEP_CLASS>"2. The inference request. "</span>
                "A request is the model id, the system prompt, the entire conversation so far and the list of tools. Entire really means entire: the earlier \"Hello who are you?\" exchange goes along too. The tool list has a single entry, "
                <code>"read_file"</code>
                ", whose JSON schema takes one "
                <code>"path"</code>
                " string, and the system prompt tells the model when to use it: \"When the answer depends on a file, call the read_file tool with a path relative to the workspace, then answer from its contents.\" The OpenAI-compatible provider turns all of this into a "
                <code>"POST /v1/chat/completions"</code>
                " with "
                <code>"stream: true"</code>
                " and asks for token usage in the stream. It makes the HTTP call on its own OS thread and passes chunks back through a channel, so it needs no async runtime."
            </P>
            <CodeExcerpt code=REQUEST_EXCERPT/>

            <P>
                <span class=STEP_CLASS>"3. The model emits a read_file call. "</span>
                <code>"llama-server"</code>
                " streams the response back as server-sent events. Text is forwarded the moment it arrives, but a tool call comes in fragments (an id, a name, then the JSON arguments a few characters at a time), so the decoder stitches them together and only emits the finished "
                <code>"ToolCall"</code>
                " once the stream ends. The arguments stay raw JSON, "
                <code>"{\"path\":\"crates/harness-core/src/agent.rs\"}"</code>
                ", so they can be echoed back to the model later. In my run this first response was only a tool call: had the model written any text first, it would sit in a bubble above the "
                <code>"read_file"</code>
                " badge. The last event also carries token usage, which updates the context meter."
            </P>

            <P>
                <span class=STEP_CLASS>"4. The loop changes state. "</span>
                "When the stream closes, the session feeds "
                <code>"Input::ModelFinished"</code>
                ". The loop records the response as a "
                <code>"Message::Assistant"</code>
                " with empty text and one tool call, sees the call list is not empty, and returns "
                <code>"ToolCallStarted"</code>
                " (the badge with its spinner) plus a "
                <code>"RunTool"</code>
                " effect. The phase becomes "
                <code>"RunningTools"</code>
                ", holding that call's id in a "
                <code>"pending"</code>
                " list. Just like in "
                <code>"Inferring"</code>
                ", the loop answers new messages and model switches with a "
                <code>"Busy"</code>
                " error, which is why the window ignores Enter and greys out the model picker mid-turn."
            </P>
            <CodeExcerpt code=TOOL_CALL_EXCERPT/>

            <P>
                <span class=STEP_CLASS>"5. The tool runs. "</span>
                "Before anything touches the disk, "
                <code>"ToolRequest::parse"</code>
                " checks that the name is a tool we know and that the arguments are a JSON object with a non-empty "
                <code>"path"</code>
                ". Then "
                <code>"FsTools"</code>
                " joins the path onto the workspace root ("
                <code>"HARNESS_WORKSPACE"</code>
                ", or the directory the app was started from), resolves it, refuses anything that lands outside the root or is over 256 KB, and reads the file as UTF-8. "
                <code>"agent.rs"</code>
                " is 27,879 bytes, well under the limit. A failed check would not crash the turn either: the error becomes text like "
                <code>"error: path is empty"</code>
                " and goes to the model as the tool result."
            </P>
            <CodeExcerpt code=RUN_TOOL_EXCERPT/>

            <P>
                <span class=STEP_CLASS>"6. The result goes back to the model. "</span>
                "The file comes back as "
                <code>"Input::ToolFinished"</code>
                ". The loop removes the id from "
                <code>"pending"</code>
                ", appends a "
                <code>"Message::Tool"</code>
                " whose content is the whole file, and emits "
                <code>"ToolCallFinished"</code>
                ". The window counts the lines and bytes of that output for the badge, which is where \"871 lines · 27 KB\" comes from. With nothing left pending and round 2 under the limit, the loop returns another "
                <code>"Infer"</code>
                " effect and goes back to "
                <code>"Inferring"</code>
                ". That second request carries everything again: the system prompt, the hello exchange, my message, the assistant message with the tool call, and the tool message with all 871 lines. The model keeps no state between requests, so the harness resends the whole conversation every time."
            </P>
            <CodeExcerpt code=TOOL_RESULT_EXCERPT/>

            <P>
                <span class=STEP_CLASS>"7. The final answer. "</span>
                "Round 2 streams text, and every "
                <code>"TextDelta"</code>
                " goes straight to the window, which is why the answer types itself out as markdown. When the stream ends the loop records the reply, sees there are no tool calls this time, emits "
                <code>"TurnFinished"</code>
                ", and goes back to "
                <code>"Idle"</code>
                "."
            </P>
            <CodeExcerpt code=FINISH_EXCERPT/>

            <P>
                <span class=STEP_CLASS>"Were there any other tool calls? "</span>
                "No. The turn made exactly one tool call and two model requests: one that asked for the file and one that answered. The loop never decides to call a tool on its own. It ends the turn the first time a response comes back without tool calls, and the second response was plain text. Every "
                <code>"ToolCallStarted"</code>
                " adds a badge to the window, so a second "
                <code>"read_file"</code>
                " would have shown up as a second badge. There was also nothing to retry: the read succeeded, "
                <code>"read_file"</code>
                " returns the whole file in one go (there is no offset or line range to page through), and it is the only tool the model has. The model could have kept going if it wanted to. A turn allows up to 8 model responses ("
                <code>"DEFAULT_MAX_ROUNDS"</code>
                "), after which it fails with \"Stopped after 8 model calls without a final answer.\" And if one response had asked for several files at once, the loop would wait for every pending call before asking the model again, with the session running them one at a time."
            </P>
        </Section>

        <Section title="The Tool Runtime" id="tool-runtime">
            <P>"Now that was our first run through the agent loop. Not as simple as it sounds but the logic behind it is sound. In simple words; we prompted pure, unadulterated text to the model asking it to read a file. This was sent through our backend to the model which decided (after parsing our text) it needed to read said file and we provide it with a model for tool invocation and streams it back to our backend. Our backend then parses the tool call and does all the IO work to read the file and stream it back to the model. Finally the model based on our prompt and the contents of the file read; answers our question/request/prompt."</P>

            <P>"At the end of the day, we have two services talking to each other. One handles the text generation and reasoning (model) and the other handles tool invocation and definition when a the model asks for available tools. We expose those tools the same way you would prompt a model. We simply attach them to the context and hope the model chooses the correct one for the task at hand."</P>
        </Section>
    }
    .into_any()
}
