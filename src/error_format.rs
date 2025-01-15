pub fn formatter(message: &str, line: usize, col: i64) {
    eprintln!("Error {} at line {} col: {}", message, line, col);
}

pub fn format_error(message: &str, line: usize, col: i64) {
    formatter(message, line, col);
}
