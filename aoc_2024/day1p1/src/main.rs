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

struct Data {
    first_data: Vec<i64>,
    sec_data: Vec<i64>,
}

fn read_contents(content: String) -> Data {
    let mut data: Data = Data {
        first_data: vec![],
        sec_data: vec![],
    };

    for line in content.split('\n') {
        if line.is_empty() {
            break;
        }

        let (first, second) = sscanf::sscanf!(line, "{i64} {i64}").unwrap();
        data.first_data.push(first);
        data.sec_data.push(second);
    }

    data.first_data.sort();
    data.sec_data.sort();
    data
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let content = fs::read_to_string(&args.file_name).expect("Unable to load the file!");
    let data = read_contents(content);

    let input_len = data.first_data.len();
    let mut total_dist: i64 = 0;
    for idx in 0..input_len {
        let first = data.first_data.iter().nth(idx).unwrap();
        let second = data.sec_data.iter().nth(idx).unwrap();
        let dist = if first > second {
            first - second
        } else {
            second - first
        };
        println!("dist is {dist}");
        total_dist += dist;
    }

    println!("Total dist is {total_dist}");
    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
