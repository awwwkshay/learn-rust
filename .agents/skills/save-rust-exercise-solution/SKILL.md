---
name: save-rust-exercise-solution
description: Verify a finished root Rust exercise, move it to a numbered attempt under solutions/, remove it from the root Cargo workspace, and commit the complete change.
---

# Save a completed Rust exercise

Use this when the developer says a root exercise is finished or asks to save its solution. Preserve every earlier solution. The root working directory must be gone when this skill finishes.

1. Identify `<project>/` at the repository root and check its README success criteria. Run its tests and sample command from the repository root. If they fail, help resolve the failure before archiving; do not delete the working project.
2. Choose an unused archive path. Use `solutions/<project>/` if available; otherwise use the first available numbered path `solutions/<project>-2/`, `-3/`, and so on. Never overwrite an existing attempt.
3. Move the root working project to the chosen `solutions/<archive>/` path. Omit or remove `target/` and other generated output. Update the archived README with the actual archive name. Document commands that work from the repository root using `--manifest-path solutions/<archive>/Cargo.toml` and root-relative fixture paths, and commands that work after `cd solutions/<archive>` using `cargo test` and `cargo run -- sample.log` (or the applicable fixture).
4. Remove `<project>` from the root Cargo workspace's `members` and add the archive path to `exclude` so it remains an independent crate with its original package name. Update the root README and Cargo.lock as needed.
5. Verify the archived tests and sample command from both locations. Confirm that `<project>/` no longer exists at the repository root. If verification fails, keep the archive intact and repair the issue before committing.

Commit the archive, root project deletion, and directly related workspace and documentation changes in one commit only after the root directory is gone and the archive passes verification. Do not stage unrelated edits. Report the commit hash, archive path, and test result.
