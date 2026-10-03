---
name: design-rust-exercise
description: Design or revise hands-on Rust learning projects in this repository when the user asks for a new exercise, a project challenge, or a better learning task. Keep the implementation for the learner unless they explicitly request a solution.
---

# Design a Rust exercise

Use the repository's `AGENTS.md` for coaching and file conventions. This skill adds a design check for project quality.

## Choose the project

- Start with a feature that belongs in real software, such as processing request logs, validating configuration, managing a local task queue, or handling a small protocol. Give the learner a concrete user or operator outcome.
- Pick a scope with a useful end-to-end path: input, domain logic, and observable output. Require several connected decisions or functions, not a single isolated function.
- Select a few Rust concepts that arise naturally from that path, such as ownership and borrowing, structs and enums, `Result` and `Option`, collections, iterators, modules, or tests. Do not force unrelated concepts into the same task.
- Inspect current repository work to avoid repeating a completed project and to calibrate difficulty. If little work is available, choose a standard-library CLI with a clear input format and small data set.
- Keep the core milestone achievable through incremental sessions. Put extra behavior in stretch goals.
- Prefer the standard library unless a dependency teaches a concept needed for the task.

## Scaffold without solving

- Create a descriptively named Cargo project in its own directory. Its `README.md` should explain the scenario, requirements, success criteria, exact commands, optional stretch goals, and a representative input with the output the finished program should produce. If output is nondeterministic or its formatting is flexible, label the output as illustrative and state which parts must match.
- Write each project's README test and run commands for execution from the repository root. For example, `request-log-analyzer/README.md` uses `cargo test -p request-log-analyzer` and `cargo run -p request-log-analyzer -- request-log-analyzer/sample.log`. Select the exercise package and use root-relative input paths in other projects too. Check that the documented commands work from the root.
- Provide the types, function signatures, fixture data, and meaningful unit tests for core behavior, errors, and useful edge cases. Use `todo!()` for substantive behavior. Leave parsing, state updates, error handling, orchestration, and formatting for the learner where those are learning goals.
- Make tests and the sample output agree with the README's required behavior. Assert exact output only when the format is specified; otherwise test required content or structure. Check that the scaffold compiles, and state clearly if tests intentionally fail at TODOs.
- Briefly tell the learner why the project matters and which Rust concepts they will practice. Suggest a sensible implementation order without giving away the implementation.
- If the exercise intentionally fails to compile, state the expected command and failure in the README and when presenting the task.

## Review before delivery

Check that finishing the core task produces a usable tool or component, each requirement has a way to verify it, and the learner must make meaningful programming decisions. Remove incidental setup and dependencies that do not teach the chosen concepts.
