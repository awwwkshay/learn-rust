// Exercise 2: Generic functions and trait bounds
//
// A generic function like `fn largest<T>(items: &[T]) -> &T` compiles
// fine in C#/TS as long as *some* type satisfies whatever operation you
// use inside -- the check happens per-call. In Rust, a generic function
// is checked ONCE, at its own definition, against nothing but what its
// trait bounds promise. `fn largest<T>` with no bound means the compiler
// assumes T could be absolutely anything -- including a type with no
// notion of ordering -- so it refuses to let you compare two `T`s with
// `>` until you say `T: PartialOrd` (or a stricter bound like `T: Ord`).
//
// This models finding the package with the highest download count in a
// registry search-results list.
//
// Goal: make this compile AND make the assertions pass, changing as FEW
// lines as possible. Don't delete the assertions.
//
// Run with:
//   cargo run --bin ex2_generic_bound

// BUG: this function compares two `T` values with `>`, but nothing tells
// the compiler T supports comparison. Add the trait bound that grants it.
fn largest<T : PartialOrd>(items: &[T]) -> &T {
    let mut max = &items[0];
    for item in items {
        if item > max {
            max = item;
        }
    }
    max
}

fn main() {
    let downloads = vec![120, 4500, 380, 99];
    assert_eq!(*largest(&downloads), 4500);

    let names = vec!["core", "cli", "zzz-tools", "auth"];
    assert_eq!(*largest(&names), "zzz-tools");

    println!("ex2 passed");
}
