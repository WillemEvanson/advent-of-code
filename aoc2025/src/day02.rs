use std::collections::HashSet;

use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut part1 = 0;
    for range in input.split(',').map(str::trim) {
        let (first, second) = range.split_once('-').unwrap();
        let first = first.parse::<u64>().unwrap();
        let second = second.parse::<u64>().unwrap();

        for i in first..=second {
            let log = i.ilog10();
            if (log + 1) % 2 != 0 {
                continue;
            }

            let power = (log + 1) / 2;
            let start = i / (10u64.pow(power));
            let end = i % (10u64.pow(power));
            if start == end {
                part1 += i;
            }
        }
    }

    let mut part2 = 0;
    for range in input.split(',').map(str::trim) {
        let (first, second) = range.split_once('-').unwrap();
        let first = first.parse::<u64>().unwrap();
        let second = second.parse::<u64>().unwrap();

        // We turn this into a set of intervals with start and end having the same
        // number of digits.
        let first_digits = first.ilog10() + 1;
        let second_digits = second.ilog10() + 1;

        let mut digits = first_digits;
        let mut found = HashSet::new();
        loop {
            let min_tens = 10u64.pow(digits - 1);
            let max_tens = min_tens * 10;

            let start = u64::max(min_tens, first);
            let end = u64::min(max_tens - 1, second);

            for splits in 1..digits {
                // Check if it can't be split into equal-sized groups
                if digits % (splits + 1) != 0 {
                    continue;
                }

                let power = digits / (splits + 1);
                let to_power = 10u64.pow(power);

                // Segment the digits of the top
                let mut cur = end;
                let mut top = vec![0; splits as usize + 1];
                for i in (0..=splits as usize).rev() {
                    let next = cur % to_power;
                    top[i] = next;
                    cur /= to_power;
                }

                // Segment the digits of the bottom
                let mut cur = start;
                let mut bottom = vec![0; splits as usize + 1];
                for i in (0..=splits as usize).rev() {
                    let next = cur % to_power;
                    bottom[i] = next;
                    cur /= to_power;
                }

                // Determine whether each starting segment results in an invalid ID.
                for j in bottom[0]..=top[0] {
                    let mut free_bottom = bottom[0] != j;
                    let mut free_top = top[0] != j;

                    let mut valid = true;
                    for i in 1..bottom.len() {
                        if !free_bottom && j < bottom[i] {
                            valid = false;
                            break;
                        } else if !free_top && j > top[i] {
                            valid = false;
                            break;
                        }

                        free_bottom = free_bottom || bottom[i] < j;
                        free_top = free_top || top[i] > j;
                    }

                    if valid {
                        let number = (0..splits as u64).fold(j, |accum, _| accum * to_power + j);

                        if found.insert(number) {
                            if splits == 1 {
                                part1 += number;
                            }

                            part2 += number;
                        }
                    }
                }
            }

            if digits == second_digits {
                break;
            }
            digits += 1;
        }
    }

    Solution::from((part1, part2))
}
