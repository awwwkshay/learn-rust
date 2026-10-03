# Inventory ledger

This is the completed first attempt archived at `solutions/inventory-ledger/`.

A small shop records deliveries and sales in a plain-text file. Build a CLI that processes the file in order and prints the stock left for each item. A sale that exceeds available stock must stop processing with a useful error.

## Input

Each nonblank line is one command, with whitespace-separated fields:

```text
RECEIVE ITEM QUANTITY
SELL ITEM QUANTITY
```

`ITEM` is a nonempty ASCII identifier containing only letters, digits, `_`, or `-`. Identifiers are case-sensitive. `QUANTITY` is a positive `u32` integer. Reject unknown commands, missing or extra fields, invalid identifiers, zero or nonnumeric quantities, quantities that overflow `u32`, sales above current stock, and deliveries that would overflow an item's stock. Blank lines may contain whitespace and are ignored. Include the original one-based `line N` in every command or stock error.

Each item starts with zero stock. Items that reach zero remain in the final report. Print items in ascending identifier order.

## Build it in stages

1. `parse_command` validates one nonblank line and returns a `Command`.
2. `apply_command` updates the inventory for one command or returns an error. An unsuccessful command must leave inventory unchanged.
3. `process_ledger` skips blank lines, processes commands in order, and adds source line numbers to errors.
4. `format_inventory` produces the exact report format below, without a trailing newline.
5. `run` and `main` read exactly one file path, print the report, or print an error to stderr and exit nonzero.

This solution uses the `regex` crate for item validation. It practices enums, `Result`, mutable collections, checked arithmetic, ownership, file I/O, and testing state changes.

## Example

From this directory, run:

```sh
cargo run -- sample.ledger
```

The completed program prints:

```text
Inventory:
  bolts: 5
  nuts: 0
  washers: 4
```

For an empty ledger, print just `Inventory:`. Error wording is flexible, but the error must include the relevant `line N` for file contents.

## Success criteria

From the repository root, run:

```sh
cargo test --manifest-path solutions/inventory-ledger/Cargo.toml
cargo run --manifest-path solutions/inventory-ledger/Cargo.toml -- solutions/inventory-ledger/sample.ledger
```

From this directory, run:

```sh
cargo test
cargo run -- sample.ledger
```

Also try a missing file, a missing or extra CLI argument, a sale before a delivery, and malformed input. Each must print an error and exit nonzero.

## Optional stretch goals

- Add a `RETURN ITEM QUANTITY` command with a policy for items never received.
- Add an integration test that invokes the CLI with a temporary ledger file.
