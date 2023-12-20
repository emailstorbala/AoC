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

fn collect_numbers(
    content_map: &HashMap<(usize, usize), char>,
    num_of_rows: usize,
    row_size: usize,
) -> HashMap<i32, Vec<(usize, usize)>> {
    println!("num_of_rows is {num_of_rows}");
    println!("num of colums is {row_size}");
    let mut num_and_positions: HashMap<i32, Vec<(usize, usize)>> = Default::default();
    for row in 0..num_of_rows {
        let mut num_str: String = String::new();
        let mut num_pos_list: Vec<(usize, usize)> = Default::default();
        for pos in 0..row_size {
            /* println!("Position fetched is ({}, {})", row, pos); */
            let chr: char = *content_map.get(&(row, pos)).unwrap();
            if chr.is_numeric() {
                num_str.push(chr);
                num_pos_list.push((row, pos));
            } else {
                if !num_str.is_empty() {
                    let number: i32 = num_str.parse::<i32>().unwrap();
                    num_and_positions.insert(number, num_pos_list.clone());
                    num_str.clear();
                    num_pos_list.clear();
                }
            }
        }
    }

    num_and_positions
}

fn check_positions_match(
    positions: Vec<(usize, usize)>,
    content_map: &HashMap<(usize, usize), char>,
) -> bool {
    let mut matched: bool = false;

    for (row, pos) in positions {
        // Left cell
        let (left_row, left_pos) = (row, pos - 1);

        if content_map.contains_key(&(left_row, left_pos)) {
            let cell_char: char = *content_map.get(&(left_row, left_pos)).unwrap();

            // If cell_char is not alphanumeric and '.', we assume it should be symbol.
            if !cell_char.is_alphanumeric() && cell_char != '.' {
                matched = true;
                break;
            }
        }
        // right cell
        let (right_row, right_pos) = (row, pos + 1);
        if content_map.contains_key(&(right_row, right_pos)) {
            let cell_char: char = *content_map.get(&(right_row, right_pos)).unwrap();

            // If cell_char is not alphanumeric and '.', we assume it should be symbol.
            if !cell_char.is_alphanumeric() && cell_char != '.' {
                matched = true;
                break;
            }
        }

        // diagonal topleft
        let (diag_topleft_row, diag_topleft_pos) = (row - 1, pos - 1);
        if content_map.contains_key(&(diag_topleft_row, diag_topleft_pos)) {
            let cell_char: char = *content_map
                .get(&(diag_topleft_row, diag_topleft_pos))
                .unwrap();

            // If cell_char is not alphanumeric and '.', we assume it should be symbol.
            if !cell_char.is_alphanumeric() && cell_char != '.' {
                matched = true;
                break;
            }
        }

        // diagonal topright
        let (diag_topright_row, diag_topright_pos) = (row - 1, pos + 1);
        if content_map.contains_key(&(diag_topright_row, diag_topright_pos)) {
            let cell_char: char = *content_map
                .get(&(diag_topright_row, diag_topright_pos))
                .unwrap();

            // If cell_char is not alphanumeric and '.', we assume it should be symbol.
            if !cell_char.is_alphanumeric() && cell_char != '.' {
                matched = true;
                break;
            }
        }

        // diagonal bottomright
        let (diag_bottomright_row, diag_bottomright_pos) = (row + 1, pos + 1);
        if content_map.contains_key(&(diag_bottomright_row, diag_bottomright_pos)) {
            let cell_char: char = *content_map
                .get(&(diag_bottomright_row, diag_bottomright_pos))
                .unwrap();

            // If cell_char is not alphanumeric and '.', we assume it should be symbol.
            if !cell_char.is_alphanumeric() && cell_char != '.' {
                matched = true;
                break;
            }
        }

        // diagonal bottomleft
        let (diag_bottomleft_row, diag_bottomleft_pos) = (row + 1, pos - 1);
        if content_map.contains_key(&(diag_bottomleft_row, diag_bottomleft_pos)) {
            let cell_char: char = *content_map
                .get(&(diag_bottomleft_row, diag_bottomleft_pos))
                .unwrap();

            // If cell_char is not alphanumeric and '.', we assume it should be symbol.
            if !cell_char.is_alphanumeric() && cell_char != '.' {
                matched = true;
                break;
            }
        }

        // above
        let (above_row, above_pos) = (row - 1, pos);
        if content_map.contains_key(&(above_row, above_pos)) {
            let cell_char: char = *content_map.get(&(above_row, above_pos)).unwrap();

            // If cell_char is not alphanumeric and '.', we assume it should be symbol.
            if !cell_char.is_alphanumeric() && cell_char != '.' {
                matched = true;
                break;
            }
        }

        // below
        let (below_row, below_pos) = (row + 1, pos);
        if content_map.contains_key(&(below_row, below_pos)) {
            let cell_char: char = *content_map.get(&(below_row, below_pos)).unwrap();

            // If cell_char is not alphanumeric and '.', we assume it should be symbol.
            if !cell_char.is_alphanumeric() && cell_char != '.' {
                matched = true;
                break;
            }
        }
    }

    matched
}

fn read_contents(content: String, row_size: usize) -> i32 {
    let mut content_map: HashMap<(usize, usize), char> = Default::default();
    let mut row_num: usize = 0;
    for line in content.split('\n') {
        if line.is_empty() {
            break;
        };

        for pos in 0..row_size {
            content_map.insert((row_num, pos), line.chars().nth(pos).unwrap());
        }
        row_num += 1;
    }

    let num_and_positions = collect_numbers(&content_map, row_num, row_size);
    // for (number, positions) in num_and_positions {
    //     println!("number, positions is ({number}, {:?}", positions);
    // }
    let mut total: i32 = 0;
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
