use clap::Parser;
use sscanf;
use std::time::Instant;
use std::{fs, i64};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

fn read_contents(content: String) -> i64 {
    let mut result: i64 = 0;
    let mut do_mul = true;
    for line in content.split('\n') {
        if line.is_empty() {
            break;
        }

        println!("line is {line}");
        for idx in 0..line.len() {
            let rem_str = &line[idx..];
            // println!("rem_str is {rem_str}");
            if rem_str.starts_with("do()") {
                println!("Enabling multiply");
                do_mul = true;
            } else if rem_str.starts_with("don't()") {
                println!("Disabling multiply");
                do_mul = false;
            } else if do_mul {
                let data = format!("{rem_str}dummy");
                // println!("data is {data}");
                let parsed = sscanf::sscanf!(data, "mul({i64},{i64}){&str}");
                match parsed {
                    Ok((first, second, _)) => {
                        println!("first, second -> {first}, {second}");
                        result += first * second;
                    }
                    Err(_) => {
                        // Ignoring error
                    }
                }
            }
        }

    }

    result
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let content = fs::read_to_string(&args.file_name).expect("Unable to load the file!");
    let res = read_contents(content);
    println!("The result is {res}");
    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
