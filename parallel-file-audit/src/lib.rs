use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq)]
pub struct FileCounts {
    pub bytes: usize,
    pub lines: usize,
    pub todo_lines: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ScanError {
    Read,
    WorkerPanicked,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ScanResult {
    pub path: PathBuf,
    pub outcome: Result<FileCounts, ScanError>,
}

/// Reads one UTF-8 file and counts its bytes, lines, and lines containing "TODO".
pub fn scan_file(path: &Path) -> Result<FileCounts, ScanError> {
    let _ = path;
    todo!()
}

/// Scans each path on its own thread and returns one result per path in input order.
pub fn scan_files(paths: Vec<PathBuf>) -> Vec<ScanResult> {
    let _ = paths;
    todo!()
}

/// Produces one line per result, in the order supplied by `results`.
pub fn format_report(results: &[ScanResult]) -> String {
    let _ = results;
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> PathBuf {
        Path::new("fixtures").join(name)
    }

    #[test]
    fn scans_a_file_and_counts_each_todo_line_once() {
        assert_eq!(
            scan_file(&fixture("notes.txt")),
            Ok(FileCounts {
                bytes: 33,
                lines: 3,
                todo_lines: 2,
            })
        );
    }

    #[test]
    fn empty_file_has_zero_counts() {
        assert_eq!(
            scan_file(&fixture("empty.txt")),
            Ok(FileCounts {
                bytes: 0,
                lines: 0,
                todo_lines: 0,
            })
        );
    }

    #[test]
    fn missing_and_invalid_utf8_files_are_read_failures() {
        assert_eq!(
            scan_file(Path::new("fixtures/missing.txt")),
            Err(ScanError::Read)
        );
        assert_eq!(
            scan_file(&fixture("invalid-utf8.bin")),
            Err(ScanError::Read)
        );
    }

    #[test]
    fn concurrent_scan_preserves_input_order_and_each_failure() {
        let paths = vec![
            fixture("empty.txt"),
            fixture("missing.txt"),
            fixture("notes.txt"),
        ];
        let results = scan_files(paths.clone());
        assert_eq!(results.len(), 3);
        assert_eq!(
            results.iter().map(|r| &r.path).collect::<Vec<_>>(),
            paths.iter().collect::<Vec<_>>()
        );
        assert_eq!(results[0].outcome.as_ref().unwrap().lines, 0);
        assert_eq!(results[1].outcome, Err(ScanError::Read));
        assert_eq!(results[2].outcome.as_ref().unwrap().todo_lines, 2);
    }

    #[test]
    fn no_paths_produce_no_results() {
        assert!(scan_files(Vec::new()).is_empty());
    }

    #[test]
    fn formatter_handles_success_and_failure_without_scanning() {
        let results = vec![
            ScanResult {
                path: PathBuf::from("notes.txt"),
                outcome: Ok(FileCounts {
                    bytes: 33,
                    lines: 3,
                    todo_lines: 2,
                }),
            },
            ScanResult {
                path: PathBuf::from("missing.txt"),
                outcome: Err(ScanError::Read),
            },
            ScanResult {
                path: PathBuf::from("worker.txt"),
                outcome: Err(ScanError::WorkerPanicked),
            },
        ];
        assert_eq!(
            format_report(&results),
            "notes.txt: 33 bytes, 3 lines, 2 TODO lines\nmissing.txt: read failed\nworker.txt: worker panicked\n"
        );
        assert_eq!(format_report(&[]), "");
    }
}
