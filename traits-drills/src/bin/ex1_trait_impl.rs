// Exercise 1: Defining and implementing a trait
//
// In C#, you declare `interface ISummarize { string Summary(); }` then a
// class opts in with `class Package : ISummarize`. Rust splits this the
// same way it splits struct/impl: a `trait` declares the contract, and a
// separate `impl Trait for Type` block provides it -- even for types you
// don't own (can't do that in C# without editing the original class).
//
// This models a registry CLI that has one generic "print a summary line"
// function, meant to work for any type that knows how to summarize
// itself -- Package today, other types later, without touching this
// function again.
//
// Goal: make this compile AND make the assertions pass, changing as FEW
// lines as possible. Don't delete the assertions or the print_line
// function's signature (it must stay generic over anything Summarize).
//
// Run with:
//   cargo run --bin ex1_trait_impl

trait Summarize {
    fn summary(&self) -> String;
}

fn print_line<T: Summarize>(item: &T) -> String {
    format!("- {}", item.summary())
}

struct Package {
    name: String,
    version: u32,
}

impl Summarize for Package {
    fn summary(&self) -> String {
        format!("{} v{}", self.name, self.version)
    }
}

// BUG: Package needs to actually implement the Summarize trait for
// print_line::<Package> to work -- right now there's no impl at all.
// Add one.

fn main() {
    let pkg = Package { name: "core".to_string(), version: 3 };

    assert_eq!(print_line(&pkg), "- core v3");

    println!("ex1 passed");
}
