use aoc2023::*;

#[rustfmt::skip]
const FUNCTIONS: &[fn(&str) -> util::Solution] = &[
    day1::solve,
    day2::solve,
    day3::solve,
    day4::solve,
    day5::solve,
    day6::solve,
    day7::solve,
    day8::solve,
    day9::solve,
    day10::solve,
    day11::solve,
    day12::solve,
    day13::solve,
    day14::solve,
    day15::solve,
    day16::solve,
    day17::solve,
    day18::solve,
    day19::solve,
    day20::solve,
    day21::solve,
    day22::solve,
    day23::solve,
    day24::solve,
    day25::solve,
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
