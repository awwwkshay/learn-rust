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
| `timestamp` | When the starter was created, in UTC ISO 8601 format (`YYYY-MM-DDTHH:MM:SSZ`). |
| `title` and `description` | The exercise name and a short summary of what you will build. |
| `topics` | Rust concepts the exercise is designed to practice. |
| `difficulty` | Estimated difficulty of the required exercise, from 1 (small beginner task) to 10 (large, involved task). |
| `solutions` | Saved attempts for that exercise, in the order they were archived. An empty array means no attempt has been saved yet. |

Each object in `solutions` has a `path` to an archived attempt, a UTC `timestamp` for when it was saved, and a `rating` from 1 to 10 for how well that attempt meets the requirements and uses Rust. Solution paths are relative to this repository's root. **Difficulty rates the exercise; rating assesses a particular saved solution.**

## Start a new exercise

1. Ask your assistant to **design a new Rust exercise**. You can suggest a topic or ask it to choose the next useful challenge based on your earlier attempts.
2. Review the new starter's README under `exercises/`. Ask for changes to the task before starting if the scope or requirements need adjustment. The assistant adds the starter to [metadata.json](metadata.json).
3. Ask your assistant to **prepare the exercise for solving**. It copies the starter to a project directory at the repository root and adds that working copy to the Cargo workspace. Then follow the steps under “Work on the active exercise.”

## Solve an existing exercise

1. Choose an exercise from [metadata.json](metadata.json) and read its starter README at the listed `path`.
2. Ask your assistant to **prepare that exercise for solving**. It makes a fresh working copy from the untouched starter, including when you have saved an earlier attempt. An existing root working copy is preserved rather than overwritten.
3. Follow the steps under “Work on the active exercise.”

## Work on the active exercise

1. Read the root working project's README for its requirements, success criteria, and exact test and run commands.
2. Implement the TODOs yourself in that working copy. Run the tests often; starter tests may fail until the behavior is implemented.
3. When you need help, share the specific error or decision you are facing. Ask for a hint, code review, or a fuller explanation at the level you want.
4. When the success criteria pass, ask your assistant to **save the solution**. It verifies your work, archives a new attempt under `solutions/`, removes the root working copy, and records the attempt in `metadata.json`.

The goal is to practice reasoning about Rust, not just to make the tests pass. Try to explain why your ownership and error-handling choices work before moving on.
