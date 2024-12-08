use clap::Parser;
use indexmap::IndexMap;
use std::time::Instant;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

fn read_contents(content: String) -> IndexMap<(usize, usize), char> {
    let mut puz_input: IndexMap<(usize, usize), char> = IndexMap::new();
    let mut row: usize = 0;
    for line in content.split('\n') {
        if line.is_empty() {
            break;
        }

        for (col, chr) in line.chars().enumerate() {
            puz_input.insert((row, col), chr);

        }
        row += 1;
    }

    puz_input
}

fn get_xmas_count(puz_input: IndexMap<(usize, usize), char>) -> usize {
    let mut found_positions: Vec<(usize, usize, usize, usize)> = Vec::new();

    for (pos, chr) in &puz_input {
        if *chr == 'X' {
            for mode in ["right", "left", "up", "down", "diagonalTLR", "diagonalTRL", "diagonalBLR", "diagonalBRL"] {
                let mut xmas_found: bool;
                match mode {
                    "right" => {
                        xmas_found = true;
                        // Get remaining chars horizontally
                        for cnt in 1..=3 {
                            let new_pos: (usize, usize) = (pos.0, pos.1 + cnt);
                            // println!("new_pos is {:?}", new_pos);
                            if puz_input.contains_key(&new_pos) {
                                if cnt == 1 && *puz_input.get(&new_pos).unwrap() == 'M' {
                                } else if cnt == 2 && *puz_input.get(&new_pos).unwrap() == 'A' {
                                } else if cnt == 3 && *puz_input.get(&new_pos).unwrap() == 'S' {
                                } else {
                                    xmas_found = false;
                                    break;
                                }
                            } else {
                                xmas_found = false;
                                break;
                            }
                        }

                        if xmas_found {
                            let end_pos = (pos.0, pos.1 + 3);
                            if puz_input.contains_key(&end_pos) && !found_positions.contains(&(pos.0, pos.1, end_pos.0, end_pos.1)) {
                                found_positions.push((pos.0, pos.1, end_pos.0, end_pos.1));
                            }
                        }
                    },
                    "left" => {
                        xmas_found = true;
                        // Get remaining chars horizontally backwords
                        for cnt in 1..=3 {
                            let new_pos: (usize, usize) = (pos.0, pos.1 - cnt);
                            // println!("new_pos is {:?}", new_pos);
                            if puz_input.contains_key(&new_pos) {
                                if cnt == 1 && *puz_input.get(&new_pos).unwrap() == 'M' {
                                } else if cnt == 2 && *puz_input.get(&new_pos).unwrap() == 'A' {
                                } else if cnt == 3 && *puz_input.get(&new_pos).unwrap() == 'S' {
                                } else {
                                    xmas_found = false;
                                    break;
                                }
                            } else {
                                xmas_found = false;
                                break;
                            }
                        }

                        if xmas_found {
                            let end_pos = (pos.0, pos.1 - 3);
                            if puz_input.contains_key(&end_pos) && !found_positions.contains(&(pos.0, pos.1, end_pos.0, end_pos.1)) {
                                found_positions.push((pos.0, pos.1, end_pos.0, end_pos.1));
                            }
                        }
                    }
                    "up" => {
                        xmas_found = true;
                        // Get remaining chars vertically up
                        for cnt in 1..=3 {
                            let new_pos: (usize, usize) = (pos.0 - cnt, pos.1);
                            // println!("new_pos is {:?}", new_pos);
                            if puz_input.contains_key(&new_pos) {
                                if cnt == 1 && *puz_input.get(&new_pos).unwrap() == 'M' {
                                } else if cnt == 2 && *puz_input.get(&new_pos).unwrap() == 'A' {
                                } else if cnt == 3 && *puz_input.get(&new_pos).unwrap() == 'S' {
                                } else {
                                    xmas_found = false;
                                    break;
                                }
                            } else {
                                xmas_found = false;
                                break;
                            }
                        }

                        if xmas_found {
                            let end_pos = (pos.0 - 3, pos.1);
                            if puz_input.contains_key(&end_pos) && !found_positions.contains(&(pos.0, pos.1, end_pos.0, end_pos.1)) {
                                found_positions.push((pos.0, pos.1, end_pos.0, end_pos.1));
                            }
                        }
                    }
                    "down" => {
                        xmas_found = true;
                        // Get remaining chars vertically down
                        for cnt in 1..=3 {
                            let new_pos: (usize, usize) = (pos.0 + cnt, pos.1);
                            // println!("new_pos is {:?}", new_pos);
                            if puz_input.contains_key(&new_pos) {
                                if cnt == 1 && *puz_input.get(&new_pos).unwrap() == 'M' {
                                } else if cnt == 2 && *puz_input.get(&new_pos).unwrap() == 'A' {
                                } else if cnt == 3 && *puz_input.get(&new_pos).unwrap() == 'S' {
                                } else {
                                    xmas_found = false;
                                    break;
                                }
                            } else {
                                xmas_found = false;
                                break;
                            }
                        }

                        if xmas_found {
                            let end_pos = (pos.0 + 3, pos.1);
                            if puz_input.contains_key(&end_pos) && !found_positions.contains(&(pos.0, pos.1, end_pos.0, end_pos.1)) {
                                found_positions.push((pos.0, pos.1, end_pos.0, end_pos.1));
                            }
                        }
                    }
                    "diagonalTLR" => {
                        xmas_found = true;
                        // Get remaining chars diagonal - left to right
                        for cnt in 1..=3 {
                            let new_pos: (usize, usize) = (pos.0 + cnt, pos.1 + cnt);
                            // println!("new_pos is {:?}", new_pos);
                            if puz_input.contains_key(&new_pos) {
                                if cnt == 1 && *puz_input.get(&new_pos).unwrap() == 'M' {
                                } else if cnt == 2 && *puz_input.get(&new_pos).unwrap() == 'A' {
                                } else if cnt == 3 && *puz_input.get(&new_pos).unwrap() == 'S' {
                                } else {
                                    xmas_found = false;
                                    break;
                                }
                            } else {
                                xmas_found = false;
                                break;
                            }
                        }

                        if xmas_found {
                            let end_pos = (pos.0 + 3, pos.1 + 3);
                            if puz_input.contains_key(&end_pos) && !found_positions.contains(&(pos.0, pos.1, end_pos.0, end_pos.1)) {
                                found_positions.push((pos.0, pos.1, end_pos.0, end_pos.1));
                            }
                        }
                    }
                    "diagonalBLR" => {
                        xmas_found = true;
                        // Get remaining chars diagonal - left to right
                        // column increasing and row decreasing
                        for cnt in 1..=3 {
                            let new_pos: (usize, usize) = (pos.0 - cnt, pos.1 + cnt);
                            // println!("new_pos is {:?}", new_pos);
                            if puz_input.contains_key(&new_pos) {
                                if cnt == 1 && *puz_input.get(&new_pos).unwrap() == 'M' {
                                } else if cnt == 2 && *puz_input.get(&new_pos).unwrap() == 'A' {
                                } else if cnt == 3 && *puz_input.get(&new_pos).unwrap() == 'S' {
                                } else {
                                    xmas_found = false;
                                    break;
                                }
                            } else {
                                xmas_found = false;
                                break;
                            }
                        }

                        if xmas_found {
                            let end_pos = (pos.0 - 3, pos.1 + 3);
                            if puz_input.contains_key(&end_pos) && !found_positions.contains(&(pos.0, pos.1, end_pos.0, end_pos.1)) {
                                found_positions.push((pos.0, pos.1, end_pos.0, end_pos.1));
                            }
                        }
                    }
                    "diagonalTRL" => {
                        xmas_found = true;
                        // Get remaining chars diagonal - right to left
                        // column decreasing and row increasing
                        for cnt in 1..=3 {
                            let new_pos: (usize, usize) = (pos.0 + cnt, pos.1 - cnt);
                            // println!("new_pos is {:?}", new_pos);
                            if puz_input.contains_key(&new_pos) {
                                if cnt == 1 && *puz_input.get(&new_pos).unwrap() == 'M' {
                                } else if cnt == 2 && *puz_input.get(&new_pos).unwrap() == 'A' {
                                } else if cnt == 3 && *puz_input.get(&new_pos).unwrap() == 'S' {
                                } else {
                                    xmas_found = false;
                                    break;
                                }
                            } else {
                                xmas_found = false;
                                break;
                            }
                        }

                        if xmas_found {
                            let end_pos = (pos.0 + 3, pos.1 - 3);
                            if puz_input.contains_key(&end_pos) && !found_positions.contains(&(pos.0, pos.1, end_pos.0, end_pos.1)) {
                                found_positions.push((pos.0, pos.1, end_pos.0, end_pos.1));
                            }
                        }
                    }
                    "diagonalBRL" => {
                        xmas_found = true;
                        // Get remaining chars diagonal - left to right
                        // column decreasing, row decreasing
                        for cnt in 1..=3 {
                            let new_pos: (usize, usize) = (pos.0 - cnt, pos.1 - cnt);
                            // println!("new_pos is {:?}", new_pos);
                            if puz_input.contains_key(&new_pos) {
                                if cnt == 1 && *puz_input.get(&new_pos).unwrap() == 'M' {
                                } else if cnt == 2 && *puz_input.get(&new_pos).unwrap() == 'A' {
                                } else if cnt == 3 && *puz_input.get(&new_pos).unwrap() == 'S' {
                                } else {
                                    xmas_found = false;
                                    break;
                                }
                            } else {
                                xmas_found = false;
                                break;
                            }
                        }

                        if xmas_found {
                            let end_pos = (pos.0 - 3, pos.1 - 3);
                            if puz_input.contains_key(&end_pos) && !found_positions.contains(&(pos.0, pos.1, end_pos.0, end_pos.1)) {
                                found_positions.push((pos.0, pos.1, end_pos.0, end_pos.1));
                            }
                        }
                    }
                    _ => {
                        panic!("Invalid mode: {mode}!");
                    }
                }
            }
        }
    }

    // println!("found_positions: {:?}", found_positions);
    println!("Count of found_positions: {}", found_positions.len());

    found_positions.len()
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let content = fs::read_to_string(&args.file_name).expect("Unable to load the file!");
    let inp_data = read_contents(content);
    // println!("The input data is {:?}", inp_data);
    let xmas_count = get_xmas_count(inp_data);
    println!("xmas_count is {xmas_count}");
    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
