use clap::Parser;
use std::time::Instant;
use std::{fs, i64};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

fn is_record_valid(rec: Vec<i64>) -> bool {
    if rec.len() <= 2 {
        // Valid record since one error record is allowed.
        return true;
    } else {
        let mut errored = false;
        for (idx, &num) in rec.iter().enumerate() {
            if idx == 0 {
                let next_num = rec.iter().nth(idx + 1).unwrap();
                let dist = (next_num - num).abs();
                if dist == 0 || dist > 3 {
                    errored = true;
                    break;
                }
            } else if idx == rec.len() - 1 {
                let prev_num = rec.iter().nth(idx - 1).unwrap();
                let dist = (prev_num - num).abs();
                if dist == 0 || dist > 3 {
                    errored = true;
                    break;
                }
            } else {
                // println!("idx is {idx}");
                let prev_num = rec.iter().nth(idx - 1).unwrap();
                let next_num = rec.iter().nth(idx + 1).unwrap();
                // println!("prev_num is {prev_num}");
                // println!("num is {num}");
                // println!("next_num is {next_num}");
                let dist = (prev_num - num).abs();
                // println!("dist: {dist}");

                if dist == 0
                    || dist > 3
                    || (*prev_num < num && *next_num < num)
                    || (*prev_num > num && *next_num > num)
                {
                    errored = true;
                    break;
                }
            }
        }

        !errored
    }
}

fn read_contents(content: String) -> i64 {
    let mut safe_records: i64 = 0;
    for line in content.split('\n') {
        if line.is_empty() {
            break;
        }

        let record: Vec<i64> = line
            .split_whitespace()
            .map(|chunk| -> i64 { chunk.parse::<i64>().unwrap() })
            .collect();

        let mut rec_valid = false;
        for (idx, _) in record.iter().enumerate() {
            let mut tmp_rec: Vec<i64> = record.clone();
            tmp_rec.remove(idx);
            if is_record_valid(tmp_rec) {
                rec_valid = true;
                break;
            }
        }

        if rec_valid {
            safe_records += 1;
        }
    }

    println!("safe_records: {safe_records}");
    safe_records
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let content = fs::read_to_string(&args.file_name).expect("Unable to load the file!");
    let safe_cnt: i64 = read_contents(content);

    println!("safe_cnt is {}", safe_cnt);
    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
