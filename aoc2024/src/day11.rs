use std::collections::HashMap;

use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut current = input
        .split_ascii_whitespace()
        .map(|str| (str.parse::<u64>().unwrap(), 1))
        .collect::<HashMap<_, _>>();

    let mut part1 = 0;
    let mut next = HashMap::new();
    for i in 0..75 {
        for (&stone, &count) in current.iter() {
            if stone == 0 {
                *next.entry(1).or_insert(0) += count;
            } else if (stone.ilog10() + 1) % 2 == 0 {
                let digits = stone.ilog10() + 1;
                let left = stone % 10u64.pow(digits / 2);
                let right = stone / 10u64.pow(digits / 2);

                *next.entry(left).or_insert(0) += count;
                *next.entry(right).or_insert(0) += count;
            } else {
                *next.entry(stone * 2024).or_insert(0) += count;
            }
        }
        current.clear();
        std::mem::swap(&mut current, &mut next);

        if i == 24 {
            part1 = current.values().sum::<u64>();
        }
    }
    let part2 = current.values().sum::<u64>();

    Solution::from((part1, part2))
}
