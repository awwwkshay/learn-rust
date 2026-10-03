use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Request {
    method: String,
    path: String,
    status: u16,
    duration_ms: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct Report {
    total: usize,
    status_counts: BTreeMap<u16, usize>,
    slowest: Option<Request>,
}

// Each log line has: METHOD PATH STATUS DURATION_MS
fn parse_request(line: &str) -> Result<Request, String> {
    todo!("parse and validate one request line")
}

fn build_report(requests: &[Request]) -> Report {
    todo!("count statuses and find the slowest request")
}

fn format_report(report: &Report) -> String {
    todo!("produce a readable report")
}

fn run(path: &str) -> Result<String, String> {
    todo!("read the file, parse each line, and return the report")
}

fn main() {
    // TODO: Read one file path from command-line arguments, call run,
    // print the report on success, and print an error on failure.
    todo!("connect the command-line interface")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_request() {
        assert_eq!(
            parse_request("GET /users 200 17"),
            Ok(Request {
                method: "GET".into(),
                path: "/users".into(),
                status: 200,
                duration_ms: 17,
            })
        );
    }

    #[test]
    fn rejects_bad_request() {
        assert!(parse_request("GET /users nope 17").is_err());
        assert!(parse_request("GET /users 200").is_err());
        assert!(parse_request("GET /users 200 17 extra").is_err());
    }

    #[test]
    fn summarizes_requests() {
        let requests = vec![
            parse_request("GET /users 200 17").unwrap(),
            parse_request("POST /users 201 42").unwrap(),
            parse_request("GET /missing 404 5").unwrap(),
            parse_request("GET /users 200 9").unwrap(),
        ];
        let report = build_report(&requests);
        assert_eq!(report.total, 4);
        assert_eq!(report.status_counts.get(&200), Some(&2));
        assert_eq!(report.status_counts.get(&201), Some(&1));
        assert_eq!(report.status_counts.get(&404), Some(&1));
        assert_eq!(report.slowest, Some(requests[1].clone()));
    }

    #[test]
    fn empty_report() {
        let report = build_report(&[]);
        assert_eq!(report.total, 0);
        assert!(report.status_counts.is_empty());
        assert_eq!(report.slowest, None);
    }
}
