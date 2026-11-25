---
name: rust-state-machines
description: >-
  Rust finite-state machine modeling with typestate, marker states, consuming
  transitions, eros error sets, retries, audit trails, and compile-fail tests.
  Use when adding or changing a workflow, lifecycle, pipeline, state machine,
  state transition, or state-dependent Rust API.
---

# Rust finite-state machines

Part of [`rust-patterns`](../rust-patterns/SKILL.md), which sets the scope and the done check.

Model the transition graph before writing the types. Name every state, valid edge, terminal state, branch, effect, and persisted event.

Use typestate when the machine is bounded and a state's valid operations differ at compile time. Use a runtime enum when code must deserialize one of many states, hold mixed states together, or follow a data-driven graph. At a boundary, parse the runtime enum and match exhaustively before entering a typed flow.

## Shape

Use one machine shell with one state parameter. Put lifecycle-wide data in the shell. Put state-specific data in the state value, so later states do not require `Option` fields.

```rust
struct Run<S> {
    id: RunId,
    history: Vec<Transition>,
    state: S,
}

struct Draft {
    query: String,
}

struct Retrieved {
    query: String,
    chunks: Vec<Chunk>,
}

struct NoEvidence {
    query: String,
}
```

Prefer a state value as the generic field. Use `PhantomData<S>` only when the state carries no data. Keep machine fields private. Expose constructors only for valid initial states.

Write transitions in an `impl` for their source state. Consume `self` and return the exact destination state. The missing method is the proof that another transition is invalid.

```rust
enum RetrieveOutcome {
    Found(Run<Retrieved>),
    Empty(Run<NoEvidence>),
}

impl Run<Draft> {
    #[context("retrieve chunks")]
    fn retrieve(
        self,
        retriever: &impl Retriever,
    ) -> eros::Result<RetrieveOutcome, (RetrieveError,)> {
        let chunks = retriever.retrieve(&self.state.query).union()?;

        if chunks.is_empty() {
            return Ok(RetrieveOutcome::Empty(
                self.transition("no_evidence", |Draft { query }| NoEvidence { query }),
            ));
        }

        Ok(RetrieveOutcome::Found(
            self.transition("retrieved", |Draft { query }| Retrieved {
                query,
                chunks,
            }),
        ))
    }
}
```

Keep the transition constructor private. Give it a closure that consumes the source state and constructs the destination. It moves shared data, installs the destination, and records one successful edge.

Do not expose a generic `set_state`, public state fields, or conversions that bypass the graph. Add a direct edge only when the domain allows that edge.

## Branches and errors

An expected domain decision is a branch in the graph. Return an enum whose variants contain the possible destination states. Match it exhaustively.

An error means the transition could not determine or commit its destination. Return an `eros` result. Use a typed set when a caller branches on an error, and `eros::Result<T>` when every caller only propagates or reports it. Add `#[context]` to each fallible transition.

List only operational failures in the error set. Do not represent an invalid transition as an error; make it impossible to call. Do not use an error for an ordinary destination such as rejected, cancelled, empty, or timed out when that destination belongs to the domain graph.

Use `.union()` for a concrete error, `.widen()` when composing transition sets, and `.recover()` only where the graph defines recovery. Read [`rust-errors`](../rust-errors/SKILL.md) whenever a transition is fallible.

## Effects, ownership, and retries

Separate the effect from the state move when a failed attempt leaves the source state valid:

1. Borrow the source state and perform the idempotent effect.
2. Retry the borrowed effect according to its policy.
3. Consume the source state only after the effect succeeds.
4. Return the destination state.

A consuming transition that returns only an error loses the source value. Use that shape only when failure ends the in-memory flow. If the caller must retry or recover the machine, preserve the source state explicitly or model recovery as another state.

Keep ports outside the machine and inject them into transitions. A transition coordinates domain state; an adapter performs database, filesystem, HTTP, or model I/O.

For durable machines, commit the new state and its transition record atomically. Append one immutable transition record after a successful edge. Give failed attempts their own attempt record when the audit contract requires them; do not report them as completed transitions.

## Composition

Start with concrete transition methods. Add a `Step` trait only when multiple transitions need the same composition mechanism.

A composable step names its input state, output state, and error set with associated types. Sequence two steps only when the first output is the second input. Prefer static dispatch unless runtime selection is a requirement.

Avoid one generic parameter per independent flag. If the type parameter list forms a Cartesian product, model cohesive states or use a runtime enum.

## Tests

Test every modeled edge:

- The source data moves into the correct destination.
- Each expected branch returns its declared destination.
- Each operational failure follows the declared retry or recovery semantics.
- The audit contains each successful edge once and in order.
- A retry does not duplicate a completed transition.

Add `compile_fail` doctests or `trybuild` cases for important forbidden edges and private construction. These tests protect the API property that ordinary runtime tests cannot exercise.

Use stub adapters for transition tests and run each adapter's port contract suite. Read [`rust-port-adapter-testing`](../rust-port-adapter-testing/SKILL.md) when a transition calls an external system.

## Before finishing

- The transition graph names every state, valid edge, branch, terminal state, effect, and persisted event.
- State-specific data lives in its state type instead of unrelated `Option` or `bool` fields.
- Each transition exists only on its source state and returns only valid destination states.
- Expected domain outcomes are graph branches; operational failures are `eros` errors with context.
- Retryable effects preserve the source state until success.
- State and audit persistence are atomic where the machine is durable.
- Tests cover every edge, branch, error policy, audit rule, and important forbidden transition.
