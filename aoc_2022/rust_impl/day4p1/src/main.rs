use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

fn get_hash_set(beg: i32, end: i32) -> HashSet<i32> {
    let mut ret_set: HashSet<i32> = HashSet::new();
    for num in beg..=end {
        ret_set.insert(num);
    }
    return ret_set;
}

fn get_ranges_list(contents: &str) -> Vec<(HashSet<i32>, HashSet<i32>)> {
    let mut ret_assignments: Vec<(HashSet<i32>, HashSet<i32>)> = Vec::new();

    for chunk in contents.split('\n') {
        if !chunk.is_empty() {
            let (pl1, pl2): (&str, &str) = (
                chunk.split(',').nth(0).unwrap(),
                chunk.split(',').nth(1).unwrap(),
            );

            let (pl1_beg, pl1_end, pl2_beg, pl2_end): (i32, i32, i32, i32) = (
                pl1.split('-').nth(0).unwrap().parse::<i32>().unwrap(),
                pl1.split('-').nth(1).unwrap().parse::<i32>().unwrap(),
                pl2.split('-').nth(0).unwrap().parse::<i32>().unwrap(),
                pl2.split('-').nth(1).unwrap().parse::<i32>().unwrap(),
            );

            ret_assignments.push((
                get_hash_set(pl1_beg, pl1_end),
                get_hash_set(pl2_beg, pl2_end),
            ));
        }
    }

    ret_assignments
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let contents = fs::read_to_string(&args.file_name).expect("Unable to load the file!");
    let ranges_list = get_ranges_list(&contents);

    let mut fully_contains: i32 = 0;
    for (hsh1, hsh2) in ranges_list {
        if hsh1.is_superset(&hsh2) || hsh2.is_superset(&hsh1) {
            fully_contains += 1;
        }
    }

    println!("Full contained assignments counts {fully_contains}");

    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
