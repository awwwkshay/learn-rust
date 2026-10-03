// Exercise 2: Borrowing (&T vs &mut T)
//
// In Python/TS/C#, passing an object to a function always hands over a
// reference and you can read AND mutate through it, no questions asked.
// Rust distinguishes "I just want to look" (&T) from "I want to mutate"
// (&mut T), and enforces at compile time that you can't have a mutable
// borrow alive at the same time as any other borrow.
//
// This models a piece of `RegistryCatalog::total_size` from the adico
// registry crate: it sums up sizes without needing ownership of the list.
//
// Goal: make this compile AND make the assertions pass, changing as FEW
// lines/signatures as possible. Don't delete the assertions.
//
// Run with:
//   cargo run --bin ex2_borrow

struct Package {
    name: String,
    size_kb: u32,
}

struct Catalog {
    packages: Vec<Package>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { packages: Vec::new() }
    }

    fn add(&mut self, name: &str, size_kb: u32) {
        self.packages.push(Package { name: name.to_string(), size_kb });
    }

    // BUG: this takes ownership of `catalog`, so it can't be called twice
    // and the caller loses `catalog` afterwards. Fix the signature (and
    // anything else needed) so it only *borrows* what it needs.
    fn total_size(catalog: &Catalog) -> u32 {
        catalog.packages.iter().map(|p| p.size_kb).sum()
    }
}

fn main() {
    let mut catalog = Catalog::new();
    catalog.add("core", 120);
    catalog.add("cli", 340);

    let first_total = Catalog::total_size(&catalog);
    // We call it again on the SAME catalog binding below -- this only
    // works once ownership isn't being taken.
    let second_total = Catalog::total_size(&catalog);

    assert_eq!(first_total, 460);
    assert_eq!(second_total, 460);

    println!("ex2 passed");
}
