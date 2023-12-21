use clap::Parser;
use std::collections::HashMap;
use std::time::Instant;
use std::fs;
use sscanf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

struct CardDetails {
    win_numbers: Vec<i32>,
    our_numbers: Vec<i32>,
}

type GiftCardDetails = HashMap<i16, CardDetails>;

fn read_contents(content: String) {
    let mut gift_card_details: GiftCardDetails = HashMap::new();

    for line in content.split('\n') {
        if line.is_empty() {
            break;
        };

        let (card_info, win_data, our_data) = sscanf::sscanf!(line, "{&str}: {&str} | {&str}").unwrap();

        let card_id_str: &str = sscanf::sscanf!(card_info, "Card {&str}").unwrap();
        let card_id: i16 = card_id_str.trim().parse::<i16>().unwrap();
        let mut win_numbers: Vec<i32> = Vec::new();
        let mut our_numbers: Vec<i32> = Vec::new();

        for win_num_str in win_data.split_whitespace() {
            let win_num: i32 = win_num_str.parse::<i32>().unwrap();
            win_numbers.push(win_num);
        }
        
        for our_num_str in our_data.split_whitespace() {
            let our_num: i32 = our_num_str.parse::<i32>().unwrap();
            our_numbers.push(our_num);
        }

        let card_details = CardDetails {
            win_numbers,
            our_numbers,
        };

        gift_card_details.insert(card_id, card_details);
    }

    let mut sel_won_details: HashMap<i16, Vec<i32>> = HashMap::new();

    for (card_id, card_details) in gift_card_details {
        let mut sel_won_numbers: Vec<i32> = Vec::new();

        for our_num in card_details.our_numbers {
            if card_details.win_numbers.contains(&our_num) {
                sel_won_numbers.push(our_num);
            }
        }
        sel_won_details.insert(card_id, sel_won_numbers);
    }

    let mut total: i32 = 0;
    for (_, sel_won_numbers) in sel_won_details {
        if sel_won_numbers.len() > 0 {
            total += (2 as i32).pow(((sel_won_numbers.len() - 1) as i32).try_into().unwrap());
        }
    }
    println!("total is {total}");
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let content: String = fs::read_to_string(&args.file_name).expect("Unable to load the file!");

    read_contents(content);

    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
