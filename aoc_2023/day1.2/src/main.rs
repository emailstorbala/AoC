use clap::Parser;
use std::{fs};
use std::time::Instant;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

fn read_contents(content: String) -> i64 {
    let mut clb_total: i64 = 0;
    let mut content_details: Vec<String> = Vec::new();

    for line in content.split('\n') {
        if line.is_empty() { break };

        let mut mod_line: String = line.to_string();
        let mut start_cnt: usize = 0;
        while start_cnt+5 <= mod_line.len() {
            println!("start_cnt->{start_cnt}");
            println!("mod_line is {mod_line}");
            let chunk: String = mod_line[start_cnt..].to_string();
            println!("chunk val->{chunk}");
            let zero_re = Regex::new(r"^zero.*").unwrap();
            let one_re = Regex::new(r"^one.*").unwrap();
            let two_re = Regex::new(r"^two.*").unwrap();
            let three_re = Regex::new(r"^three.*").unwrap();
            let four_re = Regex::new(r"^four.*").unwrap();
            let five_re = Regex::new(r"^five.*").unwrap();
            let six_re = Regex::new(r"^six.*").unwrap();
            let seven_re = Regex::new(r"^seven.*").unwrap();
            let eight_re = Regex::new(r"^eight.*").unwrap();
            let nine_re = Regex::new(r"^nine.*").unwrap();

            if one_re.is_match(chunk.as_str()) {
                mod_line = mod_line.replacen("one", "1", 1);
                continue;
            }
            if two_re.is_match(chunk.as_str()) {
                mod_line = mod_line.replacen("two", "2", 1);
                continue;
            }
            if three_re.is_match(chunk.as_str()) {
                mod_line = mod_line.replacen("three", "3", 1);
                continue;
            }
            if four_re.is_match(chunk.as_str()) {
                mod_line = mod_line.replacen("four", "4", 1);
                continue;
            }
            if five_re.is_match(chunk.as_str()) {
                mod_line = mod_line.replacen("five", "5", 1);
                continue;
            }
            if six_re.is_match(chunk.as_str()) {
                mod_line = mod_line.replacen("six", "6", 1);
                continue;
            }
            if seven_re.is_match(chunk.as_str()) {
                mod_line = mod_line.replacen("seven", "7", 1);
                continue;
            }
            if eight_re.is_match(chunk.as_str()) {
                mod_line = mod_line.replacen("eight", "8", 1);
                continue;
            }
            if nine_re.is_match(chunk.as_str()) {
                mod_line = mod_line.replacen("nine", "9", 1);
                continue;
            }
            if zero_re.is_match(chunk.as_str()) {
                mod_line = mod_line.replacen("zero", "0", 1);
                continue;
            }

            println!("mod_line is {mod_line}");
            start_cnt += 1;
        }
        println!("Before push: mod_line is {mod_line}");
        println!("------------------");
        content_details.push(mod_line.clone());
    }

    for mod_line in content_details {
        println!("mod_line: {mod_line}");
    }

    // for line in content.split('\n') {
    //     if line.is_empty() {
    //         break;
    //     }
    //
    //     let mut num_str = String::new();
    //
    //     for chr in line.chars() {
    //         if chr.is_numeric() {
    //             num_str.push(chr);
    //         }
    //     }
    //
    //     println!("num_str is {num_str}");
    //     clb_dtls.push(num_str);
    // }
    //
    // for mut num_str in clb_dtls {
    //     if num_str.len() == 2 {
    //         clb_total += num_str.parse::<i64>().unwrap();
    //     } else if num_str.len() == 1 {
    //         num_str.push(num_str.chars().nth(0).unwrap());
    //         clb_total += num_str.parse::<i64>().unwrap();
    //     } else if num_str.len() > 2 {
    //         let mut tmp_str = String::new();
    //         tmp_str.push(num_str.chars().nth(0).unwrap());
    //         tmp_str.push(num_str.chars().last().unwrap());
    //         clb_total += tmp_str.parse::<i64>().unwrap();
    //     }
    // }
    //
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
