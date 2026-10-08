use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Debug, PartialEq, Eq)]
pub enum DeskError {
    InvalidToolName,
    InvalidBorrower,
    InvalidCopies,
    ToolExists(String),
    UnknownTool(String),
    AlreadyRegistered(String),
    NotBorrowed(String),
    NotWaiting(String),
    CapacityOverflow(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum CheckoutOutcome {
    Loaned,
    Waitlisted { position: usize },
}

#[derive(Debug, PartialEq, Eq)]
pub enum ReturnOutcome {
    Available,
    AssignedTo(String),
}

/// An owned snapshot. Borrowers are sorted by name; the waitlist stays in FIFO order.
#[derive(Debug, PartialEq, Eq)]
pub struct ToolStatus {
    pub total_copies: u32,
    pub available: u32,
    pub borrowers: Vec<String>,
    pub waitlist: Vec<String>,
}

struct ToolRecord {
    total_copies: u32,
    borrowers: BTreeSet<String>,
    waitlist: VecDeque<String>,
}

/// Tracks loans and waitlists for independent kinds of equipment.
pub struct EquipmentDesk {
    tools: BTreeMap<String, ToolRecord>,
}

impl EquipmentDesk {
    pub fn new() -> Self {
        Self {
            tools: BTreeMap::new(),
        }
    }

    /// Registers a new tool with a positive number of copies.
    pub fn add_tool(&mut self, name: &str, copies: u32) -> Result<(), DeskError> {
        todo!()
    }

    /// Loans a copy immediately, or places the borrower at the end of the waitlist.
    pub fn checkout(&mut self, tool: &str, borrower: &str) -> Result<CheckoutOutcome, DeskError> {
        todo!()
    }

    /// Returns a loan and immediately assigns its copy to the first waiting borrower, if any.
    pub fn return_tool(&mut self, tool: &str, borrower: &str) -> Result<ReturnOutcome, DeskError> {
        todo!()
    }

    /// Removes a borrower from this tool's waitlist without changing any loans.
    pub fn cancel_wait(&mut self, tool: &str, borrower: &str) -> Result<(), DeskError> {
        todo!()
    }

    /// Adds copies and immediately assigns as many waiting borrowers as possible.
    /// Returns their names in assignment order.
    pub fn add_copies(&mut self, tool: &str, copies: u32) -> Result<Vec<String>, DeskError> {
        todo!()
    }

    /// Returns an owned snapshot; querying does not change the desk.
    pub fn status(&self, tool: &str) -> Result<ToolStatus, DeskError> {
        todo!()
    }
}

impl Default for EquipmentDesk {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Direct setup keeps each operation test useful before the other TODOs are done.
    fn desk_with(tool: &str, copies: u32, borrowers: &[&str], waitlist: &[&str]) -> EquipmentDesk {
        let mut desk = EquipmentDesk::new();
        desk.tools.insert(
            tool.to_string(),
            ToolRecord {
                total_copies: copies,
                borrowers: borrowers.iter().map(|name| name.to_string()).collect(),
                waitlist: waitlist.iter().map(|name| name.to_string()).collect(),
            },
        );
        desk
    }

    #[test]
    fn registers_tools_and_rejects_bad_registration_without_changing_state() {
        let mut desk = EquipmentDesk::new();
        assert_eq!(desk.add_tool("camera", 2), Ok(()));
        assert_eq!(
            desk.add_tool("camera", 1),
            Err(DeskError::ToolExists("camera".into()))
        );
        assert_eq!(desk.add_tool("  ", 1), Err(DeskError::InvalidToolName));
        assert_eq!(desk.add_tool("tripod", 0), Err(DeskError::InvalidCopies));
        assert_eq!(desk.tools.len(), 1);
        assert_eq!(desk.tools["camera"].total_copies, 2);
    }

    #[test]
    fn checkout_loans_then_waitlists_in_arrival_order() {
        let mut desk = desk_with("camera", 1, &[], &[]);
        assert_eq!(desk.checkout("camera", "Ada"), Ok(CheckoutOutcome::Loaned));
        assert_eq!(
            desk.checkout("camera", "Mira"),
            Ok(CheckoutOutcome::Waitlisted { position: 1 })
        );
        assert_eq!(
            desk.checkout("camera", "Leo"),
            Ok(CheckoutOutcome::Waitlisted { position: 2 })
        );
        let record = &desk.tools["camera"];
        assert!(record.borrowers.contains("Ada"));
        assert_eq!(
            record
                .waitlist
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            vec!["Mira", "Leo"]
        );
    }

    #[test]
    fn duplicate_or_invalid_checkout_does_not_change_the_queue() {
        let mut desk = desk_with("camera", 1, &["Ada"], &["Mira"]);
        assert_eq!(
            desk.checkout("camera", "Ada"),
            Err(DeskError::AlreadyRegistered("Ada".into()))
        );
        assert_eq!(
            desk.checkout("camera", "Mira"),
            Err(DeskError::AlreadyRegistered("Mira".into()))
        );
        assert_eq!(
            desk.checkout("camera", " "),
            Err(DeskError::InvalidBorrower)
        );
        assert_eq!(
            desk.checkout("missing", "Leo"),
            Err(DeskError::UnknownTool("missing".into()))
        );
        assert_eq!(desk.tools["camera"].waitlist.len(), 1);
        assert_eq!(desk.tools["camera"].borrowers.len(), 1);
    }

    #[test]
    fn return_transfers_a_copy_to_the_first_waiting_borrower() {
        let mut desk = desk_with("camera", 1, &["Ada"], &["Mira", "Leo"]);
        assert_eq!(
            desk.return_tool("camera", "Ada"),
            Ok(ReturnOutcome::AssignedTo("Mira".into()))
        );
        let record = &desk.tools["camera"];
        assert_eq!(
            record
                .borrowers
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            vec!["Mira"]
        );
        assert_eq!(
            record
                .waitlist
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            vec!["Leo"]
        );
    }

    #[test]
    fn return_without_waiters_frees_a_copy_and_bad_return_is_atomic() {
        let mut desk = desk_with("camera", 1, &["Ada"], &[]);
        assert_eq!(
            desk.return_tool("camera", "Leo"),
            Err(DeskError::NotBorrowed("Leo".into()))
        );
        assert!(desk.tools["camera"].borrowers.contains("Ada"));
        assert_eq!(
            desk.return_tool("camera", "Ada"),
            Ok(ReturnOutcome::Available)
        );
        assert!(desk.tools["camera"].borrowers.is_empty());
    }

    #[test]
    fn cancel_wait_removes_a_middle_entry_without_disturbing_order() {
        let mut desk = desk_with("camera", 1, &["Ada"], &["Mira", "Leo", "Nia"]);
        assert_eq!(desk.cancel_wait("camera", "Leo"), Ok(()));
        assert_eq!(
            desk.cancel_wait("camera", "Leo"),
            Err(DeskError::NotWaiting("Leo".into()))
        );
        assert_eq!(
            desk.cancel_wait("camera", "Ada"),
            Err(DeskError::NotWaiting("Ada".into()))
        );
        assert_eq!(
            desk.tools["camera"]
                .waitlist
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            vec!["Mira", "Nia"]
        );
    }

    #[test]
    fn return_and_cancel_reject_bad_requests_without_changes() {
        let mut desk = desk_with("camera", 1, &["Ada"], &["Mira"]);
        assert_eq!(
            desk.return_tool("missing", "Ada"),
            Err(DeskError::UnknownTool("missing".into()))
        );
        assert_eq!(
            desk.return_tool("camera", "  "),
            Err(DeskError::InvalidBorrower)
        );
        assert_eq!(
            desk.cancel_wait("camera", "  "),
            Err(DeskError::InvalidBorrower)
        );
        assert_eq!(
            desk.cancel_wait("missing", "Mira"),
            Err(DeskError::UnknownTool("missing".into()))
        );
        assert!(desk.tools["camera"].borrowers.contains("Ada"));
        assert_eq!(
            desk.tools["camera"].waitlist.front().map(String::as_str),
            Some("Mira")
        );
    }

    #[test]
    fn extra_copies_fill_the_waitlist_before_becoming_available() {
        let mut desk = desk_with("camera", 1, &["Ada"], &["Mira", "Leo", "Nia"]);
        assert_eq!(
            desk.add_copies("camera", 2),
            Ok(vec!["Mira".into(), "Leo".into()])
        );
        assert_eq!(desk.tools["camera"].total_copies, 3);
        assert_eq!(desk.tools["camera"].borrowers.len(), 3);
        assert_eq!(
            desk.tools["camera"]
                .waitlist
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            vec!["Nia"]
        );
        assert_eq!(desk.add_copies("camera", 2), Ok(vec!["Nia".into()]));
        assert_eq!(desk.tools["camera"].total_copies, 5);
        assert!(desk.tools["camera"].waitlist.is_empty());
    }

    #[test]
    fn invalid_or_overflowing_stock_change_is_atomic() {
        let mut desk = desk_with("camera", u32::MAX, &[], &[]);
        assert_eq!(
            desk.add_copies("camera", 1),
            Err(DeskError::CapacityOverflow("camera".into()))
        );
        assert_eq!(desk.add_copies("camera", 0), Err(DeskError::InvalidCopies));
        assert_eq!(desk.tools["camera"].total_copies, u32::MAX);
        assert_eq!(
            desk.add_copies("missing", 1),
            Err(DeskError::UnknownTool("missing".into()))
        );
    }

    #[test]
    fn status_is_a_sorted_owned_snapshot() {
        let mut desk = desk_with("camera", 2, &["Zoe", "Ada"], &["Mira", "Leo"]);
        let snapshot = desk.status("camera").unwrap();
        assert_eq!(
            snapshot,
            ToolStatus {
                total_copies: 2,
                available: 0,
                borrowers: vec!["Ada".into(), "Zoe".into()],
                waitlist: vec!["Mira".into(), "Leo".into()],
            }
        );
        assert_eq!(
            desk.status("missing"),
            Err(DeskError::UnknownTool("missing".into()))
        );
        desk.tools.get_mut("camera").unwrap().borrowers.clear();
        assert_eq!(snapshot.borrowers, vec!["Ada", "Zoe"]);
    }

    #[test]
    fn complete_workflow_keeps_tools_independent() {
        let mut desk = EquipmentDesk::new();
        desk.add_tool("camera", 1).unwrap();
        desk.add_tool("tripod", 2).unwrap();
        assert_eq!(desk.checkout("camera", "Ada"), Ok(CheckoutOutcome::Loaned));
        assert_eq!(
            desk.checkout("camera", "Mira"),
            Ok(CheckoutOutcome::Waitlisted { position: 1 })
        );
        assert_eq!(desk.checkout("tripod", "Mira"), Ok(CheckoutOutcome::Loaned));
        assert_eq!(
            desk.return_tool("camera", "Ada"),
            Ok(ReturnOutcome::AssignedTo("Mira".into()))
        );
        assert_eq!(desk.status("camera").unwrap().borrowers, vec!["Mira"]);
        assert_eq!(desk.status("tripod").unwrap().available, 1);
    }
}
