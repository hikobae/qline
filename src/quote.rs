use std::fs;
use std::io;
use std::process;

const QUOTE_MARK: &str = ">";
const NEWLINE: &[u8] = b"\n";

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
            Ok(str) => quote_line(&str, stdout, stderr),
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

fn quote_line<T, U>(line: &str, stdout: &mut T, stderr: &mut U)
where
    T: io::Write,
    U: io::Write,
{
    let is_whitespaces = |line: &str| line.is_empty() || line.chars().all(|c| c == ' ');
    let mut write_all = |buf: &[u8]| stdout.write_all(buf).unwrap_or_exit(stderr);

    write_all(QUOTE_MARK.as_bytes());
    if !is_whitespaces(line) {
        if !line.starts_with(QUOTE_MARK) {
            write_all(b" ");
        }
        write_all(line.as_bytes());
    }
    write_all(NEWLINE);
}
