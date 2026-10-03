# Request log analyzer

Build a small command-line tool that reads a web service's request log and reports how many requests it handled, how many had each HTTP status, and which request was slowest. This is a simplified version of operational reporting used by web services.

## Learning goals

Connect file I/O, command-line arguments, parsing, `Result` error handling, structs, `Option`, collections, and borrowing in one program.

## Input

Each nonempty line has four whitespace-separated fields:

```text
METHOD PATH STATUS DURATION_MS
GET /users 200 17
POST /users 201 42
GET /missing 404 5
```

The status must parse as `u16`; the duration must parse as `u32`. Treat missing, extra, or malformed fields as errors. Empty lines may be skipped. A sample file is in `sample.log`.

## Requirements

1. Implement `parse_request` to validate one line and return a `Request` or a useful error.
2. Implement `build_report` to count statuses and find the slowest request. For an empty input, `slowest` is `None`. If durations tie, keep the first request.
3. Implement `format_report` to show the total, each status count, and the slowest request (or say there were no requests). Choose a readable format; status codes should be sorted by the `BTreeMap`.
4. Implement `run` to read a log file, skip empty lines, and return the formatted report. On a malformed line, include its line number in the error.
5. Implement `main` to accept exactly one file path argument, print the report, and print errors to stderr with a nonzero exit status.

The starter deliberately uses `todo!()` in each function. It compiles, but its tests fail until you implement the behavior.

## Success criteria

From this directory, run:

```sh
cargo test
cargo run -- sample.log
```

All tests should pass. The sample run should report 4 requests, status counts of 200: 2, 201: 1, and 404: 1, with `POST /users` as the slowest at 42 ms. Also try a missing file and a malformed line; both should produce a useful error and nonzero exit status.

## Optional stretch goals

- Add tests for `run` using a temporary file.
- Report average duration per path.
- Add a `--status 500` filter without changing the input format.
