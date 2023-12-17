use clap::Parser;
use std::fs;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

fn read_contents(content: String) -> i64 {
    let mut clb_total: i64 = 0;
    let mut clb_dtls: Vec<String> = Vec::new();
    for line in content.split('\n') {
        if line.is_empty() {
            break;
        }

        let mut num_str = String::new();

        for chr in line.chars() {
            if chr.is_numeric() {
                num_str.push(chr);
            }
        }

        clb_dtls.push(num_str);
    }

    for num_str in clb_dtls {
        let item_no: i64 = num_str.chars().nth(0).unwrap().to_digit(10).unwrap() as i64 * 10 + num_str.chars().last().unwrap().to_digit(10).unwrap() as i64;
        clb_total += item_no as i64;
    }

    clb_total 
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let content = fs::read_to_string(&args.file_name).expect("Unable to load the file!");

    let clb_total: i64 = read_contents(content);
    println!("The total is {clb_total}");

    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
