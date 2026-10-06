# Borrowed HTTP request head parser

This is an untouched starter. Ask to start this exercise when you want `prepare-rust-exercise` to create a working copy at the repository root. The starter compiles; its tests and demo intentionally stop at `todo!()`.

A small HTTP gateway needs to inspect the first lines of a request before deciding what to do with it. Build a parser that returns a view of the request head without copying its field text. Then make header lookup and a compact operator summary work.

Your earlier request-log and inventory parsers built owned values, and your LRU cache returned borrowed values from a collection. Here the main new challenge is **lifetimes across a parsed value**: every `&str` in `RequestHead<'a>` must refer to the caller's input. The supplied types and signature establish that relationship; you decide how to split and validate the input.

## Input and behavior

- The first line has exactly `METHOD TARGET VERSION`, separated by whitespace. `METHOD` contains only uppercase ASCII letters, `TARGET` begins with `/`, and `VERSION` is exactly `HTTP/1.1`.
- Each following nonblank line is `Name: value`. A name is nonempty and contains only ASCII letters, digits, or `-`. Keep the name's original spelling and trim ASCII whitespace around the value. Empty values and repeated names are allowed.
- A blank line ends the head. A final newline is fine; any nonblank line after a blank terminator is an error. Preserve header order.
- Return an error containing the original one-based `line N` for malformed input. The rest of the error wording is yours.
- `header_value` compares names without ASCII case and returns the first match. It returns `None` when absent.
- `format_summary` produces exactly three lines: `METHOD TARGET (VERSION)`, `Host: VALUE` (or `Host: none`), and `Headers: N`, with no trailing newline.

The standard library is enough. Keep borrowed text in the parsed structs; do not create owned copies or leak strings to satisfy the lifetimes. `format_summary` may allocate its output `String`.

## Build it in stages

1. Parse and validate the request line into borrowed slices.
2. Parse headers, retain their input order, and report the correct source line on errors.
3. Implement case-insensitive first-match lookup.
4. Format the summary and run the demo.

From inside this crate, run:

```sh
cargo test
cargo run
```

The completed demo prints:

```text
GET /orders/42 (HTTP/1.1)
Host: example.test
Headers: 3
```

The tests cover valid borrowed fields, lookup, empty headers, malformed lines, and formatting. The starter's test failures and demo panic at TODOs are expected.

## Optional stretch goals

- Accept `\r\n` line endings while keeping all returned fields borrowed.
- Add a method that returns every value for a repeated header name.
