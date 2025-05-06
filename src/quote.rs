use std::fs;
use std::io;

const QUOTE_MARK: &str = ">";

pub fn quote<R, T>(reader: R, stdout: &mut T)
where
    R: io::BufRead,
    T: io::Write,
{
    for line in reader.lines() {
        stdout
            .write_all(quote_line(&line.unwrap()).as_bytes())
            .unwrap();
        stdout.write_all("\n".as_bytes()).unwrap();
    }
    stdout.flush().unwrap();
}

pub fn quote_files<T, U>(paths: &[String], stdout: &mut T, stderr: &mut U)
where
    T: io::Write,
    U: io::Write,
{
    for arg in paths {
        fs::File::open(arg)
            .map(|file| quote(io::BufReader::new(file), stdout))
            .unwrap_or_else(|error| {
                stderr.write_all(error.to_string().as_bytes()).unwrap();
                stderr.flush().unwrap();
            });
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
