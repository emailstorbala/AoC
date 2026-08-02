use clap::Parser;
use std::fs;
use std::time::Instant;

const DIST_CHAR_SIZE: usize = 14;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

fn has_unique_chars(window: &[char]) -> bool {
    let mut seen = std::collections::HashSet::with_capacity(window.len());
    window.iter().all(|chr| seen.insert(chr))
}

fn get_unique_char_position(content: &str) -> Option<usize> {
    let ends_with_newline = content.ends_with('\n');
    let lines: Vec<&str> = content.lines().collect();
    let mut char_offset: usize = 0;

    for (idx, line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        for window_idx in 0..chars.len().saturating_sub(DIST_CHAR_SIZE - 1) {
            let window = &chars[window_idx..window_idx + DIST_CHAR_SIZE];
            if has_unique_chars(window) {
                return Some(char_offset + window_idx + DIST_CHAR_SIZE);
            }
        }

        let has_separator = idx + 1 < lines.len() || ends_with_newline;
        char_offset += chars.len() + usize::from(has_separator);
    }

    None
}

fn main() {
    let start_time = Instant::now();
    let args = Args::parse();
    let content = match fs::read_to_string(&args.file_name) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Unable to load the file '{}': {err}", args.file_name);
            std::process::exit(1);
        }
    };

    match get_unique_char_position(&content) {
        Some(res) => println!("Result is {res}"),
        None => println!("No unique window of size {DIST_CHAR_SIZE} found"),
    }

    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
