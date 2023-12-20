use clap::Parser;
use std::collections::HashMap;
use std::time::Instant;
use std::{char, fs, usize};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
    #[arg(short, long)]
    row_size: String,
}

type Position = (usize, usize);
type ContentMap = HashMap<Position, char>;
type NumberAndPositions = HashMap<i64, Vec<Position>>;

fn collect_numbers(
    content_map: &ContentMap,
    num_of_rows: usize,
    row_size: usize, // number of columns
) -> NumberAndPositions {
    println!("num_of_rows is {num_of_rows}");
    println!("num of columns is {row_size}");
    let mut num_and_positions: NumberAndPositions = Default::default();
    for row in 0..num_of_rows {
        let mut num_str: String = String::new();
        let mut num_pos_list: Vec<Position> = Default::default();
        for col in 0..row_size {
            /* println!("Position fetched is ({}, {})", row, col); */
            let position: Position = (row, col);
            let chr: char = *content_map.get(&position).unwrap();
            if chr.is_numeric() {
                num_str.push(chr);
                num_pos_list.push(position);
            } else {
                if !num_str.is_empty() {
                    let number: i64 = num_str.parse::<i64>().unwrap();
                    num_and_positions.insert(number, num_pos_list.clone());
                    num_str.clear();
                    num_pos_list.clear();
                }
            }
        }
    }

    num_and_positions
}

fn position_matches_symbol(content_map: &ContentMap, position: Position) -> bool {
    let mut matched: bool = false;
    if content_map.contains_key(&position) {
        let cell_char: char = *content_map.get(&position).unwrap();

        // If cell_char is not alphanumeric and '.', we assume it should be symbol.
        if !cell_char.is_alphanumeric() && cell_char != '.' {
            matched = true;
        }
    }

    matched
}

fn check_positions_match(positions: Vec<Position>, content_map: &ContentMap) -> bool {
    for (row, col) in positions {
        // Left cell
        let left_cell = (row, col - 1);
        if position_matches_symbol(&content_map, left_cell) {
            return true;
        };

        // right cell
        let right_cell = (row, col + 1);
        if position_matches_symbol(&content_map, right_cell) {
            return true;
        }

        // diagonal topleft cell
        let diag_topleft_cell = (row - 1, col - 1);
        if position_matches_symbol(&content_map, diag_topleft_cell) {
            return true;
        };

        // diagonal topright cell
        let diag_topright_cell = (row - 1, col + 1);
        if position_matches_symbol(&content_map, diag_topright_cell) {
            return true;
        }

        // diagonal bottomleft cell
        let diag_bottomleft_cell = (row + 1, col - 1);
        if position_matches_symbol(&content_map, diag_bottomleft_cell) {
            return true;
        }

        // diagonal bottomright cell
        let diag_bottomright_cell = (row + 1, col + 1);
        if position_matches_symbol(&content_map, diag_bottomright_cell) {
            return true;
        }

        // above cell
        let above_cell = (row - 1, col);
        if position_matches_symbol(&content_map, above_cell) {
            return true;
        }

        // below cell
        let below_cell = (row + 1, col);
        if position_matches_symbol(&content_map, below_cell) {
            return true;
        }
    }

    false
}

fn read_contents(content: String, row_size: usize) -> i64 {
    let mut content_map: ContentMap = Default::default();
    let mut row: usize = 0;
    for line in content.split('\n') {
        if line.is_empty() {
            break;
        };

        for col in 0..row_size {
            content_map.insert((row, col), line.chars().nth(col).unwrap());
        }
        row += 1;
    }

    /* Here row is passed as number of rows */
    let num_and_positions = collect_numbers(&content_map, row, row_size);
    let mut total: i64 = 0;
    for (number, positions) in num_and_positions {
        if check_positions_match(positions, &content_map) {
            println!("The matched number is {number}");
            total += number;
        }
    }

    total
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let content: String = fs::read_to_string(&args.file_name).expect("Unable to load the file!");

    let answer = read_contents(content, args.row_size.parse::<usize>().unwrap());
    println!("The answer is {answer}");

    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
