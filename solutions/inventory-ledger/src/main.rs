use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Receive { item: String, quantity: u32 },
    Sell { item: String, quantity: u32 },
}

type Inventory = BTreeMap<String, u32>;

fn parse_command(line: &str) -> Result<Command, String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 3 {
        return Err(format!("expected 3 parts, got {}", parts.len()));
    }
    let action = parts[0].trim().to_uppercase();
    let valid_item_regex = regex::Regex::new(r"^[A-Za-z0-9_-]+$").unwrap();
    let item = parts[1].trim();
    let item_count = parts[2]
        .trim()
        .parse::<u32>()
        .map_err(|_| format!("invalid quantity: {}", parts[2].trim()))?;

    if !valid_item_regex.is_match(item) {
        return Err(format!("invalid item: {item}"));
    }
    if item_count == 0 {
        return Err(format!("quantity must be positive: {item_count}"));
    }

    match action.as_str() {
        "RECEIVE" => Ok(Command::Receive {
            item: item.into(),
            quantity: item_count,
        }),
        "SELL" => Ok(Command::Sell {
            item: item.into(),
            quantity: item_count,
        }),
        _ => Err(format!("unknown command: {}", action)),
    }
}

fn apply_command(inventory: &mut Inventory, command: Command) -> Result<(), String> {
    match command {
        Command::Receive { item, quantity } => {
            let entry = inventory.entry(item).or_insert(0);
            *entry = entry
                .checked_add(quantity)
                .ok_or_else(|| "overflow adding quantity".to_string())?;
            Ok(())
        }
        Command::Sell { item, quantity } => {
            let entry = inventory
                .get_mut(&item)
                .ok_or_else(|| format!("cannot sell {}: item not in inventory", item))?;
            if *entry < quantity {
                return Err(format!("cannot sell {}: only {} in stock", item, *entry));
            }
            *entry -= quantity;
            Ok(())
        }
    }
}

fn process_ledger(contents: &str) -> Result<Inventory, String> {
    let mut inventory = Inventory::new();
    contents
        .split("\n")
        .enumerate()
        .map(|(index, line)| {
            let line = line.trim();
            if line.is_empty() {
                return Ok(());
            }
            let command =
                parse_command(line).map_err(|error| format!("line {}: {}", index + 1, error))?;
            apply_command(&mut inventory, command)
                .map_err(|error| format!("line {}: {}", index + 1, error))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(inventory)
}

fn format_inventory(inventory: &Inventory) -> String {
    let mut output = String::from("Inventory:");
    for (item, quantity) in inventory {
        output.push_str(&format!("\n  {}: {}", item, quantity));
    }
    output
}

fn run(path: &str) -> Result<String, String> {
    let contents = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {}", path, error))?;
    let inventory = process_ledger(&contents)?;
    Ok(format_inventory(&inventory))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <ledger_file>", args[0]);
        std::process::exit(1);
    }
    let path = &args[1];
    match run(path) {
        Ok(report) => println!("{}", report),
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_commands() {
        assert_eq!(
            parse_command("RECEIVE part_2 12"),
            Ok(Command::Receive {
                item: "part_2".into(),
                quantity: 12
            })
        );
        assert_eq!(
            parse_command("SELL bolts 3"),
            Ok(Command::Sell {
                item: "bolts".into(),
                quantity: 3
            })
        );
    }

    #[test]
    fn rejects_invalid_commands() {
        for line in [
            "RETURN bolts 1",
            "SELL bolts",
            "SELL bolts 1 extra",
            "SELL bad/item 1",
            "SELL bolts 0",
            "SELL bolts -1",
            "SELL bolts nope",
            "SELL bolts 4294967296",
        ] {
            assert!(parse_command(line).is_err(), "accepted: {line}");
        }
    }

    #[test]
    fn applies_deliveries_and_sales() {
        let mut inventory = Inventory::new();
        apply_command(
            &mut inventory,
            Command::Receive {
                item: "bolts".into(),
                quantity: 8,
            },
        )
        .unwrap();
        apply_command(
            &mut inventory,
            Command::Sell {
                item: "bolts".into(),
                quantity: 8,
            },
        )
        .unwrap();
        assert_eq!(inventory.get("bolts"), Some(&0));
    }

    #[test]
    fn failed_sale_keeps_stock_unchanged() {
        let mut inventory = Inventory::from([("bolts".into(), 2)]);
        assert!(
            apply_command(
                &mut inventory,
                Command::Sell {
                    item: "bolts".into(),
                    quantity: 3
                }
            )
            .is_err()
        );
        assert_eq!(inventory.get("bolts"), Some(&2));
        assert!(
            apply_command(
                &mut inventory,
                Command::Sell {
                    item: "unknown".into(),
                    quantity: 1
                }
            )
            .is_err()
        );
        assert_eq!(inventory.get("unknown"), None);
    }

    #[test]
    fn overflowing_delivery_keeps_stock_unchanged() {
        let mut inventory = Inventory::from([("bolts".into(), u32::MAX)]);
        assert!(
            apply_command(
                &mut inventory,
                Command::Receive {
                    item: "bolts".into(),
                    quantity: 1
                }
            )
            .is_err()
        );
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
        let inventory = Inventory::from([
            ("washers".into(), 4),
            ("nuts".into(), 0),
            ("bolts".into(), 5),
        ]);
        assert_eq!(
            format_inventory(&inventory),
            "Inventory:\n  bolts: 5\n  nuts: 0\n  washers: 4"
        );
        assert_eq!(format_inventory(&Inventory::new()), "Inventory:");
    }

    #[test]
    fn sample_has_expected_report() {
        let inventory = process_ledger(include_str!("../sample.ledger")).unwrap();
        assert_eq!(
            format_inventory(&inventory),
            "Inventory:\n  bolts: 5\n  nuts: 0\n  washers: 4"
        );
    }
}
