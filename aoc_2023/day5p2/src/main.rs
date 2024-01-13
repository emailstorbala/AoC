use clap::Parser;
use sscanf;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{fs, i64, usize};
use threadpool::ThreadPool;
use array_tool::vec::Intersect;
use array_tool::vec::Uniq;

const THREAD_POOL_COUNT: usize = 1;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

fn get_common_range(loc1: (i64, i64), loc2: (i64, i64)) -> Vec<i64> {
    let (loc1_start, loc1_end) = loc1;
    let (loc2_start, loc2_end) = loc2;

    return if loc2_start <= loc1_start && loc2_end >= loc1_start {
        // overlap 
        (loc1_start..loc2_end).collect()
    } else if loc2_start <= loc1_end && loc2_end >= loc1_end {
        // overlap
        (loc2_start..loc1_end).collect()
    } else {
        // No overlap
        vec![]
    }
}

fn get_result_list_1(inp: (i64, i64), lines: &Vec<String>) -> Vec<i64> {
    let mut res_list: Vec<i64> = vec![];
    let mut com_list: Vec<i64> = vec![];
    let mut src_list: Vec<i64> = vec![];
    let mut dest_list: Vec<i64> = vec![];

    println!("Before com_list creation");
    for line in lines {
        let (dest_range, src_range, range_len) =
            sscanf::sscanf!(line, "{i64} {i64} {i64}").unwrap();
        let tmp_inp: Vec<i64> = (inp.0 .. inp.1).collect();
        let mut tmp_src: Vec<i64> = (src_range..src_range+range_len).collect();
        let mut tmp_dest: Vec<i64> = (dest_range..dest_range+range_len).collect();
        //let mut tmp_list = get_common_range((src_range, src_range+range_len), (dest_range, dest_range+range_len));
        let mut tmp_list = tmp_inp.intersect(tmp_src.clone());
        com_list.append(&mut tmp_list);
        println!("com_list is {:?}", com_list);
        src_list.append(&mut tmp_src);
        dest_list.append(&mut tmp_dest);
    }
    println!("After com_list creation");

    res_list.append(&mut com_list.iter().map(|x| -> i64 {
        let idx: usize = src_list
                .iter()
                .position(|&n| n == *x)
                .unwrap();
        *dest_list.iter().nth(idx).unwrap()
    }).collect());
    println!("com_list processed!");

    // For elements not found in source, the input is the output
    res_list.append(&mut (inp.0 .. inp.1).collect::<Vec<i64>>().uniq(com_list));
    println!("uniq_list processed ->{:?}", res_list);
    res_list
}

fn get_result_list(inp_list: &Vec<i64>, lines: &Vec<String>) -> Vec<i64> {
    inp_list
    .into_iter()
    .map(|inp| -> i64 {
        let mut dest: i64 = *inp;
        for line in lines {
            let (dest_range, src_range, range_len) =
                sscanf::sscanf!(line, "{i64} {i64} {i64}").unwrap();
            if (src_range..(src_range + range_len)).contains(&inp) {
                let idx = (src_range..(src_range + range_len))
                    .collect::<Vec<i64>>()
                    .iter()
                    .position(|n| n == inp)
                    .unwrap();
                dest = (dest_range..(dest_range + range_len)).nth(idx).unwrap();
                // Match found. Hence break
                break;
            }
        }
        dest
    }).collect()
}

fn get_location_list(seed_info: &SeedInfo) -> i64 {
    let pool = ThreadPool::new(THREAD_POOL_COUNT);
    let shared_data = Arc::new(Mutex::new(vec![]));
    for (start, range) in &seed_info.seeds {
        let begin = *start as i64;
        let end = (start + range) as i64;

        println!("start,range is ({start},{range})");
        let loc_seed_info: SeedInfo = seed_info.clone();
        let shared_data = Arc::clone(&shared_data);
        pool.execute(move || {
            let mut tmp_list = get_result_list_1((begin, end), &loc_seed_info.seed_to_soil_info);
            tmp_list = get_result_list(&tmp_list, &loc_seed_info.soil_to_fert_info);
            tmp_list = get_result_list(&tmp_list, &loc_seed_info.fert_to_water_info);
            tmp_list = get_result_list(&tmp_list, &loc_seed_info.water_to_light_info);
            tmp_list = get_result_list(&tmp_list, &loc_seed_info.light_to_temp_info);
            tmp_list = get_result_list(&tmp_list, &loc_seed_info.temp_to_humid_info);
            tmp_list = get_result_list(&tmp_list, &loc_seed_info.humid_to_loc_info);
            let loc_min: i64 = *tmp_list.iter().min().unwrap();
            println!("Thread: loc_min is {loc_min}");
            let mut data = shared_data.lock().unwrap();
            data.push(loc_min);
        });
    }

    pool.join();

    let data = shared_data.lock().unwrap();
    *data.iter().min().unwrap()
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
        seeds: vec![],
        seed_to_soil_info: vec![],
        soil_to_fert_info: vec![],
        fert_to_water_info: vec![],
        water_to_light_info: vec![],
        light_to_temp_info: vec![],
        temp_to_humid_info: vec![],
        humid_to_loc_info: vec![],
    };

    let mut seed_to_soil_mode: bool = false;
    let mut soil_to_fert_mode: bool = false;
    let mut fert_to_water_mode: bool = false;
    let mut water_to_light_mode: bool = false;
    let mut light_to_temp_mode: bool = false;
    let mut temp_to_humid_mode: bool = false;
    let mut humid_to_loc_mode: bool = false;

    let mut lines: Vec<String> = vec![];

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
            let seed_data: Vec<i64> = seed_line
                .split_whitespace()
                .map(|seed| -> i64 { seed.parse().unwrap() })
                .collect();
            let mut start: i64 = 0;
            for idx in 0..seed_data.len() {
                if idx % 2 == 0 {
                    start = *seed_data.get(idx).unwrap();
                } else {
                    let range: i64 = *seed_data.get(idx).unwrap();
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
    let loc_min = get_location_list(&seed_info);
    println!("loc min is {loc_min}");

    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_common_range() {
        let loc1: (i64, i64) = (50, 97);
        let loc2: (i64, i64) = (52, 99);
        let loc1_list: Vec<i64> = (loc1.0 .. loc1.1).collect();
        let loc2_list: Vec<i64> = (loc2.0 .. loc2.1).collect();
        assert_eq!(loc1_list.intersect(loc2_list), get_common_range(loc1, loc2));
    }
}
