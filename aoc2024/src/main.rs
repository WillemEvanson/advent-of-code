use aoc2024::*;

#[rustfmt::skip]
const FUNCTIONS: &[fn(&str) -> util::Solution] = &[
    day1::solve,
    day2::solve,
    day3::solve,
];

fn main() {
    let Some(input_dir) = std::env::args_os().nth(1) else {
        eprintln!("aoc2023 [input dir]");
        return;
    };

    let path = std::path::Path::new(&input_dir);
    let mut total_duration = std::time::Duration::default();
    for i in 1..=FUNCTIONS.len() {
        let path = path.join(format!("day{i}.txt"));
        if let Ok(input) = std::fs::read_to_string(&path) {
            let now = std::time::Instant::now();
            let solution = FUNCTIONS[i - 1](&input);
            let elapsed = now.elapsed();
            total_duration += elapsed;

            println!("Day {i}:\t{solution}{elapsed:?}");
        }
    }

    println!("Total:\t{:<40}{total_duration:?}", "");
}
