use std::fs;

fn main() {
    let contents = fs::read_to_string("inp_file.txt").expect("Unable to load file!");
    let calories = contents.split("\n\n").map(|chunk| -> i64 {
        chunk
            .split('\n')
            .map(|chunk_rec| chunk_rec.parse().unwrap_or(0))
            .sum()
    });

    let mut calorie_coll: Vec<i64> = calories.collect();
    calorie_coll.sort();

    let coll_len = calorie_coll.len();
    println!("Highest calorie is {}", calorie_coll[coll_len - 1]);
}
