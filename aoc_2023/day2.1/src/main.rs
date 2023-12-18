use clap::Parser;
use sscanf;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

struct Game {
    red: i32,
    green: i32,
    blue: i32,
}

fn read_contents(content: String) -> HashMap<i32, Game> {
    let mut game_details: HashMap<i32, Game> = HashMap::new();

    for line in content.split('\n') {
        if line.is_empty() {
            break;
        };

        // Sample -> Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let (game_id, game_data) = sscanf::sscanf!(line, "Game {i32}: {&str}").unwrap();
        // println!("game_id ->{game_id}<-");
        // println!("game_data->{game_data}<-");

        let mut game = Game {
            red: 0,
            green: 0,
            blue: 0,
        };

        for show in game_data.trim().split(';') {
            for data in show.trim().split(',') {
                let (cnt, color) = sscanf::sscanf!(data.trim(), "{i32} {&str}").unwrap();
                match color {
                    "red" => game.red += cnt,
                    "green" => game.green += cnt,
                    "blue" => game.blue += cnt,
                    _ => panic!("Invalid color detail provided!"),
                };
            }
        }

        game_details.insert(game_id, game);
    }

    return game_details;
}

fn get_game_total(game_details: HashMap<i32, Game>) -> i32 {
    let mut game_total: i32 = 0;
    for game_id in game_details.keys() {
        let game = game_details.get(game_id).unwrap();
        // println!("game id is {game_id}");
        // println!("red, green, blue is ({}, {}, {})", game.red, game.green, game.blue);
        if game.red <= 12 && game.green <= 13 && game.blue <= 14 {
            game_total += game_id;
            println!("Matched game : {game_id}");
        }
    }

    game_total
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let content:String = fs::read_to_string(&args.file_name).expect("Unable to load the file!");

    let game_details: HashMap<i32, Game> = read_contents(content);
    let game_total: i32 = get_game_total(game_details);
    println!("Game total is {game_total}");

    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
