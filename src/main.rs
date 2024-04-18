use clap::builder::Str;
use clap::{Error, Parser};
use std::io::{BufRead, BufReader, Cursor};
use std::{fs::File, path::PathBuf};

#[derive(Parser)]
struct Args {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: PathBuf
}

fn find_matches<R: BufRead>(reader: &mut R, pattern: &str, mut writer: impl std::io::Write) {
    let mut line = String::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).unwrap();
        }
        line.clear();
    }
}



fn main() -> Result<(), Error>{
    let args = Args::parse();
    let f = File::open(args.path).expect("Invalid file path");
    let mut reader = BufReader::new(f);

    find_matches(&mut reader, &args.pattern, &mut std::io::stdout());
    
    Ok(())
}

#[test]
fn test_find_matches() {
        let input_data = b"apple\nbanana\norange\napple pie\n";
        let mut reader = BufReader::new(Cursor::new(input_data));
        let mut output = Vec::new();
        find_matches(&mut reader, "apple", &mut output);

        let expected_output = b"apple\n\napple pie\n\n";
        assert_eq!(output, expected_output);
}