use std::fs;

#[derive(Debug)]
enum AnalysisError {
    FileNotFound,
    EmptyLog,
}

fn analyze_logs(filename: &str) -> Result<f32, AnalysisError> {
    let content = fs::read_to_string(filename).map_err(|_| AnalysisError::FileNotFound)?;
}

fn main() {
    //
}
