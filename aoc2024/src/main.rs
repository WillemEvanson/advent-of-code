use std::path::Path;

use aoc2024::*;
use util::Solution;

const FUNCTIONS: [fn(&str) -> Solution; 1] = [day1::solve];

fn main() {
    let Some(input_dir) = std::env::args_os().nth(1) else {
        eprintln!("aoc2024 [input dir]");
        return;
    };

    let path = Path::new(&input_dir);
    for i in 1..=FUNCTIONS.len() {
        let path = path.join(format!("day{i}.txt"));
        if let Ok(input) = std::fs::read_to_string(&path) {
            let now = std::time::Instant::now();
            let solution = FUNCTIONS[i - 1](&input);
            println!("Day {i}:\t{solution}{:?}", now.elapsed());
        }
    }
}
