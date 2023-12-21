use clap::Parser;
use std::collections::HashMap;
use std::ops::Index;
use std::time::Instant;
use std::{char, fs, usize};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

type Position = (usize, usize);
type Positions = Vec<Position>;
type ContentMap = HashMap<Position, char>;

#[derive(Clone)]
struct NumberWithPositions {
    number: i64,
    positions: Positions,
}

type NumberList = Vec<NumberWithPositions>;

fn collect_numbers(
    content_map: &ContentMap,
    num_of_rows: usize,
    row_size: usize, // number of columns
) -> NumberList {
    println!("num_of_rows is {num_of_rows}");
    println!("num of columns is {row_size}");
    let mut number_list: NumberList = Vec::new();
    let mut num_str: String = String::new();
    let mut positions: Vec<Position> = Vec::new();
    for row in 0..num_of_rows {
        for col in 0..row_size {
            /* println!("Position fetched is ({}, {})", row, col); */
            let position: Position = (row, col);
            let chr: char = *content_map.get(&position).unwrap();
            if chr.is_numeric() {
                num_str.push(chr);
                positions.push(position);
            } else {
                if !num_str.is_empty() {
                    let number: i64 = num_str.parse::<i64>().unwrap();
                    let number_with_positions = NumberWithPositions {
                        number,
                        positions: positions.clone(),
                    };
                    number_list.push(number_with_positions);
                    positions.clear();
                    num_str.clear();
                }
            }
        }
    }

    number_list
}

fn position_matches_symbol(content_map: &ContentMap, position: Position) -> bool {
    if content_map.contains_key(&position) && *content_map.get(&position).unwrap() == '*' {
        return true;
    }
    return false;
}

fn check_positions_match(positions: Vec<Position>, content_map: &ContentMap) -> Positions {
    let mut matched_positions: Positions = Vec::new();
    for (row, col) in positions {
        // Left cell
        let left_cell = (row, col - 1);
        if position_matches_symbol(&content_map, left_cell) {
            matched_positions.push(left_cell);
        };

        // right cell
        let right_cell = (row, col + 1);
        if position_matches_symbol(&content_map, right_cell) {
            matched_positions.push(right_cell);
        }

        // diagonal topleft cell
        let diag_topleft_cell = (row - 1, col - 1);
        if position_matches_symbol(&content_map, diag_topleft_cell) {
            matched_positions.push(diag_topleft_cell);
        };

        // diagonal topright cell
        let diag_topright_cell = (row - 1, col + 1);
        if position_matches_symbol(&content_map, diag_topright_cell) {
            matched_positions.push(diag_topright_cell);
        }

        // diagonal bottomleft cell
        let diag_bottomleft_cell = (row + 1, col - 1);
        if position_matches_symbol(&content_map, diag_bottomleft_cell) {
            matched_positions.push(diag_bottomleft_cell);
        }

        // diagonal bottomright cell
        let diag_bottomright_cell = (row + 1, col + 1);
        if position_matches_symbol(&content_map, diag_bottomright_cell) {
            matched_positions.push(diag_bottomright_cell)
        }

        // above cell
        let above_cell = (row - 1, col);
        if position_matches_symbol(&content_map, above_cell) {
            matched_positions.push(above_cell);
        }

        // below cell
        let below_cell = (row + 1, col);
        if position_matches_symbol(&content_map, below_cell) {
            matched_positions.push(below_cell);
        }
    }

    matched_positions
}

fn get_any_matched_numbers(number: i64, number_list: NumberList, positions: &Positions) -> i64 {
    let mut matched_number = 0;

    for num_with_pos in number_list {
        let curr_number = num_with_pos.number;
        let curr_positions = num_with_pos.positions.clone();

        for position in positions {
            if curr_positions.contains(&position) && number != curr_number {
                matched_number = curr_number;
                break;
            }
        }
    }

    matched_number
}

fn read_contents(content: String) -> i64 {
    let mut content_map: ContentMap = HashMap::new();
    let mut row: usize = 0;
    let mut row_size: usize = 0;
    for line in content.split('\n') {
        if line.is_empty() {
            break;
        };

        row_size = line.len();
        for col in 0..row_size {
            content_map.insert((row, col), line.chars().nth(col).unwrap());
        }
        row += 1;
    }

    /* Here row is passed as number of rows */
    let mut total: i64 = 0;
    let mut proc_number_list: NumberList = Vec::new();
    for number_with_positions in collect_numbers(&content_map, row, row_size) {
        let number = number_with_positions.number;
        let positions = number_with_positions.positions;
        let matched_positions = check_positions_match(positions, &content_map);
        let matched_number_positions: NumberWithPositions = NumberWithPositions {
            number,
            positions: matched_positions,
        };
        proc_number_list.push(matched_number_positions);
    }

    let mut proc_combinations: Vec<(i64, i64)> = Vec::new();

    for matched_number_positions in proc_number_list.clone() {
        let number = matched_number_positions.number;
        let positions = matched_number_positions.positions;
        // println!("number, positions ->({number},{:?})", positions);
        let matched_number = get_any_matched_numbers(number, proc_number_list.clone(), &positions);

        if matched_number > 0
            && !proc_combinations.contains(&(number, matched_number))
            && !proc_combinations.contains(&(matched_number, number))
        {
            // println!("Matched number is {matched_number}");
            total += number * matched_number;
            proc_combinations.push((number, matched_number));
        }
    }

    total
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let content: String = fs::read_to_string(&args.file_name).expect("Unable to load the file!");

    let answer = read_contents(content);
    println!("The answer is {answer}");

    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
