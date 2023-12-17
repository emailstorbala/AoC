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

        println!("num_str is {num_str}");
        clb_dtls.push(num_str);
    }

    for mut num_str in clb_dtls {
        if num_str.len() == 2 {
            clb_total += num_str.parse::<i64>().unwrap();
        } else if num_str.len() == 1 {
            num_str.push(num_str.chars().nth(0).unwrap());
            clb_total += num_str.parse::<i64>().unwrap();
        } else if num_str.len() > 2 {
            let mut tmp_str = String::new();
            tmp_str.push(num_str.chars().nth(0).unwrap());
            tmp_str.push(num_str.chars().last().unwrap());
            clb_total += tmp_str.parse::<i64>().unwrap();
        }
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
