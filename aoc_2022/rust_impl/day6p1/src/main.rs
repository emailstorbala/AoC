use clap::Parser;
use std::fs;
use std::time::Instant;

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

fn get_unique_char_position(content: String) -> i32 {
    let mut chr_pos: i32 = 0;

    for idx in 0..content.len() {
        if idx+4 < content.len() {
            let tmp: String = content[idx..idx+4].to_string();
            if has_unique_chars(&tmp) {
                chr_pos = (idx + 4) as i32;
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
