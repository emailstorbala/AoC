use clap::Parser;
use sscanf;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

struct CrateDetails {
    crate_insts: Vec<(i32, i32, i32)>,
    crate_info: HashMap<i32, Vec<char>>,
}

impl CrateDetails {
    fn new() -> CrateDetails {
        CrateDetails {
            crate_insts: Vec::from([]),
            crate_info: HashMap::from([]),
        }
    }

    fn fill_crate_info(&mut self, info_rec: &str) {
        let mut idx: i32 = 1;
        let mut item_pos: usize = 1;
        loop {
            let item: char = info_rec.chars().nth(item_pos).unwrap();

            if item.is_uppercase() {
                if self.crate_info.contains_key(&idx) {
                    self.crate_info.get_mut(&idx).unwrap().insert(0, item);
                } else {
                    self.crate_info.insert(idx, [item].to_vec());
                }
            }

            if (item_pos + 4) < info_rec.len() {
                item_pos += 4;
                idx += 1;
            } else {
                break;
            }
        }
    }

    fn fill_crate_instr(&mut self, ins_rec: &str) {
        let (num_of_crate, from_pos, to_pos): (i32, i32, i32) =
            sscanf::sscanf!(ins_rec, "move {i32} from {i32} to {i32}").unwrap();
        self.crate_insts.push((num_of_crate, from_pos, to_pos));
    }

    fn execute_instructions(&mut self) {
        for (num_of_crates, from_pos, to_pos) in self.crate_insts[..].to_owned() {
            let tmp_idx: usize = self.crate_info.get_mut(&from_pos).unwrap().len() - num_of_crates as usize;
            let mut tmp_crates: Vec<char> = self.crate_info.get_mut(&from_pos).unwrap().drain(tmp_idx..).collect();
            self.crate_info.get_mut(&to_pos).unwrap().append(&mut tmp_crates);
        }
    }

    fn get_result(&self) -> String {
        let mut res: String = "".to_string();
        for idx in 1..=(self.crate_info.len() as i32) {
            let item: char = *self.crate_info[&idx].last().unwrap();
            res.push(item);
        }

        res
    }
}

fn get_all_crate_dtls(contents: &str) -> CrateDetails {
    let mut crate_dtls: CrateDetails = CrateDetails::new();
    for record in contents.split('\n') {
        if !record.is_empty() {
            let first_chr = record.chars().nth(0).unwrap();
            match first_chr {
                '[' | ' ' => {
                    crate_dtls.fill_crate_info(record);
                }
                'm' => {
                    crate_dtls.fill_crate_instr(record);
                }
                _ => {}
            }
        }
    }

    crate_dtls
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let contents = fs::read_to_string(&args.file_name).expect("Unable to load the file!");
    let mut crate_dtls: CrateDetails = get_all_crate_dtls(&contents);

    crate_dtls.execute_instructions();

    let result: &str = &crate_dtls.get_result().to_owned();
    println!("Result is {result}");

    let duration = start_time.elapsed();
    println!("Total time taken -> {:?} ", duration);
}
