use std::time::Instant;
use std::{fs, u32};

fn get_compartment_details(file_contents: &str) -> Vec<(String, String)> {
    let mut compartment_details: Vec<(String, String)> = Vec::new();
    for rucksuck in file_contents.split('\n') {
        if !rucksuck.is_empty() {
            let compartment_len = rucksuck.len() / 2;
            let comp_pair: (String, String) = (
                rucksuck[0..compartment_len].to_string(),
                rucksuck[compartment_len..].to_string(),
            );
            compartment_details.push(comp_pair);
        }
    }

    return compartment_details;
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

    let compartment_details = get_compartment_details(&contents);
    let mut total_priority: u32 = 0;

    for (comp1, comp2) in compartment_details {
        let mut common_items: String = String::from("");
        for item in comp1.chars() {
            if comp2.contains(item) && !common_items.contains(item) {
                common_items.push(item);
            }
        }

        /* println!("Common items is {:?}", common_items); */
        for item in common_items.chars() {
            total_priority += get_item_priority(item);
        }
    }

    println!("Total priority is {}", total_priority);

    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
