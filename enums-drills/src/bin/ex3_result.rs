// Exercise 3: Result<T, E> and the `?` operator
//
// Python/TS/C# use exceptions: a failing call throws, and it silently
// unwinds the stack until something catches it -- you can't tell from a
// function's signature whether it might throw, or what. Rust has no
// exceptions for ordinary errors. A fallible function returns
// `Result<T, E>` (another plain enum: `Ok(T)` or `Err(E)`), so failure is
// visible in the type signature and the compiler forces callers to deal
// with it.
//
// Writing an explicit match on every Result would get noisy fast, so
// Rust has the `?` operator: inside a function that itself returns a
// Result, `some_call()?` means "if this is Ok(v), evaluate to v and keep
// going; if it's Err(e), return Err(e) from the whole function right now."
//
// This models parsing a package version string like "1.4" (from a
// manifest file) in the adico registry crate.
//
// Goal: make this compile AND make the assertions pass, changing as FEW
// lines as possible. Don't delete the assertions. You'll need to use `?`
// at least once instead of writing out full match statements.
//
// Run with:
//   cargo run --bin ex3_result

use std::num::ParseIntError;

struct Version {
    major: u32,
    minor: u32,
}

fn parse_version(raw: &str) -> Result<Version, ParseIntError> {
    let mut parts = raw.split('.');
    let major_str = parts.next().unwrap_or("");
    let minor_str = parts.next().unwrap_or("");

    // BUG: `.parse::<u32>()` returns a Result<u32, ParseIntError>, not a
    // u32 directly. Use `?` so a parse failure short-circuits this
    // function with Err instead of trying to assign a Result where a u32
    // is expected.
    let major: u32 = major_str.parse()?;
    let minor: u32 = minor_str.parse()?;

    Ok(Version { major, minor })
}

fn main() {
    let ok = parse_version("1.4");
    assert!(ok.is_ok());
    let v = ok.unwrap();
    assert_eq!(v.major, 1);
    assert_eq!(v.minor, 4);

    let bad = parse_version("1.oops");
    assert!(bad.is_err());

    println!("ex3 passed");
}
