use std::fs;

#[derive(Debug)]
enum AnalysisError {
    FileNotFound,
    EmptyLog,
}

fn analyze_logs(filename: &str) -> Result<f32, AnalysisError> {
    let content = fs::read_to_string(filename).map_err(|_| AnalysisError::FileNotFound)?;
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return Err(AnalysisError::EmptyLog);
    }
    let total_logs = lines.len();
    let mut error_count = 0;

    for line in lines {
        if line.to_uppercase().contains("ERROR") {
            error_count += 1;
        }
    }
    let percentage = (error_count as f32 / total_logs as f32) * 100.0;
    Ok(percentage)
}

fn main() {
    let log_data = "INFO: System started\nERROR: Disk full\nINFO: Retrying\nERROR: Timeout";
}
