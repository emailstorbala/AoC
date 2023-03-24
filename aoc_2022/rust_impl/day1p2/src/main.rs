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
    calorie_coll.reverse();
    let res_calorie = calorie_coll[0] + calorie_coll[1] + calorie_coll[2];

    println!("Highest calorie is {}", res_calorie);
}
