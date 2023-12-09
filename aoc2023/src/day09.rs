use std::collections::VecDeque;

use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        let mut numbers = vec![line
            .split_ascii_whitespace()
            .map(|str| str.parse::<i32>().unwrap())
            .collect::<VecDeque<_>>()];

        let mut i = 0;
        while numbers[i].iter().any(|i| *i != 0) {
            let row = &numbers[i];

            let mut next = VecDeque::with_capacity(row.len() + 1);
            let mut j = 0;
            while j + 1 < row.len() {
                let r0 = row[j];
                let r1 = row[j + 1];
                next.push_back(r1 - r0);

                j += 1;
            }

            numbers.push(next);

            i += 1;
        }

        numbers[i].push_front(0);
        numbers[i].push_back(0);

        while i != 0 {
            let prior_back = *numbers[i - 1].back().unwrap();
            let diff_back = *numbers[i].back().unwrap();
            numbers[i - 1].push_back(prior_back + diff_back);

            let prior_front = *numbers[i - 1].front().unwrap();
            let diff_front = *numbers[i].front().unwrap();
            numbers[i - 1].push_front(prior_front - diff_front);

            i -= 1;
        }

        part1 += *numbers[0].back().unwrap() as i64;
        part2 += *numbers[0].front().unwrap() as i64;
    }

    Solution::from((part1 as u64, part2 as u64))
}
