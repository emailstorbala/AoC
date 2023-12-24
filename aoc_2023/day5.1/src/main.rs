use clap::Parser;
use sscanf;
use core::panic;
use std::collections::HashMap;
use std::iter::zip;
use std::fs;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

type CommonMap = HashMap<i64, i64>;

struct Seeds {
    seeds: Vec<i64>,
}

fn get_result_list(inp_list: Vec<i64>,
                   src_list: Vec<i64>,
                   dest_list: Vec<i64>,
) -> Vec<i64> {
    let mut tmp_list: Vec<i64> = Vec::new();
    let mut common_map: CommonMap = HashMap::from([]);

    println!("Zipping started ...");
    for (src, dest) in zip(src_list, dest_list) {
        common_map.insert(src, dest);
    }
    println!("Seed soil mode completed");

    // Collect soil list
    for seed in inp_list {
        let soil: i64 = if common_map.contains_key(&seed) {
            *common_map.get(&seed).unwrap()
        } else {
            seed
        };
        tmp_list.push(soil);
    }

    tmp_list
}


fn read_contents(content: String) -> Vec<i64> {
    let mut seeds = Seeds {
        seeds: Vec::new(),
    };

    let mut seed_to_soil_mode: bool = false;
    let mut soil_to_fert_mode: bool = false;
    let mut fert_to_water_mode: bool = false;
    let mut water_to_light_mode: bool = false;
    let mut light_to_temp_mode: bool = false;
    let mut temp_to_humid_mode: bool = false;
    let mut humid_to_loc_mode: bool = false;
    
    let mut tmp_list: Vec<i64> = Vec::new();

    for line in content.split('\n') {
        if line.is_empty() {
            println!("One mode completed!");
            seed_to_soil_mode = false;
            soil_to_fert_mode = false;
            fert_to_water_mode = false;
            water_to_light_mode = false;
            light_to_temp_mode = false;
            temp_to_humid_mode = false;
            humid_to_loc_mode = false;
            continue;
        }

        println!("line is {line}");
        if line.starts_with("seeds:") {
            let seed_info = sscanf::sscanf!(line, "seeds: {&str}").unwrap();
            seeds.seeds = seed_info
                .split_whitespace()
                .map(|seed| -> i64 { seed.parse().unwrap() })
                .collect();
            println!("Seed info collected");
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
                    /* Do mode related operations */
                    let (dest_range, src_range, range_len) = sscanf::sscanf!(line, "{i64} {i64} {i64}").unwrap();
                    println!("dest_range,src_range,range_len->{dest_range},{src_range},{range_len}");
                    let src_list: Vec<i64> = (src_range..src_range+range_len).collect();
                    println!("src_list prepared!");
                    let dest_list: Vec<i64> = (dest_range..dest_range+range_len).collect();
                    println!("dest_list prepared!");

                    if seed_to_soil_mode {
                        println!("Seed->Soil mode started ...");
                        tmp_list = get_result_list(seeds.seeds.clone(), src_list, dest_list);
                    } else if soil_to_fert_mode {
                        println!("Soil->Fertilizer mode started ...");
                        tmp_list = get_result_list(tmp_list.clone(), src_list, dest_list);
                    } else if fert_to_water_mode {
                        println!("Fertilizer->Water mode started ...");
                        tmp_list = get_result_list(tmp_list.clone(), src_list, dest_list);
                    } else if water_to_light_mode {
                        println!("Water->Light mode started ...");
                        tmp_list = get_result_list(tmp_list.clone(), src_list, dest_list);
                    } else if light_to_temp_mode {
                        println!("Light->Temperature mode started ...");
                        tmp_list = get_result_list(tmp_list.clone(), src_list, dest_list);
                    } else if temp_to_humid_mode {
                        println!("Temperature->Humidity mode started ...");
                        tmp_list = get_result_list(tmp_list.clone(), src_list, dest_list);
                    } else if humid_to_loc_mode {
                        println!("Humidity->Location mode started ...");
                        tmp_list = get_result_list(tmp_list.clone(), src_list, dest_list);
                    } else {
                        panic!("Invalid mode detected!");
                    }
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

    println!("Before loading the input");
    let loc_list = read_contents(content);
    println!("After loading the input");
    println!("Loc list is {:?}", loc_list);
    // println!("Loc list is {:?}", loc_list);
    // let min_loc:i64 = *loc_list.iter().min().unwrap();
    //
    // println!("The answer is {}", min_loc);

    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
