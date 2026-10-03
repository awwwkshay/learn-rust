---
name: prepare-rust-exercise
description: Copy an existing Rust starter from exercises/ into a root working project, add it to the Cargo workspace, verify commands, and commit the setup.
---

# Prepare a Rust exercise for solving

Use this when the developer chooses an exercise to start or restart. Do not implement its TODOs.

1. Select `exercises/<project>/` and check whether `<project>/` already exists at the repository root. Preserve existing work; do not overwrite or merge it automatically.
2. Copy the starter into `<project>/`, excluding generated files such as `target/`. If the starter manifest has its own `[workspace]` section, remove it from the working copy so the root Cargo workspace can own that package.
3. Add `<project>` to the root `workspace.members`. Keep the exercise starter independent. Update the root README to point to the active working copy.
4. Document commands for each working directory, using paths relative to that directory. In both the root README and the active project's README, show `cargo test -p <package>` and `cargo run -p <package> -- ./<project>/sample.log` for use from the repository root. Also show `cargo test -p <package>` and `cargo run -p <package> -- sample.log` for use after `cd <project>`. Substitute the actual package, project, and fixture names. Do not put a project-local fixture path in a root command.
5. Run the documented commands from their stated directories, not merely `cargo check` or `cargo metadata`. Confirm they select the intended crate and resolve the fixture path. Tests and runs may fail at TODOs; distinguish those expected failures from a wrong package or missing file.

Commit only this preparation and directly necessary workspace or documentation changes. Do not stage unrelated edits. Report the commit hash and the root-level command to start working.
