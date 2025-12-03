use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        let len = line.len();

        let mut first = 0;
        let mut second = 0;
        let mut values = [0; 12];
        for (i, c) in line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .enumerate()
        {
            for j in 0..values.len() {
                let threshold = len - (values.len() - j);
                if c > values[j] && i <= threshold {
                    values[j] = c;
                    for k in j + 1..values.len() {
                        values[k] = 0;
                    }
                    break;
                }
            }

            if c > first && i != len - 1 {
                first = c;
                second = 0;
            } else if c > second {
                second = c;
            }
        }
        part1 += first * 10 + second;
        part2 += values.iter().fold(0, |cur, accum| cur * 10 + accum);
    }

    Solution::from((part1, part2))
}
