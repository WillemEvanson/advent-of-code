use aoc2025::*;

#[rustfmt::skip]
const FUNCTIONS: &[fn(&str) -> util::Solution] = &[
    day01::solve,
];

fn main() {
    util::run_year("aoc2025", FUNCTIONS);
}
