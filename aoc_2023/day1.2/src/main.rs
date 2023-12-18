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

fn get_clb_details(content_details: Vec<String>) -> i64 {
    let mut clb_total: i64 = 0;
    for line in content_details {
        let num_iter = line.chars().filter(|chr| chr.is_numeric());
        let first_num = num_iter.clone().nth(0).unwrap().to_digit(10).unwrap();
        let last_num = num_iter.last().unwrap().to_digit(10).unwrap();
        let clb = first_num * 10 + last_num;
        clb_total += clb as i64
    }
    clb_total 
}

fn read_contents(content: String) -> i64 {
    let mut content_details: Vec<String> = Vec::new();

    let one_re = Regex::new(r"(^one).*").unwrap();
    let two_re = Regex::new(r"(^two).*").unwrap();
    let three_re = Regex::new(r"(^three).*").unwrap();
    let four_re = Regex::new(r"(^four).*").unwrap();
    let five_re = Regex::new(r"(^five).*").unwrap();
    let six_re = Regex::new(r"(^six).*").unwrap();
    let seven_re = Regex::new(r"(^seven).*").unwrap();
    let eight_re = Regex::new(r"(^eight).*").unwrap();
    let nine_re = Regex::new(r"(^nine).*").unwrap();

    for line in content.split('\n') {
        if line.is_empty() { break };

        let mut mod_line: String = String::new();
        let mut start_cnt: usize = 0;
        while start_cnt < line.len() {
            let chunk: String = line[start_cnt..].to_string();
            // Line less than smallest string number E.g: one

            if one_re.is_match(chunk.as_str()) {
                mod_line.push('1');
            } else if two_re.is_match(chunk.as_str()) {
                mod_line.push('2');
            } else if three_re.is_match(chunk.as_str()) {
                mod_line.push('3');
            } else if four_re.is_match(chunk.as_str()) {
                mod_line.push('4');
            } else if five_re.is_match(chunk.as_str()) {
                mod_line.push('5');
            } else if six_re.is_match(chunk.as_str()) {
                mod_line.push('6');
            } else if seven_re.is_match(chunk.as_str()) {
                mod_line.push('7');
            } else if eight_re.is_match(chunk.as_str()) {
                mod_line.push('8');
            } else if nine_re.is_match(chunk.as_str()) {
                mod_line.push('9');
            } else {
                let chr = line.chars().nth(start_cnt).unwrap();
                if chr.is_numeric() { mod_line.push(chr)};
            }

            start_cnt += 1;
        }
        content_details.push(mod_line);
    }

    let clb_total = get_clb_details(content_details);
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
