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

struct Game {
    red: i32,
    green: i32,
    blue: i32,
}

fn read_contents(content: String) -> Vec<Game> {
    let mut all_games: Vec<Game> = Vec::new();
    for line in content.split('\n') {
        if line.is_empty() {
            break;
        };

        let mut game = Game {
            red: 0,
            green: 0,
            blue: 0,
        };
        // Sample -> Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let (_, game_data) = sscanf::sscanf!(line, "Game {i32}: {&str}").unwrap();

        for show in game_data.trim().split(';') {
            for data in show.trim().split(',') {
                let (cnt, color) = sscanf::sscanf!(data.trim(), "{i32} {&str}").unwrap();
                match color {
                    "red" => {
                        if game.red < cnt {
                            game.red = cnt;
                        }
                    }
                    "green" => {
                        if game.green < cnt {
                            game.green = cnt;
                        }
                    }
                    "blue" => {
                        if game.blue < cnt {
                            game.blue = cnt;
                        }
                    }
                    _ => panic!("Invalid color!"),
                }
            }
        }
        all_games.push(game);
    }

    all_games
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let content: String = fs::read_to_string(&args.file_name).expect("Unable to load the file!");

    let all_games: Vec<Game> = read_contents(content);

    let mut answer: i32 = 0;
    for game in all_games {
        //println!("(red, green, blue) is ({}, {}, {})", game.red, game.green, game.blue);
        answer += game.red * game.green * game.blue;
    }

    println!("The answer is {answer}");

    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
