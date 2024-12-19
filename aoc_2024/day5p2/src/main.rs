use clap::Parser;
use indexmap::IndexMap;
use std::fs;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

struct Input {
    rules: Vec<(i32, i32)>,
    pages: IndexMap<i32, Vec<i32>>,
}

fn read_contents(content: String) -> Input {
    let mut input: Input = Input {
        rules: Vec::new(),
        pages: IndexMap::new(),
    };
    let mut page_cnt = 0;

    for line in content.split('\n') {
        if line.is_empty() {
            continue;
        }

        if line.contains('|') {
            // This is a rule line
            let (first, second) = sscanf::sscanf!(line, "{i32}|{i32}").unwrap();
            input.rules.push((first, second));
        } else {
            let tmp_page: Vec<i32> = line
                .split(',')
                .map(|tmps| -> i32 { tmps.parse::<i32>().unwrap() })
                .collect();
            input.pages.insert(page_cnt, tmp_page);
            page_cnt += 1;
        }
    }

    input
}

fn get_correct_pages(input: Input) -> Vec<Vec<i32>> {
    let mut correct_pages: Vec<Vec<i32>> = Vec::new();
    for (_, orig_page) in input.pages {
        let mut page = orig_page.clone();
        let mut changed = true;
        while changed {
            changed = false;
            for (first, second) in &input.rules {
                if !page.contains(first) || !page.contains(second) {
                    // If the rule number doesn't exist in the page, then the rule doesn't apply
                    continue;
                }
                let first_pos = page.iter().position(|n| n == first).unwrap();
                let second_pos = page.iter().position(|n| n == second).unwrap();
                if first_pos > second_pos {
                    page.swap(first_pos, second_pos);
                    changed = true;
                }
            }
        }

        if orig_page != page {
            correct_pages.push(page);
        }
    }

    // println!("correct_pages: {:?}", correct_pages);
    correct_pages
}

fn get_middle_sum(pages: Vec<Vec<i32>>) -> i32 {
    pages
        .iter()
        .map(|page| -> i32 {
            let middle = page.len() / 2;
            *page.get(middle).unwrap()
        })
        .sum()
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let content = fs::read_to_string(&args.file_name).expect("Unable to load the file!");
    let input = read_contents(content);
    // println!("input rules is {:?}", input.rules);
    // println!("input pages is {:?}", input.pages);
    let correct_pages = get_correct_pages(input);
    // println!("correct pages are {:?}", correct_pages);
    let middle_sum = get_middle_sum(correct_pages);
    println!("The middle sum is {middle_sum}");
    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
