use clap::Parser;
use sscanf;
use core::panic;
use std::sync::mpsc::channel;
use std::{fs, i64};
use std::time::Instant;
use std::thread;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

fn get_result_list(inp_list: std::ops::Range<i64>, seed_info: &SeedInfo) -> Vec<i64> {
    let mut res_list: Vec<i64> = Vec::new();
    let mut tmp_list: Vec<i64> = Vec::new();

    println!("Collecting soil list ...");
    // Collect soil list
    for inp in inp_list {
        let mut dest:i64 = inp;
        for line in &seed_info.seed_to_soil_info {
            let (dest_range, src_range, range_len) = sscanf::sscanf!(line, "{i64} {i64} {i64}").unwrap();
            if (src_range..(src_range+range_len)).contains(&inp) {
                let idx: usize = (src_range..(src_range+range_len)).position(|n| n == inp).unwrap();
                dest = (dest_range..(dest_range+range_len)).nth(idx).unwrap();
                // Match found. Hence break
            }
        }
        res_list.push(dest);
    }
    print!("Collected soil list");

    println!("Collecting fertilizer list ...");
    // Collect fertilizer list
    for soil in &res_list {
        let mut dest:i64 = *soil;
        for line in &seed_info.soil_to_fert_info {
            let (dest_range, src_range, range_len) = sscanf::sscanf!(line, "{i64} {i64} {i64}").unwrap();
            if (src_range..(src_range+range_len)).contains(&soil) {
                let idx: usize = (src_range..(src_range+range_len)).position(|n| n == *soil).unwrap();
                dest = (dest_range..(dest_range+range_len)).nth(idx).unwrap();
                // Match found. Hence break
            }
        }
        tmp_list.push(dest);
    }
    println!("Collected fertilizer list");

    res_list.clear();

    println!("Collecting water list ...");
    // Collect water list
    for fert in &tmp_list {
        let mut dest:i64 = *fert;
        for line in &seed_info.fert_to_water_info {
            let (dest_range, src_range, range_len) = sscanf::sscanf!(line, "{i64} {i64} {i64}").unwrap();
            if (src_range..(src_range+range_len)).contains(&fert) {
                let idx: usize = (src_range..(src_range+range_len)).position(|n| n == *fert).unwrap();
                dest = (dest_range..(dest_range+range_len)).nth(idx).unwrap();
                // Match found. Hence break
            }
        }
        res_list.push(dest);
    }
    println!("Collected water list");

    tmp_list.clear();

    println!("Collecting light list ...");
    // Collect light list
    for water in &res_list {
        let mut dest:i64 = *water;
        for line in &seed_info.water_to_light_info {
            let (dest_range, src_range, range_len) = sscanf::sscanf!(line, "{i64} {i64} {i64}").unwrap();
            if (src_range..(src_range+range_len)).contains(&water) {
                let idx: usize = (src_range..(src_range+range_len)).position(|n| n == *water).unwrap();
                dest = (dest_range..(dest_range+range_len)).nth(idx).unwrap();
                // Match found. Hence break
            }
        }
        tmp_list.push(dest);
    }
    println!("Collected light list");

    res_list.clear();

    // Collect temperature list
    for light in &tmp_list {
        let mut dest:i64 = *light;
        for line in &seed_info.light_to_temp_info {
            let (dest_range, src_range, range_len) = sscanf::sscanf!(line, "{i64} {i64} {i64}").unwrap();
            if (src_range..(src_range+range_len)).contains(&light) {
                let idx: usize = (src_range..(src_range+range_len)).position(|n| n == *light).unwrap();
                dest = (dest_range..(dest_range+range_len)).nth(idx).unwrap();
                // Match found. Hence break
            }
        }
        res_list.push(dest);
    }

    tmp_list.clear();

    // Collect humidity list
    for temp in &res_list {
        let mut dest:i64 = *temp;
        for line in &seed_info.temp_to_humid_info {
            let (dest_range, src_range, range_len) = sscanf::sscanf!(line, "{i64} {i64} {i64}").unwrap();
            if (src_range..(src_range+range_len)).contains(&temp) {
                let idx: usize = (src_range..(src_range+range_len)).position(|n| n == *temp).unwrap();
                dest = (dest_range..(dest_range+range_len)).nth(idx).unwrap();
                // Match found. Hence break
            }
        }
        tmp_list.push(dest);
    }

    res_list.clear();

    // Collect location list
    for humid in &tmp_list {
        let mut dest:i64 = *humid;
        for line in &seed_info.humid_to_loc_info {
            let (dest_range, src_range, range_len) = sscanf::sscanf!(line, "{i64} {i64} {i64}").unwrap();
            if (src_range..(src_range+range_len)).contains(&humid) {
                let idx: usize = (src_range..(src_range+range_len)).position(|n| n == *humid).unwrap();
                dest = (dest_range..(dest_range+range_len)).nth(idx).unwrap();
                // Match found. Hence break
            }
        }
        res_list.push(dest);
    }

    // println!("tmp_list is {:?}", tmp_list);
    res_list 
}

fn get_location_list(seed_info: &SeedInfo) {
    let mut handles = vec![];
    for (start, range) in &seed_info.seeds {
        let begin = *start as i64;
        let end = (start + range) as i64;

        println!("start,range is ({start},{range})");
        let (sender, receiver) = channel();
        let handle = thread::spawn(move || {
            let mut loc_seed_info: SeedInfo = receiver.recv().unwrap();
            println!("Before get_result_list ...");
            let tmp_list = get_result_list(std::ops::Range{start:begin, end}, &loc_seed_info);
            println!("After get_result_list");

            loc_seed_info.loc_list.push(*tmp_list.iter().min().unwrap());
            println!("loc_list is {:?}", loc_seed_info.loc_list);
        });
        handles.push(handle);
        sender.send(seed_info.clone()).unwrap();
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[derive(Clone)]
struct SeedInfo {
    seeds: Vec<(usize, usize)>,
    seed_to_soil_info: Vec<String>,
    soil_to_fert_info: Vec<String>,
    fert_to_water_info: Vec<String>,
    water_to_light_info: Vec<String>,
    light_to_temp_info: Vec<String>,
    temp_to_humid_info: Vec<String>,
    humid_to_loc_info: Vec<String>,
    loc_list: Vec<i64>,
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
        loc_list: vec![],
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
            let mut start: usize = 0;
            for idx in 0..seed_data.len() {
                if idx%2 ==0 {
                    start = *seed_data.get(idx).unwrap();
                } else {
                    let range: usize = *seed_data.get(idx).unwrap();
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
    get_location_list(&seed_info);
    println!("Seed seed_info is {:?}", seed_info.seeds);
    println!("Seed seed_to_soil_info is {:?}", seed_info.seed_to_soil_info);

    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
