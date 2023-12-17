use clap::Parser;
use std::fs;
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
    let mut clb_dtls: Vec<String> = Vec::new();
    let mut content_details: Vec<String> = Vec::new();

    for line in content.split('\n') {
        if line.is_empty() { break };

        let mut mod_line: String = line.to_string();
        let mut start_cnt: usize = 0;
        while start_cnt <= mod_line.len() {
            let chunk: String = mod_line[start_cnt..].to_string();
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

            start_cnt += 1;
        }
        content_details.push(mod_line.clone());
    }

    for mod_line in &content_details {
        println!("mod_line: {mod_line}");
    }

    for line in content_details {
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
