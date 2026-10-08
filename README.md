# Learn Rust

This repository is for learning Rust by building small projects yourself. Each exercise has a starter, tests, and a README that describes the behavior to implement.

## What you need

- A Rust toolchain with `cargo` available in your terminal.
- A coding assistant that can work with this repository, such as Codex or Claude, to prepare exercises and coach you when you get stuck.
- A willingness to write the Rust implementation yourself, run tests, and learn from compiler errors.

## How the repository works

- [metadata.json](metadata.json) lists the recommended solving order, difficulty, topics, and saved solution attempts.
- `exercises/` contains untouched starters. Leave these as a reference.
- A project directory at the repository root is your active working copy. Write your solution there.
- `solutions/` contains archived attempts. You can solve an exercise again without replacing an earlier attempt.

## Reading `metadata.json`

The `exercises` array is in recommended solving order. Each entry describes one untouched starter:

| Field | Meaning |
| --- | --- |
| `slug` | Directory name under `exercises/`; use it when asking your assistant to prepare an exercise. |
| `path` | Location of the untouched starter directory, relative to this repository's root. |
| `title` and `description` | The exercise name and a short summary of what you will build. |
| `topics` | Rust concepts the exercise is designed to practice. |
| `difficulty` | Estimated difficulty of the required exercise, from 1 (small beginner task) to 10 (large, involved task). |
| `solutions` | Saved attempts for that exercise, in the order they were archived. An empty array means no attempt has been saved yet. |

Each object in `solutions` has a `path` to an archived attempt, relative to this repository's root, and a `rating` from 1 to 10 for how well that attempt meets the requirements and uses Rust. **Difficulty rates the exercise; rating assesses a particular saved solution.**

## Practice loop

1. Choose an exercise from [metadata.json](metadata.json), then ask your assistant to prepare it.
2. Read the active project's README for requirements, examples, and its exact test and run commands.
3. Implement the TODOs in the root working copy. Run the tests often and use compiler errors as clues.
4. When you need help, share the specific error or decision you are facing. Ask for a hint, a review, or a full explanation at the level you want.
5. When the exercise is finished, ask your assistant to save the solution. It will verify the work, archive a new attempt, and update the metadata.

The goal is to practice reasoning about Rust, not just to make the tests pass. Try to explain why your ownership and error-handling choices work before moving on.
