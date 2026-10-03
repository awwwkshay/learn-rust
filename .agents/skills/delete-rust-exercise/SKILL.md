---
name: delete-rust-exercise
description: Delete a named Rust exercise, including its starter, root working copy, and every saved solution attempt, when the developer explicitly requests permanent removal.
---

# Delete a Rust exercise

Use this only when the developer asks to delete a named exercise and its work. Do not use it to finish or archive a solution. Deletion includes the learner's code, so a general cleanup request is not enough authorization.

1. Identify the exact project name from the request. Inventory `exercises/<project>/`, the root `<project>/`, and **all** saved attempts: `solutions/<project>/` plus every `solutions/<project>-N/` directory where `N` is a positive integer. Do not stop at the first attempt or assume the numbering is consecutive. Check each candidate's Cargo manifest and repository status so an unrelated directory with a similar name is not removed. If the project or requested scope is unclear, ask before deleting.
2. Show the developer the concrete paths that will be deleted, including any uncommitted files or changes in them. An explicit request to delete that named exercise authorizes those paths; ask for approval only if the request did not clearly cover a path containing learner work or if uncommitted work makes the intent uncertain. Do not delete unrelated projects or broad parent directories.
3. Remove the authorized starter, root project, and every saved solution attempt identified in step 1. Remove the root project from `workspace.members` if present, and remove deleted starter and solution paths from `workspace.exclude`. Update the root README to remove or revise links and active-project instructions for this exercise. Regenerate or adjust the root Cargo.lock if needed.
4. Re-scan `solutions/` for any remaining attempt belonging to the exercise. Verify that all listed paths are gone, remaining Cargo workspace members still load, and root documentation has no stale reference to the deleted exercise. Ignore generated `target/` output when reviewing changes.

Commit only the deletion and directly related workspace or documentation edits if the developer requested a commit. Never stage unrelated changes. Report the deleted paths and verification result.
