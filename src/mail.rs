use std::io::BufRead;

const QUOTE_MARK: &'static str = ">";

pub fn quote<T: BufRead>(reader: T) {
    for line in reader.lines() {
        println!("{}", quote_line(&line.unwrap()));
    }
}

fn quote_line(line: &str) -> String {
    let is_whitespaces = |line: &str| line.chars().all(|c| c == ' ');

    if line.is_empty() || is_whitespaces(line) {
        QUOTE_MARK.to_string()
    } else {
        format!("{} {}", QUOTE_MARK, line)
    }
}
