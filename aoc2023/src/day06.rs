use util::Solution;

pub fn solve(input: &str) -> Solution {
    let (times, distances) = input.split_once('\n').unwrap();
    let times = times
        .trim_matches(|c: char| !c.is_ascii_digit())
        .split_ascii_whitespace()
        .map(|str| str.parse::<u64>().unwrap());
    let distances = distances
        .trim_matches(|c: char| !c.is_ascii_digit())
        .split_ascii_whitespace()
        .map(|str| str.parse::<u64>().unwrap());

    let mut part1 = 1;
    for (time, distance) in times.zip(distances) {
        let (x1, x2) = solve_quadratic(time, distance);
        part1 *= x2.floor() as u64 - x1.ceil() as u64 + 1;
    }

    let (times, distances) = input.split_once('\n').unwrap();
    let time = times
        .chars()
        .filter(char::is_ascii_digit)
        .fold(0, |count, digit| {
            count * 10 + digit.to_digit(10).unwrap() as u64
        });
    let distance = distances
        .chars()
        .filter(char::is_ascii_digit)
        .fold(0, |count, digit| {
            count * 10 + digit.to_digit(10).unwrap() as u64
        });

    let (x1, x2) = solve_quadratic(time, distance);
    let part2 = x2.floor() as u64 - x1.ceil() as u64 + 1;

    Solution::from((part1, part2))
}

fn solve_quadratic(t: u64, d: u64) -> (f64, f64) {
    let under = t * t - 4 * d;
    let sqrt = f64::sqrt(under as f64);
    ((t as f64 - sqrt) / 2.0, (t as f64 + sqrt) / 2.0)
}
