use clap::Parser;
use sscanf;
use std::fs;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

fn read_contents(content: String) -> Vec<i32> {
    let mut matched_game_ids: Vec<i32> = Vec::new();
    for line in content.split('\n') {
        if line.is_empty() {
            break;
        };

        // Sample -> Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let (game_id, game_data) = sscanf::sscanf!(line, "Game {i32}: {&str}").unwrap();
        let mut matched = true;

        for show in game_data.trim().split(';') {
            for data in show.trim().split(',') {
                let (cnt, color) = sscanf::sscanf!(data.trim(), "{i32} {&str}").unwrap();
                if (color == "red" && cnt > 12)
                    || (color == "green" && cnt > 13)
                    || (color == "blue" && cnt > 14)
                {
                    matched = false;
                    break;
                }
            }
        }

        if matched {
            matched_game_ids.push(game_id);
        }
    }

    matched_game_ids
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let content: String = fs::read_to_string(&args.file_name).expect("Unable to load the file!");

    let game_ids: Vec<i32> = read_contents(content);
    let game_id_total: i32 = game_ids.iter().sum();
    println!("Game total is {game_id_total}");

    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
