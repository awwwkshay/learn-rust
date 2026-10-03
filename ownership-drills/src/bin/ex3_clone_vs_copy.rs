// Exercise 3: Clone vs Copy
//
// In C#, value types (struct, int) are copied by default; reference types
// are shared. In Python/TS, everything non-primitive is a shared reference
// and "copying" is something you opt into explicitly (copy.deepcopy,
// structuredClone). Rust makes the distinction explicit via two traits:
//   - Copy: bitwise-copied implicitly on assignment/pass (only for simple,
//     stack-only data like integers, bools, and structs made entirely of
//     Copy types).
//   - Clone: an explicit, possibly expensive `.clone()` call you opt into
//     (for anything holding heap data, like String or Vec).
//
// This models version tags in the adico registry: a cheap numeric Version
// that should be Copy, and a Manifest (holding a String) that should not
// be -- it must be explicitly cloned.
//
// Goal: make this compile AND make the assertions pass, changing as FEW
// lines as possible. Don't delete the assertions.
//
// Run with:
//   cargo run --bin ex3_clone_vs_copy

#[derive(Debug, PartialEq, Clone)]
struct Version {
    major: u32,
    minor: u32,
}
// TODO: Version is just two u32s -- derive whatever traits let it be
// implicitly copied on assignment instead of moved.

#[derive(Debug, Clone, PartialEq)]
struct Manifest {
    namespace: String,
    version: Version,
}

fn describe(v: &Version) -> String {
    format!("{}.{}", v.major, v.minor)
}

fn main() {
    let v = Version { major: 1, minor: 4 };

    // BUG: without Copy, `v` is moved into `describe`, so using `v` again
    // below fails to compile.
    let label = describe(&v);
    assert_eq!(v, Version { major: 1, minor: 4 });
    assert_eq!(label, "1.4");

    let manifest = Manifest {
        namespace: "core".to_string(),
        version: v,
    };
    let manifest_copy = manifest.clone();

    assert_eq!(manifest, manifest_copy);
    assert_eq!(manifest.namespace, "core");

    println!("ex3 passed");
}
