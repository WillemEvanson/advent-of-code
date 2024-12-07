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
            .collect::<Vec<_>>();

        let mut stack = vec![(numbers[0], 1)];
        while let Some((current, idx)) = stack.pop() {
            if idx != numbers.len() {
                stack.push((current + numbers[idx], idx + 1));
                stack.push((current * numbers[idx], idx + 1));
            } else if current == result {
                part1 += result;
                break;
            }
        }

        let mut stack = vec![(numbers[0], 1)];
        while let Some((current, idx)) = stack.pop() {
            if idx != numbers.len() {
                stack.push((current + numbers[idx], idx + 1));
                stack.push((current * numbers[idx], idx + 1));
                stack.push((
                    current * 10u64.pow(digits(numbers[idx])) + numbers[idx],
                    idx + 1,
                ));
            } else if current == result {
                part2 += result;
                break;
            }
        }
    }

    Solution::from((part1, part2))
}

fn digits(mut n: u64) -> u32 {
    if n == 0 {
        return 1;
    }

    let mut count = 0;
    while n != 0 {
        count += 1;
        n /= 10;
    }
    count
}
