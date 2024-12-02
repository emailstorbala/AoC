use clap::Parser;
use std::{fs, i64};
use sscanf;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

struct Data {
    first_data: Vec<i64>,
    sec_data: Vec<i64>
}

fn read_contents(content: String) -> Data {
    let mut data: Data = Data {
        first_data: vec![],
        sec_data: vec![]
    };

    for line in content.split('\n') {
        if line.is_empty() {
            break;
        }

        let (first, second) = sscanf::sscanf!(line, "{i64} {i64}").unwrap();
        data.first_data.push(first);
        data.sec_data.push(second);
    }

    data
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let content = fs::read_to_string(&args.file_name).expect("Unable to load the file!");
    let data = read_contents(content);

    let mut total_dist: i64 = 0;
    for first in data.first_data {
        let mut cnt = 0;
        for second in &data.sec_data {
            if first == *second {
                cnt += 1;
            }
        }

        total_dist += first * cnt;
        // println!("first is {first}");
        // println!("cnt is {cnt}");
        // println!("tot dist: {total_dist}");
    }

    println!("Total dist is {total_dist}");
    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
