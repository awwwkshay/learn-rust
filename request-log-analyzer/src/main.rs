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
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() != 4 {
        return Err(format!("expected 4 parts, got {}", parts.len()));
    }

    let method = parts[0].trim();
    let path = parts[1].trim();
    let status: u16 = parts[2]
        .trim()
        .parse()
        .map_err(|_| format!("invalid status: {}", parts[2].trim()))?;
    let duration_ms: u32 = parts[3]
        .trim()
        .parse()
        .map_err(|_| format!("invalid duration: {}", parts[3].trim()))?;
    Ok(Request {
        method: method.into(),
        path: path.into(),
        status,
        duration_ms,
    })
}

fn parse_requests(contents: &str) -> Result<Vec<Request>, String> {
    let request_lines = contents.split("\n");
    let requests = request_lines
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(index, line)| {
            parse_request(line.trim()).map_err(|error| format!("line {}: {}", index + 1, error))
        });
    return requests.collect();
}

fn build_report(requests: &[Request]) -> Report {
    let total_reqs = requests.len();
    let mut status_counts = BTreeMap::new();
    let mut slowest: Option<Request> = None;

    for request in requests {
        *status_counts.entry(request.status).or_insert(0) += 1;

        match slowest.as_ref() {
            None => slowest = Some(request.clone()),
            Some(current) if request.duration_ms > current.duration_ms => {
                slowest = Some(request.clone());
            }
            Some(_) => {} // shorter or tied: keep the first
        }
    }

    Report {
        total: total_reqs,
        status_counts,
        slowest,
    }
}

fn format_report(report: &Report) -> String {
    return format!(
        "Requests: {}\nStatus counts:\n{}Slowest: {}",
        report.total,
        if !report.status_counts.is_empty() {
            format!(
                "{}\n",
                report
                    .status_counts
                    .iter()
                    .map(|(status, count)| format!("  {}: {}", status, count))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        } else {
            "".into()
        },
        match &report.slowest {
            Some(request) => format!(
                "{} {} ({}, {} ms)",
                request.method, request.path, request.status, request.duration_ms
            ),
            None => "none".into(),
        }
    );
}

fn run(path: &str) -> Result<String, String> {
    let contents = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read file {}: {}", path, e))?;
    let requests = parse_requests(&contents)?;
    let report = build_report(&requests);
    Ok(format_report(&report))
}

fn main() {
    // TODO: Read one file path from command-line arguments, call run,
    // print the report on success, and print an error on failure.
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <request-log-file>", args[0]);
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
        assert!(parse_request("GET /users 200 nope").is_err());
        assert!(parse_request("GET /users 200").is_err());
        assert!(parse_request("GET /users 200 17 extra").is_err());
    }

    #[test]
    fn parses_file_contents_and_skips_blank_lines() {
        let requests = parse_requests("GET /health 200 3\n\n  \nPOST /jobs 201 42\n").unwrap();
        assert_eq!(requests.len(), 2);
        assert_eq!(requests[0].path, "/health");
        assert_eq!(requests[1].status, 201);
    }

    #[test]
    fn parse_error_includes_source_line_number() {
        let error = parse_requests("GET /health 200 3\n\nGET /jobs nope 42\n").unwrap_err();
        assert!(error.contains("line 3"), "unexpected error: {error}");
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

    #[test]
    fn ties_keep_first_slowest_request() {
        let requests = vec![
            parse_request("GET /first 200 42").unwrap(),
            parse_request("POST /second 201 42").unwrap(),
        ];
        assert_eq!(build_report(&requests).slowest, Some(requests[0].clone()));
    }

    #[test]
    fn formats_report() {
        let report = Report {
            total: 2,
            status_counts: BTreeMap::from([(200, 1), (503, 1)]),
            slowest: Some(Request {
                method: "GET".into(),
                path: "/api/v1/reports/weekly".into(),
                status: 503,
                duration_ms: 1204,
            }),
        };
        assert_eq!(
            format_report(&report),
            "Requests: 2\nStatus counts:\n  200: 1\n  503: 1\nSlowest: GET /api/v1/reports/weekly (503, 1204 ms)"
        );
    }

    #[test]
    fn formats_empty_report() {
        let report = Report {
            total: 0,
            status_counts: BTreeMap::new(),
            slowest: None,
        };
        assert_eq!(
            format_report(&report),
            "Requests: 0\nStatus counts:\nSlowest: none"
        );
    }

    #[test]
    fn sample_log_produces_expected_report() {
        let requests = parse_requests(include_str!("../sample.log")).unwrap();
        let output = format_report(&build_report(&requests));
        assert_eq!(
            output,
            "Requests: 16\nStatus counts:\n  200: 7\n  201: 3\n  204: 1\n  401: 1\n  404: 1\n  429: 1\n  500: 1\n  503: 1\nSlowest: GET /api/v1/reports/weekly (503, 1204 ms)"
        );
    }
}
