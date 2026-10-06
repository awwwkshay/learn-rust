use request_head_parser::{format_summary, parse_request_head};

fn main() {
    let input = include_str!("../sample.request");
    let request = parse_request_head(input).expect("sample request should be valid");
    println!("{}", format_summary(&request));
}
