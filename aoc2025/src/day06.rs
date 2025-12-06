use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut part1 = 0;
    let columns = input
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .count();
    let mut problems = vec![vec![]; columns];
    for line in input.lines() {
        for (i, symbol) in line.split_ascii_whitespace().enumerate() {
            if let Ok(x) = symbol.parse::<u64>() {
                problems[i].push(x);
            } else if symbol == "*" {
                part1 += problems[i].iter().product::<u64>();
            } else if symbol == "+" {
                part1 += problems[i].iter().sum::<u64>();
            }
        }
    }

    let mut part2 = 0;
    let mut is_add = true;
    let mut numbers = Vec::new();
    let lines = input.lines().collect::<Vec<_>>();
    let max_lines = lines.iter().map(|line| line.len()).max().unwrap();

    let op_line = lines.last().unwrap();
    for i in 0..max_lines {
        if let Some(c) = op_line.get(i..i + 1) {
            if c == "+" || c == "*" {
                if is_add {
                    part2 += numbers.iter().sum::<u64>();
                } else {
                    part2 += numbers.iter().product::<u64>();
                }
                numbers.clear();
            }

            if c == "+" {
                is_add = true;
            } else if c == "*" {
                is_add = false;
            }
        }

        let mut number = 0;
        for x in lines.iter().take(lines.len() - 1) {
            let column = &x[i..i + 1];
            if column == " " {
                continue;
            }

            let digit = column.parse::<u64>().unwrap();
            number = number * 10 + digit;
        }
        if number != 0 {
            numbers.push(number);
        }
    }

    if is_add {
        part2 += numbers.iter().sum::<u64>();
    } else {
        part2 += numbers.iter().product::<u64>();
    }

    Solution::from((part1, part2))
}
