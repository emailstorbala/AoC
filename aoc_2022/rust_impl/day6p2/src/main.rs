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

fn has_unique_chars(tmp_str: &str) -> bool {
    let mut is_unique: bool = true;
    let mut tmp: String = "".to_string();

    for chr in tmp_str.chars() {
        if tmp.contains(chr) {
            is_unique = false;
            break;
        }
        tmp.push(chr);
    }

    is_unique
}

fn get_unique_char_position(content: String) -> usize {
    let mut chr_pos: usize = 0;

    for idx in 0..content.len() {
        if (idx + DIST_CHAR_SIZE) < content.len() {
            let tmp: String = content[idx..idx+DIST_CHAR_SIZE].to_string();
            if has_unique_chars(&tmp) {
                chr_pos = idx + DIST_CHAR_SIZE;
                break;
            }
        }
    }

    chr_pos
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let content = fs::read_to_string(&args.file_name).expect("Unable to load the file!");

    let res = get_unique_char_position(content);
    println!("Result is {res}");

    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
