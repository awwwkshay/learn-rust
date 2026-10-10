# Parallel file audit

This is your working copy at the repository root. Write your implementation here. The starter compiles; tests and the demo intentionally panic at `todo!()`.

## The situation

A build operator wants a quick audit of several text files before a release. The tool reports each file's size, line count, and number of lines containing `TODO`. Some paths may be unreadable, but the operator still needs results for the others. Scan files concurrently so one slow read need not prevent another from starting.

Your equipment desk kept several related collections consistent on one thread. This exercise adds **thread ownership and joining** while reusing `Result`, owned paths, and a small report type. The design choice is how the parent keeps each path associated with its worker result while returning rows in the same order as the arguments, even if workers finish in another order.

## Required behavior

- `scan_file(path)` reads the whole file as UTF-8. Count its bytes with `String::len()`, lines with `str::lines()`, and lines containing the exact, case-sensitive substring `TODO`. A line containing `TODO` twice counts once. An empty file has all zero counts. Any file I/O error or invalid UTF-8 returns `ScanError::Read`.
- `scan_files(paths)` starts one `std::thread::spawn` worker per path. Each worker scans its own path. Join every worker and return exactly one `ScanResult` per input path, in input order. A read failure becomes that row's `Read` outcome; it does not discard other rows. If a worker panics, use `ScanError::WorkerPanicked` for that path and continue joining the others. An empty path list returns an empty vector.
- `format_report(results)` emits one line per result, with a trailing newline after each. Use the path's display form and these exact formats: `<path>: <bytes> bytes, <lines> lines, <todo_lines> TODO lines`, `<path>: read failed`, or `<path>: worker panicked`. An empty result list produces an empty string. The supplied CLI already passes its arguments to your functions.

The standard library is enough. `thread::spawn(move || ...)` moves owned values into a worker; the closure must own everything it uses because it may outlive the caller's current scope. `JoinHandle::join()` waits for that worker and returns `Result<T, Box<dyn Any + Send>>`: `Ok(T)` is the worker's return value, and `Err(_)` means it panicked. A worker returning `Err(ScanError::Read)` is a normal completed thread, so distinguish that inner result from a failed join. `PathBuf` owns a path; `&Path` borrows one for `scan_file`.

## Example

From the repository root, run:

```sh
cargo test -p parallel-file-audit
cargo run -p parallel-file-audit -- ./parallel-file-audit/fixtures/notes.txt ./parallel-file-audit/fixtures/missing.txt ./parallel-file-audit/fixtures/empty.txt
```

Or from inside this crate, run:

```sh
cargo test -p parallel-file-audit
cargo run -p parallel-file-audit -- fixtures/notes.txt fixtures/missing.txt fixtures/empty.txt
```

When implemented, the run from inside this crate prints exactly:

```text
fixtures/notes.txt: 33 bytes, 3 lines, 2 TODO lines
fixtures/missing.txt: read failed
fixtures/empty.txt: 0 bytes, 0 lines, 0 TODO lines
```

The missing file is deliberate: it shows that one failed read does not stop the audit. `fixtures/invalid-utf8.bin` exercises a second read failure in the tests. The starter's `cargo test` and demo panic at TODOs until you implement them.

## Suggested order

1. Implement `scan_file` and run its focused tests.
2. Implement `format_report`; its test supplies results directly, so it can be solved independently of scanning.
3. Implement `scan_files`: start all workers before joining any, then check output order and per-file errors.
4. Run the full test suite and CLI example.

## Success criteria

`cargo test` passes, the example output matches, and every supplied path has exactly one row in argument order. Explain where each path is owned while its worker runs, and why a read error differs from a thread panic.

## Optional stretch goals

- Add a limit on the number of simultaneous workers so a very large argument list does not create a thread per file.
- Include the underlying I/O error in read failures while keeping the report useful to an operator.
