use clap::Parser;
use std::{fs, i64};
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

fn read_contents(content: String) -> usize {
    let mut records: Vec<Vec<i64>> = Vec::new();
    for line in content.split('\n') {
        if line.is_empty() {
            break;
        }

        let mut tmp_vec: Vec<i64> = Vec::new();
        for num_str in line.split_whitespace() {
            tmp_vec.push(num_str.parse::<i64>().unwrap());
        }

        if tmp_vec.len() <= 2 {
            // Valid record since one error record is allowed.
            records.push(tmp_vec);
        } else {
            let mut error_idx: usize = 0;
            let mut errored = false;
            // println!("tmp_vec -> {:?}", tmp_vec);
            for (idx, &num) in tmp_vec.iter().enumerate() {
                if idx == 0 {
                    let next_num = tmp_vec.iter().nth(idx + 1).unwrap();
                    let dist = (next_num - num).abs();
                    if dist == 0 || dist > 3 {
                        error_idx = idx;
                        errored = true;
                        break;
                    }
                } else if idx == tmp_vec.len() - 1 {
                    let prev_num = tmp_vec.iter().nth(idx - 1).unwrap();
                    let dist = (prev_num - num).abs();
                    if dist == 0 || dist > 3 {
                        error_idx = idx;
                        errored = true;
                        break;
                    }
                } else {
                    // println!("idx is {idx}");
                    // println!("num is {num}");
                    let prev_num = tmp_vec.iter().nth(idx - 1).unwrap();
                    let next_num = tmp_vec.iter().nth(idx + 1).unwrap();
                    let dist = (prev_num - num).abs();

                    if dist == 0 || dist > 3 || (*prev_num < num && *next_num < num) || (*prev_num > num && *next_num > num) {
                        error_idx = idx;
                        errored = true;
                        break;
                    }
                }
            }

            if errored {
                // println!("error_idx: {error_idx}");
                tmp_vec.remove(error_idx);
            }
            records.push(tmp_vec);
        }
    }

    println!("first filter records: {:?}", records);

    let mut safe_records: Vec<Vec<i64>> = Vec::new();
    for new_rec in &records {
        if new_rec.len() <= 2 {
            // Valid record since one error record is allowed.
            safe_records.push(new_rec.clone());
        } else {
            let mut errored = false;
            for (idx, &num) in new_rec.iter().enumerate() {
                if idx == 0 {
                    let next_num = new_rec.iter().nth(idx + 1).unwrap();
                    let dist = (next_num - num).abs();
                    if dist == 0 || dist > 3 {
                        errored = true;
                        break;
                    }
                } else if idx == new_rec.len() - 1 {
                    let prev_num = new_rec.iter().nth(idx - 1).unwrap();
                    let dist = (prev_num - num).abs();
                    if dist == 0 || dist > 3 {
                        errored = true;
                        break;
                    }
                } else {
                    // println!("idx is {idx}");
                    let prev_num = new_rec.iter().nth(idx - 1).unwrap();
                    let next_num = new_rec.iter().nth(idx + 1).unwrap();
                    // println!("prev_num is {prev_num}");
                    // println!("num is {num}");
                    // println!("next_num is {next_num}");
                    let dist = (prev_num - num).abs();
                    // let back_dist = (num - next_num).abs();
                    // println!("dist: {dist}");
                    // println!("back_dist: {back_dist}");

                    if dist == 0 || dist > 3 || (*prev_num < num && *next_num < num) || (*prev_num > num && *next_num > num) {
                        errored = true;
                        break;
                    }
                }
            }

            if !errored {
                safe_records.push(new_rec.clone());
            }
        }
    }

    println!("safe_records: {:?}", safe_records);
    safe_records.len()
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let content = fs::read_to_string(&args.file_name).expect("Unable to load the file!");
    let safe_cnt: usize = read_contents(content);

    println!("safe_cnt is {}", safe_cnt);
    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
