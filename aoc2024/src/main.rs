use aoc2024::*;

#[rustfmt::skip]
const FUNCTIONS: &[fn(&str) -> util::Solution] = &[
    day01::solve,
];

fn main() {
    util::run_year("aoc2024", FUNCTIONS);
}
