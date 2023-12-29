use clap::Parser;
use sscanf;
use core::panic;
use std::sync::mpsc::channel;
use std::{fs, i64};
use std::time::Instant;
use threadpool::ThreadPool;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

fn get_result_list(inp_list: &Vec<i64>, lines: &Vec<String>) -> Vec<i64> {
    let mut res_list: Vec<i64> = Vec::new();

    println!("Collecting list ...");
    // Collect list
    for inp in inp_list {
        let mut dest:i64 = *inp;
        for line in lines {
            let (dest_range, src_range, range_len) = sscanf::sscanf!(line, "{i64} {i64} {i64}").unwrap();
            let src_list: Vec<i64> = (src_range..(src_range+range_len)).collect();
            if src_list.contains(inp) {
                let idx = src_list.iter().position(|&n| n == *inp).unwrap();
                dest = (dest_range..(dest_range+range_len)).nth(idx).unwrap();
                // Match found. Hence break
                break;
            }
        }
        res_list.push(dest);
    }
    println!("Collected list");
    res_list 
}

fn get_location_list(seed_info: &SeedInfo) -> Vec<i64> {
    let pool = ThreadPool::new(3);
    let mut collected_mins: Vec<i64> = Vec::new();
    for (start, range) in &seed_info.seeds {
        let begin = *start as i64;
        let end = (start + range) as i64;

        println!("start,range is ({start},{range})");
        let (sender, receiver) = channel::<SeedInfo>();
        let (tx, rx) = channel::<i64>();
        pool.execute(move || {
            let loc_seed_info: SeedInfo = receiver.recv().unwrap();
            println!("Before get_result_list ...");
            let tmp_list: Vec<i64> = (begin..end).collect();
            println!("tmp_list size is {}", tmp_list.len());
            let mut tmp_list = get_result_list(&tmp_list, &loc_seed_info.seed_to_soil_info);
            tmp_list = get_result_list(&tmp_list, &loc_seed_info.soil_to_fert_info);
            tmp_list = get_result_list(&tmp_list, &loc_seed_info.fert_to_water_info);
            tmp_list = get_result_list(&tmp_list, &loc_seed_info.water_to_light_info);
            tmp_list = get_result_list(&tmp_list, &loc_seed_info.light_to_temp_info);
            tmp_list = get_result_list(&tmp_list, &loc_seed_info.temp_to_humid_info);
            tmp_list = get_result_list(&tmp_list, &loc_seed_info.humid_to_loc_info);
            println!("After get_result_list");
            let min_val: i64 = *tmp_list.iter().min().unwrap();
            tx.send(min_val).unwrap();
        });
        sender.send(seed_info.clone()).unwrap();
        collected_mins.push(rx.recv().unwrap());
    }

    println!("collected_mins is {:?}", collected_mins);
    collected_mins
}

#[derive(Clone)]
struct SeedInfo {
    seeds: Vec<(i64, i64)>,
    seed_to_soil_info: Vec<String>,
    soil_to_fert_info: Vec<String>,
    fert_to_water_info: Vec<String>,
    water_to_light_info: Vec<String>,
    light_to_temp_info: Vec<String>,
    temp_to_humid_info: Vec<String>,
    humid_to_loc_info: Vec<String>,
}

fn read_contents(content: String) -> SeedInfo {
    let mut seed_info = SeedInfo {
        seeds : vec![],
        seed_to_soil_info : vec![],
        soil_to_fert_info : vec![],
        fert_to_water_info : vec![],
        water_to_light_info : vec![],
        light_to_temp_info : vec![],
        temp_to_humid_info : vec![],
        humid_to_loc_info : vec![],
    };

    let mut seed_to_soil_mode: bool = false;
    let mut soil_to_fert_mode: bool = false;
    let mut fert_to_water_mode: bool = false;
    let mut water_to_light_mode: bool = false;
    let mut light_to_temp_mode: bool = false;
    let mut temp_to_humid_mode: bool = false;
    let mut humid_to_loc_mode: bool = false;
    
    let mut lines: Vec<String> = Vec::new();

    for line in content.split('\n') {
        if line.is_empty() {
            if seed_to_soil_mode {
                seed_info.seed_to_soil_info = lines.clone();
            } else if soil_to_fert_mode {
                seed_info.soil_to_fert_info = lines.clone();
            } else if fert_to_water_mode {
                seed_info.fert_to_water_info = lines.clone();
            } else if water_to_light_mode {
                seed_info.water_to_light_info = lines.clone();
            } else if light_to_temp_mode {
                seed_info.light_to_temp_info = lines.clone();
            } else if temp_to_humid_mode {
                seed_info.temp_to_humid_info = lines.clone();
            } else if humid_to_loc_mode {
                seed_info.humid_to_loc_info = lines.clone();
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
            let seed_line = sscanf::sscanf!(line, "seeds: {&str}").unwrap();
            let seed_data:Vec<usize> = seed_line
                .split_whitespace()
                .map(|seed| -> usize { seed.parse().unwrap() })
                .collect();
            let mut start: i64 = 0;
            for idx in 0..seed_data.len() {
                if idx%2 ==0 {
                    start = *seed_data.get(idx).unwrap() as i64;
                } else {
                    let range: i64 = *seed_data.get(idx).unwrap() as i64;
                    seed_info.seeds.push((start, range));
                }
            }
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
                    lines.push(line.to_string());
                }
            }
        }

    }
    //println!("Seeds info is {:?}", seeds.seeds);
    seed_info
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let content: String = fs::read_to_string(&args.file_name).expect("Unable to load the file!");

    let seed_info = read_contents(content);
    let loc_min = *get_location_list(&seed_info).iter().min().unwrap();
    println!("loc min is {loc_min}");

    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
