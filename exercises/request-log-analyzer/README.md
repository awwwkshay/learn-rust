# Request log analyzer

This is the untouched starter. Use `prepare-rust-exercise` to make an active working copy at the repository root. The starter compiles, but its tests fail at `todo!()` until you implement the behavior.

## Start working

From inside this directory, run:

```sh
cargo test
cargo run -- sample.log
```

When this exercise is prepared at the repository root, the working copy uses `cargo test -p request-log-analyzer` and `cargo run -p request-log-analyzer -- request-log-analyzer/sample.log` from the repository root.

An on-call developer wants a quick summary of a web service's request log: traffic by HTTP status and the slowest request. Build a CLI that reads a log file and prints that summary.

You will practice parsing, `Result` error propagation, structs, `Option`, `BTreeMap`, borrowing, file I/O, and command-line arguments. The starter deliberately contains `todo!()` calls; its tests compile but fail until you implement them.

## Input

Each nonblank line contains four whitespace-separated fields:

```text
METHOD PATH STATUS DURATION_MS
GET /api/v1/users/42 200 13
POST /api/v1/projects 201 142
```

`STATUS` must parse as `u16`, and `DURATION_MS` as `u32`. Paths contain no spaces. Skip blank lines. Reject lines with missing, extra, or nonnumeric fields, and include the original file line number in the error. Use `sample.log` for a realistic input file.

## Build it in stages

1. `parse_request`: turn one line into a `Request` or a useful error.
2. `parse_requests`: skip blank lines, collect requests, and add line numbers to errors.
3. `build_report`: count requests by status and select the slowest. On a tie, keep the first request. Empty input has no slowest request.
4. `format_report`: produce the exact format below, with status codes in ascending order and no trailing newline. `println!` in `main` can add the final newline.
5. `run` and `main`: read one file path argument, process the file, print the report, and print errors to stderr with a nonzero exit status. Reject zero or multiple path arguments.

This is one end-to-end tool; the functions separate the parts you can test while you build it. The standard library is enough.

## Expected output

From inside the finished project directory, run `cargo run -- sample.log`. It should print:

```text
Requests: 16
Status counts:
  200: 7
  201: 3
  204: 1
  401: 1
  404: 1
  429: 1
  500: 1
  503: 1
Slowest: GET /api/v1/reports/weekly (503, 1204 ms)
```

For an empty file, print `Requests: 0`, `Status counts:`, then `Slowest: none` on separate lines. For a malformed line, the error wording is your choice, but it must include `line N` with the correct one-based line number.

## Success criteria

From inside this directory, run:

```sh
cargo test
cargo run -- sample.log
```

The starter compiles but tests intentionally fail at the TODOs. Once complete, all unit tests should pass, and the sample output should match above. Also try a missing file, malformed input, and missing CLI argument; each should print an error and exit nonzero.

## Optional stretch goals

- Add a test for `run` using a temporary file.
- Report average duration per path.
- Add a `--status 500` filter.
