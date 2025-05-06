use std::env;
use std::fs;
use std::io;

mod quote;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        quote::quote(io::stdin().lock());
    } else {
        for arg in &args[1..] {
            fs::File::open(arg)
                .map(|file| quote::quote(io::BufReader::new(file)))
                .unwrap_or_else(|error| eprintln!("{}", error));
        }
    }
}
