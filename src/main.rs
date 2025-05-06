use std::env;
use std::io;

mod quote;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        quote::quote(io::stdin().lock());
    } else {
        quote::quote_files(&args[1..]);
    }
}
