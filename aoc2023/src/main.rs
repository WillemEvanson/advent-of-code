use aoc2023::*;

#[rustfmt::skip]
const FUNCTIONS: &[fn(&str) -> util::Solution] = &[
    day01::solve,
    day02::solve,
    day03::solve,
    day04::solve,
];

fn main() {
    util::run_year("aoc2023", FUNCTIONS);
}
