use std::fs;
use std::io;
use std::process;

const QUOTE_MARK: &str = ">";

trait ResultExt<T> {
    fn unwrap_or_exit<U: io::Write>(self, stderr: &mut U);
}

impl<T> ResultExt<T> for io::Result<T> {
    fn unwrap_or_exit<U: io::Write>(self, stderr: &mut U) {
        self.unwrap_or_else(|_e| {
            let _ = stderr.write_all("no message".as_bytes());
            let _ = stderr.flush();
            process::exit(1);
        });
    }
}

fn write_error<T>(stderr: &mut T, error: io::Error)
where
    T: io::Write,
{
    stderr
        .write_all(error.to_string().as_bytes())
        .unwrap_or_exit(stderr);
    stderr.flush().unwrap_or_exit(stderr);
}

pub fn quote<R, T, U>(reader: R, stdout: &mut T, stderr: &mut U)
where
    R: io::BufRead,
    T: io::Write,
    U: io::Write,
{
    for line in reader.lines() {
        match line {
            Ok(str) => {
                let quoted = quote_line(&str);
                stdout.write_all(quoted.as_bytes()).unwrap_or_exit(stderr);
                stdout.write_all("\n".as_bytes()).unwrap_or_exit(stderr);
            }
            Err(error) => write_error(stderr, error),
        }
    }
    stdout.flush().unwrap_or_exit(stderr);
}

pub fn quote_files<T, U>(paths: &[String], stdout: &mut T, stderr: &mut U)
where
    T: io::Write,
    U: io::Write,
{
    for arg in paths {
        fs::File::open(arg)
            .map(|file| quote(io::BufReader::new(file), stdout, stderr))
            .unwrap_or_else(|error| write_error(stderr, error));
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
