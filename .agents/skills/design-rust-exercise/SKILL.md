---
name: design-rust-exercise
description: Design or revise a hands-on Rust exercise in this repository and save its untouched starter under exercises/. Do not prepare the root working copy or archive a solution.
---

# Design a Rust exercise

Use the repository's `AGENTS.md` for coaching and file conventions. This skill creates only `exercises/<project>/`.

## Learn from prior work

- Before choosing a new exercise, inspect `exercises/` for existing starters, `solutions/` for completed attempts, and any root working project. Read the relevant READMEs, tests, and learner-written code; use repository history or coaching context when it helps explain what the learner actually practiced. Distinguish a designed starter from a completed attempt.
- Compare each completed attempt with its starter requirements and tests. Make a brief working assessment of concepts already practiced, evidence of recurring difficulties or gaps in required behavior, unfinished stretch goals, and concepts the learner has not yet used. Use actual solution code and available coaching history to support any claim about where the learner is struggling. A passing solution shows experience with that task, not mastery of every concept it contains; an unpracticed concept is not a weakness. Do not infer a weakness from a single style choice.
- Choose a project that builds on that evidence: reuse familiar concepts in a different context and introduce one main new Rust challenge at a manageable step up. Avoid repeating a completed project or stacking several unfamiliar ideas in the core milestone. Put larger jumps and adjacent topics in stretch goals or a later exercise.
- If prior work is sparse or the intended direction is unclear, choose a reasonable next step from the available evidence. Ask about level or interests only when the answer would materially change the exercise.

## Choose the project

- Start with a feature that belongs in real software, such as processing request logs, validating configuration, managing a local task queue, or handling a small protocol. Give the learner a concrete user or operator outcome.
- Pick a scope with a useful end-to-end path: input, domain logic, and observable output. Require several connected decisions or functions, not a single isolated function.
- Select a few Rust concepts that arise naturally from that path, such as ownership and borrowing, structs and enums, `Result` and `Option`, collections, iterators, modules, or tests. Do not force unrelated concepts into the same task.
- If little work is available, choose a standard-library CLI with a clear input format and small data set.
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
- In the README and delivery message, briefly connect the new challenge to prior completed work and identify the main new concept. Keep the learner's TODOs and design decisions for them to solve.
- If the exercise intentionally fails to compile, state the expected command and failure in the README and when presenting the task.

## Review before delivery

Check that finishing the core task produces a usable tool or component, each requirement has a way to verify it, and the learner must make meaningful programming decisions. Remove incidental setup and dependencies that do not teach the chosen concepts.

Commit only the exercise and directly necessary repository metadata after validation. Do not stage unrelated changes. Report the commit hash and how to invoke `prepare-rust-exercise` next.
