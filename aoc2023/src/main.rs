use aoc2023::*;

#[rustfmt::skip]
const FUNCTIONS: &[fn(&str) -> util::Solution] = &[
    day01::solve,
    day02::solve,
    day03::solve,
    day04::solve,
    day05::solve,
    day06::solve,
    day07::solve,
    day08::solve,
    day09::solve,
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
];

fn main() {
    util::run_year("aoc2023", FUNCTIONS);
}
