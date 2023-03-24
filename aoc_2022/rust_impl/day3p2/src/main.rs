use std::time::Instant;
use std::{fs, u32};

fn get_group_common_items(file_contents: &str) -> String {
    let mut total_common_chars: String = String::from("");
    let mut tmp_grp: Vec<String> = Vec::new();

    for record in file_contents.split('\n') {
        if !record.is_empty() {
            tmp_grp.push(record.to_string());
            if tmp_grp.len() == 3 {
                let comp1: String = tmp_grp.iter().nth(0).unwrap().to_string();
                let comp2: String = tmp_grp.iter().nth(1).unwrap().to_string();
                let comp3: String = tmp_grp.iter().nth(2).unwrap().to_string();

                let mut common_chars: String = String::from("");
                for item in comp1.chars() {
                    if !common_chars.contains(item) && comp2.contains(item) && comp3.contains(item)
                    {
                        common_chars.push(item);
                    }
                }
                total_common_chars += &common_chars;
                tmp_grp.clear();
            }
        }
    }

    total_common_chars
}

fn get_item_priority(item: char) -> u32 {
    let priority: u32;
    if item as u32 >= 65 && item as u32 <= 90 {
        // Capital letter case
        priority = item as u32 - 65 + 27;
    } else if item as u32 >= 97 && item as u32 <= 122 {
        // lower letter case should come here
        priority = item as u32 - 96;
    } else {
        panic!("Invalid item '{}' in input file!", item);
    }

    priority
}

fn main() {
    let start_time = Instant::now();
    let contents = fs::read_to_string("inp_file.txt").expect("Unable to load the file!");

    let mut total_priority: u32 = 0;
    for item in get_group_common_items(&contents).chars() {
        total_priority += get_item_priority(item);
    }

    println!("Total priority is {}", total_priority);

    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
