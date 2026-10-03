// Exercise 3: Trait objects (dyn Trait) for heterogeneous collections
//
// Generics (`fn print_line<T: Summarize>`) get you polymorphism resolved
// at COMPILE time -- great, but it means a `Vec<T>` can only ever hold
// ONE concrete type T. In C#/TS, `List<ISummarize>` freely holds a mix of
// any classes implementing the interface, resolved at runtime (a vtable
// lookup under the hood). Rust's equivalent is a trait object:
// `Vec<Box<dyn Summarize>>` -- a vector of heap-allocated values, each
// remembering at runtime which concrete type (and which impl) it is.
//
// This models a registry CLI printing a mixed list of "things that can be
// summarized" -- packages and registries -- in one loop, the same way a
// C#/TS list of a shared interface type would.
//
// Goal: make this compile AND make the assertions pass, changing as FEW
// lines as possible. Don't delete the assertions.
//
// Run with:
//   cargo run --bin ex3_trait_objects

trait Summarize {
    fn summary(&self) -> String;
}

struct Package {
    name: String,
}

impl Summarize for Package {
    fn summary(&self) -> String {
        format!("package:{}", self.name)
    }
}

struct Registry {
    url: String,
}

impl Summarize for Registry {
    fn summary(&self) -> String {
        format!("registry:{}", self.url)
    }
}

fn main() {
    // BUG: a plain Vec<T> needs ONE concrete element type, but we're
    // pushing a Package and a Registry into the same vec. Change the
    // vec's element type to a trait object so it can hold either,
    // boxed on the heap.
    let items: Vec<Box<dyn Summarize>> = vec![
        Box::new(Package {
            name: "core".to_string(),
        }),
        Box::new(Registry {
            url: "https://reg.example".to_string(),
        }),
    ];

    let summaries: Vec<String> = items.iter().map(|i| i.summary()).collect();

    assert_eq!(
        summaries,
        vec!["package:core", "registry:https://reg.example"]
    );

    println!("ex3 passed");
}
