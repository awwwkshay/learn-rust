---
name: design-rust-exercise
description: Design or revise a hands-on Rust exercise in this repository and save its untouched starter under exercises/. Do not prepare the root working copy or archive a solution.
---

# Design a Rust exercise

Use the repository's `AGENTS.md` for coaching and file conventions. This skill creates only `exercises/<project>/`.

## Choose the project

- Start with a feature that belongs in real software, such as processing request logs, validating configuration, managing a local task queue, or handling a small protocol. Give the learner a concrete user or operator outcome.
- Pick a scope with a useful end-to-end path: input, domain logic, and observable output. Require several connected decisions or functions, not a single isolated function.
- Select a few Rust concepts that arise naturally from that path, such as ownership and borrowing, structs and enums, `Result` and `Option`, collections, iterators, modules, or tests. Do not force unrelated concepts into the same task.
- Inspect current repository work to avoid repeating a completed project and to calibrate difficulty. If little work is available, choose a standard-library CLI with a clear input format and small data set.
- Keep the core milestone achievable through incremental sessions. Put extra behavior in stretch goals.
- Prefer the standard library unless a dependency teaches a concept needed for the task.

## Create the starter

- Create `exercises/<project>/` as a self-contained, unsolved Cargo crate. Do not create its root working copy or a solution archive in this skill.
- Leave substantive behavior behind `todo!()` or equivalent scaffolding. Its README should explain the scenario, requirements, success criteria, optional stretch goals, representative input and expected output. If output is nondeterministic or flexible, label examples accordingly.
- Keep the starter independent of the root workspace. An empty `[workspace]` section in its Cargo manifest is one way; `prepare-rust-exercise` removes it from the root working copy when adding that copy to the root workspace.
- Document exact test and run commands from inside the crate using relative fixture paths, and explain that `prepare-rust-exercise` will make the root working copy. The crate must remain runnable after being archived in `solutions/`.
- Provide types, function signatures, fixture data, and meaningful unit tests for core behavior, errors, and useful edge cases. Leave parsing, state updates, error handling, orchestration, and formatting for the learner where those are learning goals.
- Make tests and sample output agree with the README. Assert exact output only when the format is specified. Check the starter compiles and note that tests intentionally fail at TODOs.
- Briefly tell the learner why the project matters and which Rust concepts they will practice. Suggest a sensible implementation order without giving away the implementation.
- If the exercise intentionally fails to compile, state the expected command and failure in the README and when presenting the task.

## Review before delivery

Check that finishing the core task produces a usable tool or component, each requirement has a way to verify it, and the learner must make meaningful programming decisions. Remove incidental setup and dependencies that do not teach the chosen concepts.

Commit only the exercise and directly necessary repository metadata after validation. Do not stage unrelated changes. Report the commit hash and how to invoke `prepare-rust-exercise` next.
