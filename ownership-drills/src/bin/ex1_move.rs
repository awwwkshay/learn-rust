// Exercise 1: Move semantics
//
// This models `RegistryCatalog::insert` from the adico registry crate,
// which does roughly:
//     let namespace = registry.manifest.namespace.clone();
//     ... use namespace ...
//     self.registries.insert(namespace, registry);  // moves `registry`
//
// Goal: make this compile AND make the assertions at the bottom pass,
// changing as FEW lines as possible. Don't delete the assertions.
//
// Run with:
//   cargo run --bin ex1_move

#[derive(Debug, Clone)]
struct Item {
    name: String,
}

struct Catalog {
    items: Vec<Item>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: Vec::new() }
    }

    fn insert(&mut self, item: &Item) -> String {
        // We want to return the item's name AFTER moving `item` into
        // self.items. Right now this is written in the wrong order / with
        // the wrong ownership and won't compile. Fix it.
        self.items.push(item.clone());
        item.name.clone()
    }
}

fn main() {
    let mut catalog = Catalog::new();

    let returned_name = catalog.insert(&Item {
        name: String::from("button"),
    });

    assert_eq!(returned_name, "button");
    assert_eq!(catalog.items.len(), 1);
    assert_eq!(catalog.items[0].name, "button");

    println!("ex1 passed");
}
