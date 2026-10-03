---
name: prepare-rust-exercise
description: Copy an existing Rust starter from exercises/ into a root working project, add it to the Cargo workspace, verify commands, and commit the setup.
---

# Prepare a Rust exercise for solving

Use this when the developer chooses an exercise to start or restart. Do not implement its TODOs.

1. Select `exercises/<project>/` and check whether `<project>/` already exists at the repository root. Preserve existing work; do not overwrite or merge it automatically.
2. Copy the starter into `<project>/`, excluding generated files such as `target/`. If the starter manifest has its own `[workspace]` section, remove it from the working copy so the root Cargo workspace can own that package.
3. Add `<project>` to the root `workspace.members`. Keep the exercise starter independent. Update the root README to point to the active working copy.
4. Give exact commands from the repository root, for example `cargo test -p <package>` and `cargo run -p <package> -- <project>/sample.log`. Also document commands from inside `<project>/`: `cargo test` and `cargo run -- sample.log`. Use the actual package name and fixture.
5. Verify that the working copy compiles and that both root and project-local Cargo commands select the intended crate. Tests and runs may fail at TODOs; explain that expected result.

Commit only this preparation and directly necessary workspace or documentation changes. Do not stage unrelated edits. Report the commit hash and the root-level command to start working.
