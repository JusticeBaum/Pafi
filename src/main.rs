use clap::Parser;
use std::io::{BufRead, BufReader};
use std::{fs::File, path::PathBuf};

#[derive(Parser)]
struct Args {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: PathBuf
}

fn main() {
    let args = Args::parse();
    let f = File::open(args.path).expect("Invalid file path");
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        if line.contains(&args.pattern) {
            println!("{}", line);
            return
        }
        line.clear();
    }
}
