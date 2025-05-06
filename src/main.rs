use std::env;
use std::io;

mod quote;

fn main() {
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    let mut stderr = io::BufWriter::new(io::stderr().lock());
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        quote::quote(io::stdin().lock(), &mut stdout);
    } else {
        quote::quote_files(&args[1..], &mut stdout, &mut stderr);
    }
}
