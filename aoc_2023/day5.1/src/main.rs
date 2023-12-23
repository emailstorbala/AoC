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

type SeedToSoilMap = HashMap<i64, i64>;
type SoilToFertMap = HashMap<i64, i64>;
type FertToWaterMap = HashMap<i64, i64>;
type WaterToLightMap = HashMap<i64, i64>;
type LightToTempMap = HashMap<i64, i64>;
type TempToHumidMap = HashMap<i64, i64>;
type HumidToLocMap = HashMap<i64, i64>;

struct Seeds {
    seeds: Vec<i64>,
    seed_to_soil_map: SeedToSoilMap,
    soil_to_fert_map: SoilToFertMap,
    fert_to_water_map: FertToWaterMap,
    water_to_light_map: WaterToLightMap,
    light_to_temp_map: LightToTempMap,
    temp_to_humid_map: TempToHumidMap,
    humid_to_loc_map: HumidToLocMap,
}

fn read_contents(content: String) -> Seeds {
    let mut seeds = Seeds {
        seeds: Vec::new(),
        seed_to_soil_map: HashMap::new(),
        soil_to_fert_map: HashMap::new(),
        fert_to_water_map: HashMap::new(),
        water_to_light_map: HashMap::new(),
        light_to_temp_map: HashMap::new(),
        temp_to_humid_map: HashMap::new(),
        humid_to_loc_map: HashMap::new(),
    };

    let mut seed_to_soil_mode: bool = false;
    let mut soil_to_fert_mode: bool = false;
    let mut fert_to_water_mode: bool = false;
    let mut water_to_light_mode: bool = false;
    let mut light_to_temp_mode: bool = false;
    let mut temp_to_humid_mode: bool = false;
    let mut humid_to_loc_mode: bool = false;

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
                        // println!("src_range->{src_range}");
                        // println!("dest_range->{dest_range}");
                        // println!("src_list->{:?}", src_list);
                        // println!("dest_list->{:?}", dest_list);
                        println!("Preparing zip ...");
                        for (seed, soil) in zip(src_list, dest_list) {
                            // println!("seed, soil -> ({seed}, {soil})");
                            seeds.seed_to_soil_map.insert(seed, soil);
                        }
                        println!("Seed soil mode completed");
                    } else if soil_to_fert_mode {
                        println!("Soil Fertilizer mode started ...");
                        for (soil, fert) in zip(src_list, dest_list) {
                            seeds.soil_to_fert_map.insert(soil, fert);
                        }
                        println!("Soil Fertilizer mode completed");
                    } else if fert_to_water_mode {
                        for (fert, water) in zip(src_list, dest_list) {
                            seeds.fert_to_water_map.insert(fert, water);
                        }
                        println!("Fertilizer water mode completed");
                    } else if water_to_light_mode {
                        for (water, light) in zip(src_list, dest_list) {
                            seeds.water_to_light_map.insert(water, light);
                        }
                    } else if light_to_temp_mode {
                        for (light, temp) in zip(src_list, dest_list) {
                            seeds.light_to_temp_map.insert(light, temp);
                        }
                    } else if temp_to_humid_mode {
                        for (temp, humid) in zip(src_list, dest_list) {
                            seeds.temp_to_humid_map.insert(temp, humid);
                        }
                    } else if humid_to_loc_mode {
                        for (humid, loc) in zip(src_list, dest_list) {
                            seeds.humid_to_loc_map.insert(humid, loc);
                        }
                    } else {
                        panic!("Invalid mode detected!");
                    }
                }
            }
        }

    }
    //println!("Seeds info is {:?}", seeds.seeds);

    seeds
}

fn collect_loc_data(seeds: Seeds) -> Vec<i64> {
    let mut soil_list: Vec<i64> = Vec::new();
    let mut fert_list: Vec<i64> = Vec::new();
    let mut water_list: Vec<i64> = Vec::new();
    let mut light_list: Vec<i64> = Vec::new();
    let mut temp_list: Vec<i64> = Vec::new();
    let mut humid_list: Vec<i64> = Vec::new();
    let mut loc_list: Vec<i64> = Vec::new();

    // Collect soil list
    for seed in seeds.seeds {
        let soil: i64 = if seeds.seed_to_soil_map.contains_key(&seed) {
            *seeds.seed_to_soil_map.get(&seed).unwrap()
        } else {
            seed
        };
        soil_list.push(soil);
    }

    // Collect fertilizer list
    for soil in &soil_list {
        let fert: i64 = if seeds.soil_to_fert_map.contains_key(&soil) {
            *seeds.soil_to_fert_map.get(&soil).unwrap()
        } else {
           *soil
        };
        fert_list.push(fert);
    }

    // Collect water list
    for fert in &fert_list {
        let water: i64 = if seeds.fert_to_water_map.contains_key(&fert) {
            *seeds.fert_to_water_map.get(&fert).unwrap()
        } else {
            *fert 
        };
        water_list.push(water);
    }

    // Collect light list
    for water in &water_list {
        let light: i64 = if seeds.fert_to_water_map.contains_key(&water) {
            *seeds.water_to_light_map.get(&water).unwrap()
        } else {
            *water
        };
        light_list.push(light);
    }

    // Collect temperature list
    for light in &light_list {
        let temp: i64 = if seeds.light_to_temp_map.contains_key(&light) {
            *seeds.light_to_temp_map.get(&light).unwrap()
        } else {
            *light
        };
        temp_list.push(temp);
    }

    // Collect humidity list
    for temp in &temp_list {
        let humid: i64 = if seeds.temp_to_humid_map.contains_key(&temp) {
            *seeds.temp_to_humid_map.get(&temp).unwrap()
        } else {
            *temp
        };
        humid_list.push(humid);
    }

    // Collect location list
    for humid in &humid_list {
        let loc: i64 = if seeds.humid_to_loc_map.contains_key(&humid) {
            *seeds.humid_to_loc_map.get(&humid).unwrap()
        } else {
            *humid
        };
        loc_list.push(loc);
    }

    loc_list
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let content: String = fs::read_to_string(&args.file_name).expect("Unable to load the file!");

    println!("Before loading the input");
    let seeds = read_contents(content);
    println!("After loading the input");
    println!("Seeds is {:?}", seeds.seeds);
    // let loc_list = collect_loc_data(seeds);
    // println!("Loc list is {:?}", loc_list);
    // let min_loc:i64 = *loc_list.iter().min().unwrap();
    //
    // println!("The answer is {}", min_loc);

    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
