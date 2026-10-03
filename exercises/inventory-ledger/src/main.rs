use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Receive { item: String, quantity: u32 },
    Sell { item: String, quantity: u32 },
}

type Inventory = BTreeMap<String, u32>;

fn parse_command(line: &str) -> Result<Command, String> {
    todo!("validate and parse a nonblank command")
}

fn apply_command(inventory: &mut Inventory, command: Command) -> Result<(), String> {
    todo!("update stock with checked arithmetic")
}

fn process_ledger(contents: &str) -> Result<Inventory, String> {
    todo!("process commands in order and attach source line numbers to errors")
}

fn format_inventory(inventory: &Inventory) -> String {
    todo!("format the inventory report")
}

fn run(path: &str) -> Result<String, String> {
    todo!("read a ledger file and return its report")
}

fn main() {
    todo!("accept one path, print the report, and exit nonzero on error")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_commands() {
        assert_eq!(
            parse_command("RECEIVE part_2 12"),
            Ok(Command::Receive { item: "part_2".into(), quantity: 12 })
        );
        assert_eq!(
            parse_command("SELL bolts 3"),
            Ok(Command::Sell { item: "bolts".into(), quantity: 3 })
        );
    }

    #[test]
    fn rejects_invalid_commands() {
        for line in [
            "RETURN bolts 1", "SELL bolts", "SELL bolts 1 extra", "SELL bad/item 1",
            "SELL bolts 0", "SELL bolts -1", "SELL bolts nope", "SELL bolts 4294967296",
        ] {
            assert!(parse_command(line).is_err(), "accepted: {line}");
        }
    }

    #[test]
    fn applies_deliveries_and_sales() {
        let mut inventory = Inventory::new();
        apply_command(&mut inventory, Command::Receive { item: "bolts".into(), quantity: 8 }).unwrap();
        apply_command(&mut inventory, Command::Sell { item: "bolts".into(), quantity: 8 }).unwrap();
        assert_eq!(inventory.get("bolts"), Some(&0));
    }

    #[test]
    fn failed_sale_keeps_stock_unchanged() {
        let mut inventory = Inventory::from([("bolts".into(), 2)]);
        assert!(apply_command(&mut inventory, Command::Sell { item: "bolts".into(), quantity: 3 }).is_err());
        assert_eq!(inventory.get("bolts"), Some(&2));
        assert!(apply_command(&mut inventory, Command::Sell { item: "unknown".into(), quantity: 1 }).is_err());
        assert_eq!(inventory.get("unknown"), None);
    }

    #[test]
    fn overflowing_delivery_keeps_stock_unchanged() {
        let mut inventory = Inventory::from([("bolts".into(), u32::MAX)]);
        assert!(apply_command(&mut inventory, Command::Receive { item: "bolts".into(), quantity: 1 }).is_err());
        assert_eq!(inventory.get("bolts"), Some(&u32::MAX));
    }

    #[test]
    fn processes_in_order_and_skips_blank_lines() {
        let inventory = process_ledger("RECEIVE bolts 4\n\n  \nSELL bolts 2\n").unwrap();
        assert_eq!(inventory.get("bolts"), Some(&2));
    }

    #[test]
    fn errors_include_original_line_number() {
        for contents in [
            "RECEIVE bolts 2\n\nSELL bolts 3\n",
            "RECEIVE bolts 2\n\nSELL bolts nope\n",
        ] {
            let error = process_ledger(contents).unwrap_err();
            assert!(error.contains("line 3"), "unexpected error: {error}");
        }
    }

    #[test]
    fn formats_inventory_in_identifier_order() {
        let inventory = Inventory::from([("washers".into(), 4), ("nuts".into(), 0), ("bolts".into(), 5)]);
        assert_eq!(format_inventory(&inventory), "Inventory:\n  bolts: 5\n  nuts: 0\n  washers: 4");
        assert_eq!(format_inventory(&Inventory::new()), "Inventory:");
    }

    #[test]
    fn sample_has_expected_report() {
        let inventory = process_ledger(include_str!("../sample.ledger")).unwrap();
        assert_eq!(format_inventory(&inventory), "Inventory:\n  bolts: 5\n  nuts: 0\n  washers: 4");
    }
}
