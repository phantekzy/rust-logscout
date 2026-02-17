# LOGSCOUT: TECHNICAL LOG ANALYSIS ENGINE

A high-performance utility designed to demonstrate safe error handling and zero-copy string processing in Rust.

## Core Technical Concepts

### 1. Robust Error Handling
This project avoids unwrap() in favor of the Result<T, E> pattern. 
- The ? Operator: Used for clean error propagation, allowing the function to exit early if a file operation fails.
- Error Mapping: Utilizes .map_err() to translate standard library std::io::Error types into a custom AnalysisError enum, providing clearer domain-specific feedback.

### 2. Memory-Efficient Parsing
Instead of allocating new memory for every line in a log file, LogScout uses String Slicing:
- Vec<&str>: The analyzer creates a collection of references that point directly into the original file buffer.
- Zero-Copy: This approach ensures that no matter how large the log file is, we are not duplicating the text data unnecessarily.

### 3. Functional Data Processing
The analysis logic leverages Rust iterators:
- .filter(): To isolate specific error patterns.
- .count(): To aggregate data without manual loops.
- .collect(): To transform the lazy iterator into a structured vector.

## Usage

cargo run

## License
MIT - Developed for high-efficiency systems monitoring.
