use std::{fs, char};
use std::time::Instant;

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;

const WIN_BONUS: u32 = 6;
const DRAW_BONUS: u32 = 3;
const LOSE_BONUS: u32 = 0;

fn item_value(item: char) -> u32 {
    match item {
        'A' | 'X' => ROCK,
        'B' | 'Y' => PAPER,
        'C' | 'Z' => SCISSORS,
        _ => panic!("Invalid item sent"),
    }
}

fn get_score(opp: char, mine: char) -> u32 {
    let mut res: u32 = 0;

    let opp_val = item_value(opp);
    let my_val = item_value(mine);

    match my_val {
        ROCK => {
            // LOSE case
            match opp_val {
                ROCK => res += SCISSORS + LOSE_BONUS,
                PAPER => res += ROCK + LOSE_BONUS,
                _ => res += PAPER + LOSE_BONUS
            }
        },
        // DRAW case
        PAPER => res += opp_val + DRAW_BONUS,
        _ => {
            // Winning case - SCISSORS
            match opp_val {
                ROCK => res += PAPER + WIN_BONUS,
                PAPER => res += SCISSORS + WIN_BONUS,
                _ => res += ROCK + WIN_BONUS
            }
        }
    }

    res
}

fn main() {
    let start_time = Instant::now();
    let contents = fs::read_to_string("inp_file.txt").expect("Unable to load the file!");
    let mut ctx = Vec::new();

    for rec in contents.split('\n') {
        if !rec.is_empty() {
            let pair: (char, char) = (rec.chars().nth(0).unwrap(), rec.chars().nth(2).unwrap());
            ctx.push(pair);
        }
    }

    let mut tot_score: u32 = 0;
    for pair in ctx {
        tot_score += get_score(pair.0, pair.1);
    }
    println!("Total score is {}", tot_score);
    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
