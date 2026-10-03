// Exercise 2: Option<T>
//
// Python has `None`, TS has `null`/`undefined`, C# has `null` (and
// nullable value types via `int?`) -- in all three, forgetting to check
// for the "nothing" case is a runtime crash (NullPointerException /
// NullReferenceException / "Cannot read properties of undefined").
// Rust has no null. Instead, anything that might be absent is wrapped in
// `Option<T>`, which is just an enum: `Some(T)` or `None`. Because match
// (and every other way of getting at the inner value) must be exhaustive,
// the "what if it's missing" case is checked at compile time, not
// discovered in production.
//
// This models looking up a package's registry mirror override, from the
// adico registry crate's config resolution.
//
// Goal: make this compile AND make the assertions pass, changing as FEW
// lines as possible. Don't delete the assertions.
//
// Run with:
//   cargo run --bin ex2_option

struct Config {
    mirror_override: Option<String>,
}

fn resolve_mirror(config: &Config, default: &str) -> String {
    // BUG: this treats `mirror_override` as if it were already a String.
    // It's an Option<String> -- use pattern matching (or an Option method
    // like `.unwrap_or_else` / `.as_deref`) to handle both the Some and
    // None cases instead of assuming it's always present.
    match &config.mirror_override {
        Some(mirror) => mirror.clone(),
        None => default.to_string(),
    }
}

fn main() {
    let with_override = Config {
        mirror_override: Some("https://mirror.example".to_string()),
    };
    let without_override = Config {
        mirror_override: None,
    };

    assert_eq!(
        resolve_mirror(&with_override, "https://default.example"),
        "https://mirror.example"
    );
    assert_eq!(
        resolve_mirror(&without_override, "https://default.example"),
        "https://default.example"
    );

    println!("ex2 passed");
}
