use equipment_desk::EquipmentDesk;

fn main() {
    let mut desk = EquipmentDesk::new();
    desk.add_tool("camera", 1).expect("valid tool");

    for borrower in ["Ada", "Mira", "Leo"] {
        println!(
            "{borrower}: {:?}",
            desk.checkout("camera", borrower).expect("valid checkout")
        );
    }

    println!(
        "added copies: {:?}",
        desk.add_copies("camera", 1).expect("valid addition")
    );
    println!(
        "Ada returns: {:?}",
        desk.return_tool("camera", "Ada").expect("valid return")
    );

    let status = desk.status("camera").expect("known tool");
    println!(
        "camera: {} available; borrowers {:?}; waiting {:?}",
        status.available, status.borrowers, status.waitlist
    );
}
