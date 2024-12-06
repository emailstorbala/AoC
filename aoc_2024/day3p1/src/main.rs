use clap::Parser;
use std::time::Instant;
use std::{fs, i64};
use sscanf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}



fn read_contents(content: String) -> i64 {
    let mut result: i64 = 0;
    for mut line in content.split('\n') {
        if line.is_empty() {
            break;
        }

        let line: &str = &line.replace("mul", " mul");
        for ins in line.split_whitespace() {
            if ins.starts_with("mul") {
                let data = format!("{ins}dummy");
                println!("data is {data}");
                let parsed = sscanf::sscanf!(data, "mul({i64},{i64}){&str}");
                match parsed {
                    Ok((first, second, _)) => {
                        println!("first, second -> {first}, {second}");
                        result += first*second;
                        println!("result is {result}");
                    },
                    Err(e) => println!("Error occured: {e}"),
                }
            }
        }
    }

    println!("The result is {result}");
    result
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let content = fs::read_to_string(&args.file_name).expect("Unable to load the file!");
    let res = read_contents(content);
}
