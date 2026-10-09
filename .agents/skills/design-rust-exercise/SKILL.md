---
name: design-rust-exercise
description: Design or revise a hands-on Rust exercise, save its untouched starter under exercises/, and maintain root metadata.json. Do not prepare the root working copy or archive a solution.
---

# Design a Rust exercise

Use the repository's `AGENTS.md` for coaching and file conventions. This skill creates an untouched starter under `exercises/<project>/` and maintains the root `metadata.json`. Do not prepare a root working copy or archive a solution.

## Learn from prior work

- Before choosing a new exercise, inspect `exercises/` for existing starters, `solutions/` for completed attempts, and any root working project. Read the relevant READMEs, tests, and learner-written code; use repository history or coaching context when it helps explain what the learner actually practiced. Distinguish a designed starter from a completed attempt.
- Compare each completed attempt with its starter requirements and tests. Make a brief working assessment of concepts already practiced, evidence of recurring difficulties or gaps in required behavior, unfinished stretch goals, and concepts the learner has not yet used. Use actual solution code and available coaching history to support any claim about where the learner is struggling. A passing solution shows experience with that task, not mastery of every concept it contains; an unpracticed concept is not a weakness. Do not infer a weakness from a single style choice.
- Make gradual Rust mastery the primary goal of the exercise sequence. Build on concepts demonstrated in completed solutions, revisit a shaky concept in a new context when evidence supports it, and introduce one main new Rust concept at a manageable step up. Compare the whole workflow and program shape with recent exercises, not just the domain or concept list. Avoid repeating a completed project or stacking several unfamiliar ideas in the core milestone. Put larger jumps and adjacent topics in stretch goals or a later exercise.
- Steer the sequence toward the learner's day-to-day work: production-minded APIs, async services, AI agents and workflows, and eventually Rust UI. First build familiarity with Rust threading and async programming through small, useful tasks; do not make an API or agent workflow the learner's first encounter with those concepts. Existing exercises already cover request logs, stateful collections, a cache, jobs, and pipelines; use those as foundations rather than repeating them in isolation.
- If prior work is sparse or the intended direction is unclear, choose a reasonable next step from the available evidence. Ask about level or interests only when the answer would materially change the exercise.

## Choose the project

- Start with a feature that belongs in real software and give the learner a concrete user or operator outcome. Before networked services, use small tasks to teach thread ownership and `join`, message passing, shared state with `Arc` and a lock, and when `Send` and `Sync` matter. Then introduce `async fn`, futures, `.await`, the Tokio runtime, spawning and joining tasks, and the distinction between concurrent waiting and parallel CPU work. Teach cancellation, timeouts, streams, and bounded work in later focused steps as the task needs them. Do not require all of these in one exercise.
- Let the prior attempt determine the next step. A possible path is a small threaded task → a small Tokio task with concurrent waits → HTTP request/response basics and streaming → an Axum endpoint and state → validation and API boundaries → caching, rate limiting, authentication and authorization, OAuth flows, encryption, and sockets → an AI workflow with replaceable external dependencies → a small Dioxus UI consuming an API. This is a direction, not a fixed syllabus; choose the next task from demonstrated readiness and the learner's current interests. Introduce later concerns as separate, connected exercises rather than one large service.
- Pick a scope with a useful end-to-end path: input, domain logic, and observable output. Require several connected decisions or functions, not a single isolated function.
- Select a few Rust concepts that arise naturally from that path, such as ownership and borrowing, structs and enums, `Result` and `Option`, collections, iterators, modules, or tests. Do not force unrelated concepts into the same task.
- Balance Rust practice with one concrete low-level design decision: where state lives, what an interface exposes, which invariants it maintains, how components coordinate, or how failures are represented. Let the learner make and explain that decision through the task's behavior and tests; avoid turning a small exercise into a broad architecture assignment.
- Give the core task one small twist, such as a meaningful constraint, failure case, or state change that makes the new concept necessary and asks the learner to reason beyond the familiar path. Keep the twist part of the real behavior, not an arbitrary complication, and make it visible in a focused test.
- If little work is available, choose the smallest task that exposes the next needed concept. Check for actual practice with threading and async before making them prerequisites of a networked task.
- Keep the core milestone achievable through incremental sessions. Put extra behavior in stretch goals.
- Use Axum for web servers, Tokio for async execution, and `futures` where stream or future composition is the learning goal. Dioxus is the preferred UI direction when a frontend exercise fits. Other crates are welcome when they represent how this feature would reasonably be built; check their current documentation and maintenance before choosing them. Explain each new crate's role and relevant Rust API in the README so dependency setup does not become a guessing exercise. Keep the dependency set small and avoid making the learner implement security primitives or protocol machinery that a suitable crate should provide.
- Scope the core milestone around one new application concern, one main Rust or async concept, and one related design choice. For example, a rate limiter can focus on shared state and its expiration rule without also requiring OAuth, persistence, and deployment. Use local fakes or deterministic fixtures for external AI providers, credentials, clocks, and network peers when those are not the concept being taught. Tests should exercise observable behavior and important failure or boundary cases without requiring paid services.

## Create the starter

- Create `exercises/<project>/` as a self-contained, unsolved Cargo crate. Do not create its root working copy or a solution archive in this skill.
- Leave substantive behavior behind `todo!()` or equivalent scaffolding. Its README should explain the scenario, requirements, success criteria, optional stretch goals, representative input and expected output. If output is nondeterministic or flexible, label examples accordingly.
- Keep the starter independent of the root workspace. An empty `[workspace]` section in its Cargo manifest is one way; `prepare-rust-exercise` removes it from the root working copy when adding that copy to the root workspace.
- Document exact test and run commands from inside the crate using relative fixture paths, and explain that `prepare-rust-exercise` will make the root working copy. The crate must remain runnable after being archived in `solutions/`.
- Provide types, function signatures, fixture data, and unit tests for distinct usage scenarios: a normal end-to-end path, relevant empty or boundary inputs, failure paths, and ordering or state transitions when they matter. Make each scenario test a behavior the learner can reason about; avoid tests that merely change input values or mirror the implementation. Keep tests focused enough that an earlier TODO does not mask every later behavior—for example, construct a report directly when testing its formatter. Leave parsing, state updates, error handling, orchestration, and formatting for the learner where those are learning goals.
- Make tests and sample output agree with the README. Assert exact output only when the format is specified. Check the starter compiles and note that tests intentionally fail at TODOs.
- Briefly tell the learner why the project matters, which Rust concepts they will practice, and which design tradeoff they will consider. Suggest a sensible implementation order without giving away the implementation.
- For a threading or async exercise, explain the small set of relevant primitives and their purpose directly in the README. Make the learner reason about ownership, coordination, and failure behavior rather than recall method names or runtime setup from memory.
- In the README and delivery message, briefly connect the new challenge to prior completed work and identify the main new concept. Keep the learner's TODOs and design decisions for them to solve.
- If the exercise intentionally fails to compile, state the expected command and failure in the README and when presenting the task.

## Maintain exercise metadata

- Update the root `metadata.json` whenever you create or revise an exercise. It contains one `exercises` array in recommended solving order. Each entry has a unique `slug`, a root-relative `path` set to `exercises/<slug>`, a `timestamp` in UTC ISO 8601 format (`YYYY-MM-DDTHH:MM:SSZ`), a human-readable `title`, a short `description` of the project, a `topics` array naming the Rust concepts the learner will practice, an integer `difficulty` from 1 to 10, and a `solutions` array.
- Set a new exercise's `timestamp` to the current UTC time when its starter is created. Treat it as a creation timestamp: preserve it when revising that exercise. Do not copy a local-time offset into the metadata.
- Assign `difficulty` from the required core work, not optional stretch goals: 1 means a small beginner exercise and 10 means a large exercise with several interacting Rust concepts and state rules. For a new exercise, initialize `solutions` to `[]`; when revising one, preserve every existing solution object.
- For a new exercise, insert one entry at the appropriate point in the learning sequence; usually this is after the exercise it builds on. For a revision, update its existing entry rather than creating a duplicate. Keep descriptions and topics consistent with the exercise README and the actual starter.
- Preserve other entries and their order unless the learning sequence itself needs to change. Do not store active or completed status here; those change when exercises are prepared or saved.
- Validate that `metadata.json` is valid JSON, every starter has exactly one entry, each `path` equals `exercises/<slug>` and points to an existing starter, timestamps use UTC ISO 8601 format, every difficulty is an integer from 1 to 10, and existing solution objects are preserved before committing.

## Review before delivery

Check that finishing the core task produces a usable tool or component, each requirement has a way to verify it, and the learner must make meaningful programming decisions. Remove incidental setup and dependencies that do not teach the chosen concepts.

Commit only the exercise, its metadata update, and directly necessary repository changes after validation. Do not stage unrelated changes. Report the commit hash and how to invoke `prepare-rust-exercise` next.
