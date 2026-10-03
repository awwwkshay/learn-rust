// Exercise 4: Lifetimes
//
// Python/TS/C# all have a garbage collector, so a function can freely
// return a reference into something it was given and the GC keeps the
// underlying data alive as long as anyone points to it. Rust has no GC:
// when a function returns a borrowed reference, the compiler must prove
// the data it points to will outlive the reference. When a function takes
// more than one reference and returns one of them, the compiler can't
// guess which input the output is tied to -- you must say so with a
// lifetime annotation.
//
// This models picking the "preferred" registry mirror out of a primary
// and fallback URL, from the adico registry crate's mirror-selection code.
//
// Goal: make this compile AND make the assertions pass, changing as FEW
// lines as possible (this one is solved by ADDING lifetime annotations,
// not by restructuring the code). Don't delete the assertions.
//
// Run with:
//   cargo run --bin ex4_lifetimes

// BUG: two input references, one output reference -- the compiler has no
// way to know if the return value borrows from `primary` or `fallback`.
// Add lifetime annotations that tell it both inputs share one lifetime and
// the output borrows from that same lifetime.
fn pick_mirror<'a >(primary: &'a str, fallback: &'a str, prefer_fallback: bool) -> &'a str {
    if prefer_fallback { fallback } else { primary }
}

fn main() {
    let primary = String::from("https://primary.example");
    let chosen;
    {
        let fallback = String::from("https://fallback.example");
        chosen = pick_mirror(&primary, &fallback, false).to_string();
    }

    assert_eq!(chosen, "https://primary.example");
    assert_eq!(pick_mirror("a", "b", true), "b");

    println!("ex4 passed");
}
