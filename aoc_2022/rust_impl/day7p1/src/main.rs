use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
    #[arg(short, long)]
    exp_size: u64,
}

fn get_directory_size(dir: &str, dir_dtls_map: &HashMap<String, Vec<String>>) -> u64 {
    let mut dir_size: u64 = 0;
    let dir_dtls: Vec<String> = dir_dtls_map.get(dir).unwrap().to_vec();

    for item in dir_dtls {
        if item.starts_with("dir ") {
            // Directory case
            let tmp_dir: String = item
                .split_whitespace()
                .nth(1)
                .unwrap()
                .to_owned()
                .to_string();
            let abs_dir: String = if dir.ends_with('/') {
                format!("{}{}", dir, tmp_dir)
            } else {
                format!("{}/{}", dir, tmp_dir)
            };

            dir_size += get_directory_size(&abs_dir, dir_dtls_map);
        } else {
            // File case
            let file_size: u64 = item
                .split_whitespace()
                .nth(0)
                .unwrap()
                .parse::<u64>()
                .unwrap();
            dir_size += file_size;
        }
    }

    dir_size
}

fn read_contents(content: &str) -> HashMap<String, Vec<String>> {
    let mut curr_dir: String = "".to_string();

    let mut dir_dtls: Vec<String> = Vec::new();
    let mut dir_dtls_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut dir_stack: Vec<String> = Vec::new();

    for line in content.split('\n') {
        if line.is_empty() {
            break;
        }

        if line.starts_with('$') {
            // This is a command
            if line.starts_with("$ cd ") {
                if !dir_dtls.is_empty() {
                    dir_dtls_map.insert(curr_dir.clone(), dir_dtls.to_owned());
                }
                dir_dtls.clear();

                // Change directory
                let prev_dir: String = curr_dir.to_owned();
                let tmp_dir: String = line.split_whitespace().nth(2).unwrap().to_string();
                match tmp_dir.as_str() {
                    "/" => {
                        curr_dir = "/".to_string();
                        dir_stack.push(curr_dir.clone());
                    },
                    ".." => {
                        dir_stack.pop().unwrap();
                        curr_dir = dir_stack[dir_stack.len() - 1].to_string();
                    },
                    _ => {
                        if prev_dir.ends_with("/") {
                            curr_dir = format!("{}{}", prev_dir, tmp_dir);
                        } else {
                            curr_dir = format!("{}/{}", prev_dir, tmp_dir);
                        }
                        dir_stack.push(curr_dir.clone());
                    }
                }
            }
            continue;
        }

        // println!("list content line is {line}");
        dir_dtls.push(line.to_owned().to_string());
    }

    if !dir_dtls.is_empty() {
        dir_dtls_map.insert(curr_dir, dir_dtls.to_owned());
    }

    dir_dtls_map
}

fn get_dirs_to_match_expected(dir_dtls_map: &HashMap<String, Vec<String>>, exp_size: u64) -> u64 {
    let mut total_size_of_matching_dirs: u64 = 0;
    for (dir_name, _) in dir_dtls_map {
        let dir_size: u64 = get_directory_size(&dir_name, &dir_dtls_map);
        if dir_name != "/" && dir_size <= exp_size {
            // println!("{dir_name} : {dir_size}");
            total_size_of_matching_dirs += dir_size;
        }
    }

    total_size_of_matching_dirs
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let content = fs::read_to_string(&args.file_name).expect("Unable to load the file!");

    let dir_dtls_map: HashMap<String, Vec<String>> = read_contents(&content);
    // println!("dir_dtls_map -> {:?}", dir_dtls_map);
    let result: u64 = get_dirs_to_match_expected(&dir_dtls_map, args.exp_size);
    println!("Result is {result}");

    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
