#[derive(Debug, PartialEq, Eq)]
pub struct Header<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RequestHead<'a> {
    pub method: &'a str,
    pub target: &'a str,
    pub version: &'a str,
    pub headers: Vec<Header<'a>>,
}

impl<'a> RequestHead<'a> {
    /// Returns the first matching header value. Header names are ASCII case-insensitive.
    pub fn header_value(&self, name: &str) -> Option<&'a str> {
        todo!()
    }
}

/// Parses a request head while borrowing all field text from `input`.
pub fn parse_request_head<'a>(input: &'a str) -> Result<RequestHead<'a>, String> {
    todo!()
}

/// Formats a compact operator-facing summary without a trailing newline.
pub fn format_summary(request: &RequestHead<'_>) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.request");

    #[test]
    fn parses_request_and_borrows_from_input() {
        let request = parse_request_head(SAMPLE).unwrap();
        assert_eq!(request.method, "GET");
        assert_eq!(request.target, "/orders/42");
        assert_eq!(request.version, "HTTP/1.1");
        assert_eq!(request.headers.len(), 3);
        assert_eq!(
            request.headers[0],
            Header {
                name: "Host",
                value: "example.test"
            }
        );
        assert_eq!(request.headers[1].value, "application/json");
        assert!(std::ptr::eq(request.method.as_ptr(), SAMPLE.as_ptr()));
    }

    #[test]
    fn trims_header_values_and_looks_up_first_match() {
        let input = "POST /jobs HTTP/1.1\nX-Mode:  fast  \nx-mode: slow\nEmpty:\n";
        let request = parse_request_head(input).unwrap();
        assert_eq!(request.headers[0].value, "fast");
        assert_eq!(request.headers[2].value, "");
    }

    #[test]
    fn finds_first_header_without_requiring_a_parser() {
        let request = RequestHead {
            method: "GET",
            target: "/",
            version: "HTTP/1.1",
            headers: vec![
                Header {
                    name: "X-Mode",
                    value: "fast",
                },
                Header {
                    name: "x-mode",
                    value: "slow",
                },
                Header {
                    name: "Empty",
                    value: "",
                },
            ],
        };
        assert_eq!(request.header_value("X-MODE"), Some("fast"));
        assert_eq!(request.header_value("empty"), Some(""));
        assert_eq!(request.header_value("missing"), None);
    }

    #[test]
    fn accepts_request_without_headers() {
        let request = parse_request_head("GET /health HTTP/1.1\n").unwrap();
        assert!(request.headers.is_empty());
        assert_eq!(request.header_value("Host"), None);
    }

    #[test]
    fn rejects_bad_request_lines() {
        for input in [
            "",
            "GET /only-two",
            "GET /x HTTP/1.1 extra",
            "get /x HTTP/1.1",
            "GET x HTTP/1.1",
            "GET /x HTTP/2",
        ] {
            let error = parse_request_head(input).unwrap_err();
            assert!(error.contains("line 1"), "{input:?}: {error}");
        }
    }

    #[test]
    fn rejects_bad_headers_with_source_line() {
        for input in [
            "GET / HTTP/1.1\nNoColon",
            "GET / HTTP/1.1\nBad Name: value",
            "GET / HTTP/1.1\n: value",
        ] {
            let error = parse_request_head(input).unwrap_err();
            assert!(error.contains("line 2"), "{input:?}: {error}");
        }
    }

    #[test]
    fn rejects_content_after_blank_terminator() {
        let error =
            parse_request_head("GET / HTTP/1.1\nHost: example.test\n\nExtra: value\n").unwrap_err();
        assert!(error.contains("line 4"), "{error}");
    }

    #[test]
    fn formats_sample_summary() {
        let request = RequestHead {
            method: "GET",
            target: "/orders/42",
            version: "HTTP/1.1",
            headers: vec![
                Header {
                    name: "Host",
                    value: "example.test",
                },
                Header {
                    name: "Accept",
                    value: "application/json",
                },
                Header {
                    name: "X-Trace",
                    value: "7f2",
                },
            ],
        };
        assert_eq!(
            format_summary(&request),
            "GET /orders/42 (HTTP/1.1)\nHost: example.test\nHeaders: 3"
        );
    }

    #[test]
    fn formats_missing_host() {
        let request = RequestHead {
            method: "GET",
            target: "/health",
            version: "HTTP/1.1",
            headers: vec![],
        };
        assert_eq!(
            format_summary(&request),
            "GET /health (HTTP/1.1)\nHost: none\nHeaders: 0"
        );
    }
}
