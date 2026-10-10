use parallel_file_audit::{format_report, scan_files};
use std::path::PathBuf;

fn main() {
    let paths: Vec<PathBuf> = std::env::args_os().skip(1).map(PathBuf::from).collect();
    if paths.is_empty() {
        eprintln!("usage: parallel-file-audit <file> [file ...]");
        std::process::exit(2);
    }

    print!("{}", format_report(&scan_files(paths)));
}
