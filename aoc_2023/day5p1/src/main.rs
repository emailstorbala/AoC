use clap::Parser;
use sscanf;
use core::panic;
use std::fs;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

fn get_result_list(inp_list: &Vec<i64>, lines: &Vec<&str>) -> Vec<i64> {
    let mut tmp_list: Vec<i64> = Vec::new();

    // Collect list
    for inp in inp_list {
        let mut dest:i64 = *inp;
        for line in lines {
            let (dest_range, src_range, range_len) = sscanf::sscanf!(line, "{i64} {i64} {i64}").unwrap();
            if (src_range..(src_range+range_len)).contains(inp) {
                let idx: usize = (src_range..(src_range+range_len)).position(|n| n == *inp).unwrap();
                dest = (dest_range..(dest_range+range_len)).nth(idx).unwrap();
                // Match found. Hence break
            }
        }
        tmp_list.push(dest);
    }

    // println!("tmp_list is {:?}", tmp_list);
    tmp_list
}

fn read_contents(content: String) -> Vec<i64> {
    let mut seeds: Vec<i64> = Vec::new();

    let mut seed_to_soil_mode: bool = false;
    let mut soil_to_fert_mode: bool = false;
    let mut fert_to_water_mode: bool = false;
    let mut water_to_light_mode: bool = false;
    let mut light_to_temp_mode: bool = false;
    let mut temp_to_humid_mode: bool = false;
    let mut humid_to_loc_mode: bool = false;
    
    let mut tmp_list: Vec<i64> = Vec::new();
    let mut lines: Vec<&str> = Vec::new();

    for line in content.split('\n') {
        if line.is_empty() {
            /* Do mode related operations */
            if seed_to_soil_mode {
                tmp_list = get_result_list(&seeds, &lines);
            } else if soil_to_fert_mode {
                tmp_list = get_result_list(&tmp_list, &lines);
            } else if fert_to_water_mode {
                tmp_list = get_result_list(&tmp_list, &lines);
            } else if water_to_light_mode {
                tmp_list = get_result_list(&tmp_list, &lines);
            } else if light_to_temp_mode {
                tmp_list = get_result_list(&tmp_list, &lines);
            } else if temp_to_humid_mode {
                tmp_list = get_result_list(&tmp_list, &lines);
            } else if humid_to_loc_mode {
                tmp_list = get_result_list(&tmp_list, &lines);
            } else {
                if seeds.is_empty() {
                    panic!("Invalid mode detected!");
                }
            }
            seed_to_soil_mode = false;
            soil_to_fert_mode = false;
            fert_to_water_mode = false;
            water_to_light_mode = false;
            light_to_temp_mode = false;
            temp_to_humid_mode = false;
            humid_to_loc_mode = false;
            lines.clear();
            continue;
        }

        if line.starts_with("seeds:") {
            let seed_info = sscanf::sscanf!(line, "seeds: {&str}").unwrap();
            seeds = seed_info
                .split_whitespace()
                .map(|seed| -> i64 { seed.parse().unwrap() })
                .collect();
        } else {
            match line {
                "seed-to-soil map:" => seed_to_soil_mode = true,
                "soil-to-fertilizer map:" => soil_to_fert_mode = true,
                "fertilizer-to-water map:" => fert_to_water_mode = true,
                "water-to-light map:" => water_to_light_mode = true,
                "light-to-temperature map:" => light_to_temp_mode = true,
                "temperature-to-humidity map:" => temp_to_humid_mode = true,
                "humidity-to-location map:" => humid_to_loc_mode = true,
                _ => {
                    lines.push(line);
                }
            }
        }

    }
    //println!("Seeds info is {:?}", seeds.seeds);
    tmp_list
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let content: String = fs::read_to_string(&args.file_name).expect("Unable to load the file!");

    let loc_list = read_contents(content);
    // println!("Loc list is {:?}", loc_list);
    let min_loc:i64 = *loc_list.iter().min().unwrap();
    println!("The answer is {}", min_loc);

    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
