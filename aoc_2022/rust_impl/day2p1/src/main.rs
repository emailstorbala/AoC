use std::time::Instant;
use std::{char, fs};

#[derive(PartialEq, Eq)]
enum Objects {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

const WIN_BONUS: i32 = 6;
const DRAW_BONUS: i32 = 3;
const LOSE_BONUS: i32 = 0;

fn item_value(item: char) -> Objects {
    match item {
        'A' | 'X' => Objects::ROCK,
        'B' | 'Y' => Objects::PAPER,
        'C' | 'Z' => Objects::SCISSORS,
        _ => panic!("Invalid item sent"),
    }
}

fn get_score(opp_val: Objects, my_val: Objects) -> i32 {
    let mut res: i32 = 0;

    if opp_val == my_val {
        // Draw case
        res += opp_val as i32 + DRAW_BONUS;
    } else if (opp_val == Objects::ROCK && my_val == Objects::SCISSORS)
        || (opp_val == Objects::SCISSORS && my_val == Objects::PAPER)
        || (opp_val == Objects::PAPER && my_val == Objects::ROCK)
    {
        // Losing case
        res += my_val as i32 + LOSE_BONUS;
    } else {
        // Winning case
        res += my_val as i32 + WIN_BONUS;
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

    let mut tot_score: i32 = 0;
    for pair in ctx {
        tot_score += get_score(item_value(pair.0), item_value(pair.1));
    }
    println!("Total score is {}", tot_score);
    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
