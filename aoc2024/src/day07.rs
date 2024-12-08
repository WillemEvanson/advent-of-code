use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        let (result, rest) = line.split_once(':').unwrap();
        let result = result.parse::<u64>().unwrap();
        let numbers = rest
            .trim()
            .split_ascii_whitespace()
            .map(|str| str.parse::<u64>().unwrap())
            .rev()
            .collect::<Vec<_>>();

        let mut stack = vec![(result, 0)];
        while let Some((current, idx)) = stack.pop() {
            if idx < numbers.len() - 1 {
                if current % numbers[idx] == 0 {
                    stack.push((current / numbers[idx], idx + 1));
                }
                if current > numbers[idx] {
                    stack.push((current - numbers[idx], idx + 1))
                }
            } else if current == numbers[idx] {
                part1 += result;
                break;
            }
        }

        let mut stack = vec![(result, 0)];
        while let Some((current, idx)) = stack.pop() {
            if idx < numbers.len() - 1 {
                if current % numbers[idx] == 0 {
                    stack.push((current / numbers[idx], idx + 1));
                }
                if current > numbers[idx] {
                    stack.push((current - numbers[idx], idx + 1))
                }

                if let Some(new_goal) = current.checked_sub(numbers[idx]) {
                    let tens = 10u64.pow(numbers[idx].ilog10() + 1);
                    if new_goal % tens == 0 {
                        stack.push((new_goal / tens, idx + 1));
                    }
                }
            } else if current == numbers[idx] {
                part2 += result;
                break;
            }
        }
    }

    Solution::from((part1, part2))
}
