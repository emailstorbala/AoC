use clap::Parser;
use std::time::Instant;
use std::{fs, i64};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

fn read_contents(content: String) -> Vec<Vec<i64>> {
    let mut data: Vec<Vec<i64>> = Vec::new();
    for line in content.split('\n') {
        if line.is_empty() {
            break;
        }

        let mut tmp_vec: Vec<i64> = Vec::new();
        for num_str in line.split_whitespace() {
            tmp_vec.push(num_str.parse::<i64>().unwrap());
        }
        data.push(tmp_vec);
    }

    data
}

fn is_ascending(tmp_data: Vec<i64>) -> bool {
    return tmp_data.first() < tmp_data.last();
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let content = fs::read_to_string(&args.file_name).expect("Unable to load the file!");
    let data: Vec<Vec<i64>> = read_contents(content);
    let mut safe_cnt = 0;

    for tmp_data in data {
        let mut safe: bool = true;
        if is_ascending(tmp_data.clone()) {
            let mut prev_num = -1;
            for num in &tmp_data {
                if prev_num == -1 {
                    prev_num = *num;
                    continue;
                }

                // println!("prev_num, num -> {prev_num}, {num}");
                if *num == prev_num || *num < prev_num || (*num - prev_num) > 3 {
                    safe = false;
                    break;
                }
                prev_num = *num;
            }
        } else {
            let mut prev_num = -1;
            for num in &tmp_data {
                if prev_num == -1 {
                    prev_num = *num;
                    continue;
                }

                // println!("prev_num, num -> {prev_num}, {num}");
                if *num == prev_num || *num > prev_num || (prev_num - *num) > 3 {
                    safe = false;
                    break;
                }
                prev_num = *num;
            }
        }

        if safe {
            safe_cnt += 1;
        }
    }

    println!("safe_cnt is {safe_cnt}");
    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
