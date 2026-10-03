// Exercise 1: Enums + exhaustive match
//
// In C#, `enum` is just named integers. In TS, a union type like
// `type Shape = Circle | Square` is checked at edit-time by the editor,
// not enforced by the runtime. In Rust, `enum` variants can each carry
// their own data, and `match` forces you to handle every variant --
// leaving one out is a compile error, not a bug you find in production.
//
// This models registry events from the adico registry crate: something
// happened to a package, and callers need to react differently per event.
//
// Goal: make this compile AND make the assertions pass, changing as FEW
// lines as possible. Don't delete the assertions.
//
// Run with:
//   cargo run --bin ex1_match

enum RegistryEvent {
    Published { name: String, version: u32 },
    Yanked(String),
    Down,
}

fn describe(event: &RegistryEvent) -> String {
    // BUG: this match doesn't handle every variant of RegistryEvent, so it
    // won't compile. Add the missing arm(s). Note the field/tuple binding
    // syntax differs per variant shape.
    match event {
        RegistryEvent::Published { name, version } => format!("{name} v{version} published"),
        RegistryEvent::Yanked(name) => format!("{name} yanked"),
        _ => "registry unreachable".to_string(),
    }
}

fn main() {
    let published = RegistryEvent::Published {
        name: "core".to_string(),
        version: 3,
    };
    let yanked = RegistryEvent::Yanked("cli".to_string());
    let down = RegistryEvent::Down;

    assert_eq!(describe(&published), "core v3 published");
    assert_eq!(describe(&yanked), "cli yanked");
    assert_eq!(describe(&down), "registry unreachable");

    println!("ex1 passed");
}
