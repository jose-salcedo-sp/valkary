use crate::{Li, Link, Mermaid, P, Post, Quote, Section, TocSection, Ul};
use leptos::prelude::*;

const AGENT_LOOP_DIAGRAM: &str = r#"sequenceDiagram
    participant User
    participant Model
    participant Tools

    User->>Model: Send prompt
    Model->>Model: Parse prompt & figure out requirements
    loop Until question is answered
        Model->>Tools: read_file(path)
        Tools-->>Model: File contents
        Model->>Model: Was the question answered?
    end
    Model-->>User: Respond to question"#;

const ARCHITECTURE_DIAGRAM: &str = r#"flowchart LR
    UI["GPUI chatbot"] --> Backend["Rust backend<br/>Minimal agent loop"]
    Backend -->|"OpenAI-compatible HTTP"| Server["llama-server"]
    Server --> Model["Bonsai 2 9B"]"#;

pub const ARCHITECTURE_POST: Post = Post {
    slug: "architecture",
    title: "Architecture",
    date: "2026-11-25",
    summary: "This post will cover the architecture of the harness and how it is designed to work.",
    sections: &[
        TocSection {
            id: "what-is-a-harness",
            title: "What is a Harness?",
        },
        TocSection {
            id: "architecture",
            title: "Architecture",
        },
    ],
    body: Architecture,
};

#[component]
fn Architecture() -> AnyView {
    view! {
        <div class="space-y-6 leading-relaxed text-zinc-300">
            <Section title="What is a Harness?" id="what-is-a-harness">
                <P>"While reading a lot about harnesses and coding agents I arrived at the same definition over and over again on many sites. `Agent = Model + Harness`. We know what the model is, everyone's used one even if it is just the ChatGPT chatbot to cheat on your schoolwork. So the only thing we're missing is what is a harness and what are the parts that make it up?"</P>

                <P>"We will define a harness with the following layers:"</P>
                <Ul>
                    <Li>"A UI interface"</Li>
                    <Li>"An agent loop"</Li>
                    <Li>"A memory system"</Li>
                    <Li>"A tool and action system"</Li>
                    <Li>"A safety and permissions engine"</Li>
                    <Li>"Extensibility and modularity (e.g. skills, rules, plugins, etc.)"</Li>
                </Ul>

                <P>
                    "When I first started reading about all this, the thing I was most curious about was the `agent loop`. To quote from the paper:"
                </P>

                <Quote cite="Harness Engineering: Anatomy, Architecture, and Evolution of Coding Agents -- A Source-Code Study of Eleven Systems">
                    "The agent loop, the central control flow that alternates between LLM inference and action execution,
is the architectural backbone of every SWE agent."
                </Quote>

                <P>"So this will be our very first task and layer. We will be coming back to this layer a lot so it is good we start getting familiar with it now. So let's start with the most basic agent loop we can think of. And I can think of no better than a chatbot with the ability to read files (tool call) to answer a user's prompt. The gist of it is that the agent will read the file, parse it, and answer the user's prompt based on the contents of the file."</P>

                <Mermaid src=AGENT_LOOP_DIAGRAM/>
            </Section>

            <Section title="Architecture" id="architecture">
                <P>"Now that we have a basic understanding of the agent loop, we can start to think about the architecture of the harness. We will implement the UI and minimal agent loop with the following \"architecture\" (it's not really an architecture yet, just a proof of concept):"</P>

                <Ul>
                    <Li>"GPUI chatbot app frontend"</Li>
                    <Li>"Rust backend with a minimal agent loop"</Li>
                    <Li>
                        <Link href="https://github.com/ggerganov/llama.cpp/tree/master/examples/server" title="llama-server" />
                        " serving an OpenAI-compatible API"
                    </Li>
                    <Li>
                        <Link href="https://prismml.com/news/bonsai-2-family" title="Bonsai 2 9B" />
                        " model running locally behind llama-server"
                    </Li>
                </Ul>

                <Mermaid src=ARCHITECTURE_DIAGRAM/>
                <P>"The boundaries are intentional. The GPUI layer owns presentation and user input, but it does not know how inference or tools work. The Rust backend owns conversation history and the agent loop. It converts UI commands into model requests, interprets each response, and decides whether to finish the turn or execute another action. `llama-server` is only an inference provider: it receives an OpenAI-shaped request and streams tokens or structured tool calls back. The model itself remains stateless; every fact it can use must be included in the request the backend builds."</P>

                <P>"That separation gives us a small system we can reason about one boundary at a time. We can replace the local model without changing the loop, replace GPUI without changing tool execution, and test the loop as a state machine without opening a window or making an HTTP request. It also makes responsibility explicit: the model may request an action, but the harness validates and performs it. This distinction becomes important as soon as tools can do more than read a file."</P>

                <P>"For this first version there is no separate database, memory service, plugin host, or remote backend. Conversation history lives in the running process, `read_file` is the only tool, and requests travel directly from the desktop app to the local server. Those omissions are deliberate. The goal is to expose the complete path from a prompt to one tool call before adding the layers that make a production harness more capable—and harder to understand."</P>

            </Section>
        </div>
    }
    .into_any()
}
