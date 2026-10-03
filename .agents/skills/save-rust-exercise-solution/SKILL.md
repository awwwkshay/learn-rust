---
name: save-rust-exercise-solution
description: Verify a finished root Rust exercise, archive it as a numbered attempt under solutions/, remove the root working project, update Cargo and docs, and commit the result.
---

# Save a completed Rust exercise

Use this when the developer says a root exercise is finished or asks to save its solution. The request authorizes removing that root working project after a verified archive. Preserve every earlier solution.

1. Identify `<project>/` at the repository root and check its README success criteria. Run its tests and sample command from the repository root. If they fail, help resolve the failure before archiving; do not delete the working project.
2. Choose an unused archive path. Use `solutions/<project>/` if available; otherwise use the first available numbered path `solutions/<project>-2/`, `-3/`, and so on. Never overwrite an existing attempt.
3. Copy the project into that archive, excluding `target/` and other generated output. Update the archived README with the actual archive name. Document commands that work from the repository root using `--manifest-path solutions/<archive>/Cargo.toml` and root-relative fixture paths, and commands that work after `cd solutions/<archive>` using `cargo test` and `cargo run -- sample.log` (or the applicable fixture).
4. Add the archive path to the root Cargo workspace's `exclude` list so it remains an independent crate with its original package name. Verify the archived tests and sample command from both locations. Compare source and fixture files with the root project before removing it.
5. Remove `<project>/` from the root workspace's `members`, then remove the root working directory only after the archive is verified. Update the root README and Cargo.lock as needed. Confirm the archived project still works after removal.

Commit the archive, root project removal, and directly related workspace and documentation changes in one commit. Do not stage unrelated edits. Report the commit hash, archive path, and test result.
