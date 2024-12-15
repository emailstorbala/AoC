use clap::Parser;
use indexmap::IndexMap;
use std::time::Instant;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

fn read_contents(content: String) -> IndexMap<(usize, usize), char> {
    let mut puz_input: IndexMap<(usize, usize), char> = IndexMap::new();
    let mut row: usize = 0;
    for line in content.split('\n') {
        if line.is_empty() {
            break;
        }

        for (col, chr) in line.chars().enumerate() {
            puz_input.insert((row, col), chr);

        }
        row += 1;
    }

    puz_input
}

fn get_xmas_count(puz_input: IndexMap<(usize, usize), char>) -> i32 {
    let mut xmas_count: i32 = 0;

    for (pos, chr) in &puz_input {
        if *chr == 'A' {
            let mut lr_diag: String = "".to_string();
            // Gather LR diagonal
            if puz_input.contains_key(&(pos.0 - 1, pos.1 - 1)) {
                let tmp_chr = puz_input.get(&(pos.0 - 1, pos.1 - 1)).unwrap();
                lr_diag.push(*tmp_chr);
            } else { continue }

            lr_diag.push(*chr);

            if puz_input.contains_key(&(pos.0 + 1, pos.1 + 1)) {
                let tmp_chr = puz_input.get(&(pos.0 + 1, pos.1 + 1)).unwrap();
                lr_diag.push(*tmp_chr);
            } else { continue }

            // Gather RL diagonal
            let mut rl_diag: String = "".to_string();
            if puz_input.contains_key(&(pos.0 - 1, pos.1 + 1)) {
                let tmp_chr = puz_input.get(&(pos.0 - 1, pos.1 + 1)).unwrap();
                rl_diag.push(*tmp_chr);
            } else { continue }
            rl_diag.push(*chr);

            if puz_input.contains_key(&(pos.0 + 1, pos.1 - 1)) {
                let tmp_chr = puz_input.get(&(pos.0 + 1, pos.1 - 1)).unwrap();
                rl_diag.push(*tmp_chr);
            } else { continue }

            // println!("lr_diag: {lr_diag}");
            // println!("rl_diag: {rl_diag}");
            // println!("pos is {:?}", pos);

            if (lr_diag == "MAS" || lr_diag == "SAM") && (rl_diag == "MAS" || rl_diag == "SAM") { xmas_count += 1 }
        }
    }

    xmas_count
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let content = fs::read_to_string(&args.file_name).expect("Unable to load the file!");
    let inp_data = read_contents(content);
    // println!("The input data is {:?}", inp_data);
    let xmas_count = get_xmas_count(inp_data);
    println!("xmas_count is {xmas_count}");
    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
