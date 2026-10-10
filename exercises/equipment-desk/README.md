# Equipment desk

This is an untouched starter under `exercises/equipment-desk/`. Ask to start it when you want `prepare-rust-exercise` to make a root working copy and add that copy to the Cargo workspace. Work in the root copy, not here. The starter compiles; its tests and demo intentionally stop at `todo!()`.

## The situation

A community workshop lends equipment such as cameras and tripods. Staff need to know who currently has each tool, who is waiting, and what happens when equipment is returned or more copies arrive. Build the **in-memory checkout desk** behind that workflow. A caller will invoke methods over time; there is no file parser or command interpreter to build.

This is deliberately larger than the earlier event feed and message pipeline. Those projects moved values through one path. Here, several operations must maintain the same state over a sequence of actions. The main new challenge is **coordinating state changes across collections while respecting Rust's ownership and borrowing rules**. You will use `BTreeMap`, `BTreeSet`, `VecDeque`, enums, and `Result` in a connected component.

## Public behavior

Keep the public types and method signatures in `src/lib.rs`. You may change private fields or add private helpers. A tool name and borrower name are case-sensitive and kept exactly as supplied; they are invalid only when empty or all whitespace. Do not trim a valid name. A borrower may use different tools independently, but may have only one loan **or** waitlist entry for any one tool.

| Operation | Required behavior |
| --- | --- |
| `new()` | Start with no registered tools. |
| `add_tool(name, copies)` | Register a new tool with a positive number of copies. Reject a blank name, zero copies, or a name already registered. On success it has no loans or waiters. |
| `checkout(tool, borrower)` | If a copy is free, loan it and return `Loaned`. Otherwise append the borrower to that tool's waitlist and return `Waitlisted { position }`, where position starts at 1. Reject a blank borrower, an unknown tool, or someone already borrowing or waiting for that same tool. |
| `return_tool(tool, borrower)` | Only a current borrower may return a copy. Remove their loan. If the waitlist is nonempty, immediately loan the returned copy to its first person and return `AssignedTo(name)`; otherwise return `Available`. |
| `cancel_wait(tool, borrower)` | Remove a waiting borrower from anywhere in this tool's queue. Keep everyone else's relative order. It is an error if the person is not waiting, including when they currently have a loan. |
| `add_copies(tool, copies)` | Add a positive number of copies. Before leaving any copy available, assign free copies to waiting people in FIFO order. Return a `Vec<String>` of the people assigned by this call, in that order. Reject an unknown tool, zero copies, or a `u32` overflow. |
| `status(tool)` | Return an owned `ToolStatus` snapshot. `available` is the number of copies without a borrower. `borrowers` is sorted by name, and `waitlist` follows arrival order. Reject an unknown tool. Calling this must not change desk state. |

Use the supplied `DeskError` variants for these failures:

| Operation | Failure | Error |
| --- | --- | --- |
| `add_tool` | Blank tool name | `InvalidToolName` |
| `add_tool` | Zero copies | `InvalidCopies` |
| `add_tool` | Tool already registered | `ToolExists(tool name)` |
| `checkout`, `return_tool`, `cancel_wait` | Blank borrower name | `InvalidBorrower` |
| Any operation that looks up a tool | Unknown tool | `UnknownTool(tool name)` |
| `checkout` | Borrower already has a loan or waitlist entry for this tool | `AlreadyRegistered(borrower name)` |
| `return_tool` | Borrower does not currently have a loan for this tool | `NotBorrowed(borrower name)` |
| `cancel_wait` | Borrower is not waiting for this tool, even if they have a loan | `NotWaiting(borrower name)` |
| `add_copies` | Zero copies | `InvalidCopies` |
| `add_copies` | New total exceeds `u32::MAX` | `CapacityOverflow(tool name)` |

The tests do not require a particular error precedence when a call has multiple invalid arguments.

## State rules

- Each tool's total copy count stays positive and can grow only through `add_copies`.
- A person cannot appear twice for the same tool or be both a borrower and a waiter for that tool.
- The number of active borrowers cannot exceed the total copy count.
- A nonempty waitlist means all copies are currently loaned. Returning or adding a copy must serve the queue before making a copy available.
- Any operation returning `Err` leaves the desk unchanged. In particular, check stock overflow before changing the count or assigning a waiter.
- Operations on one tool must not affect another tool.

The stock overflow case is a small twist: `add_copies` must reject the whole change if the new total cannot fit in `u32`, leaving every existing loan and waitlist entry intact.

## Example sequence

With one `camera` copy, Ada checks it out. Mira and Leo then join the waitlist at positions 1 and 2. Adding one copy assigns it to Mira. When Ada returns her copy, it goes directly to Leo. At the end, the camera has two copies, both loaned, with no one waiting.

From inside this crate, run:

```sh
cargo test
cargo run
```

Once implemented, the supplied demo prints exactly:

```text
Ada: Loaned
Mira: Waitlisted { position: 1 }
Leo: Waitlisted { position: 2 }
added copies: ["Mira"]
Ada returns: AssignedTo("Leo")
camera: 0 available; borrowers ["Leo", "Mira"]; waiting []
```

The demo is only one workflow. The library is the product: another Rust program should be able to create a desk, make calls, inspect outcomes, and continue using it.

## Suggested implementation order

1. Implement `add_tool` and `status` so you can create and inspect a tool.
2. Implement `checkout`, first for a free copy, then for a full tool and duplicate borrowers.
3. Implement `return_tool` and `cancel_wait`, checking the queue after each state change.
4. Implement `add_copies`, including FIFO promotion and overflow without partial changes.
5. Run the full demo and review the state rules against your implementation.

The unit tests set up some private records directly. That lets you work on one operation without first finishing every earlier TODO. They cover an end-to-end workflow, empty registration, duplicate and invalid calls, queue order, return and cancellation transitions, snapshots, independent tools, and stock overflow.

## Success criteria

From this crate directory, `cargo test` passes and `cargo run` prints the sequence above. Review the state rules as well as the tests. The untouched starter's test failures and demo panic at TODOs are expected.

## Optional stretch goals

- Add `remove_tool` that succeeds only when the tool has no active loans or waiters.
- Add a method to transfer a waitlist position from one borrower to another without changing its place in the queue.
- Split the implementation into modules after the core behavior works, while keeping the public API intact.
