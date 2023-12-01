use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut part1 = 0;
    for line in input.lines() {
        let mut first = 0;
        let mut last = u32::MAX;

        for c in line.chars() {
            if c.is_ascii_digit() {
                first = c.to_digit(10).unwrap();
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                last = c.to_digit(10).unwrap();
                break;
            }
        }

        if last == u32::MAX {
            last = first;
        }
        part1 += u64::from(first * 10 + last);
    }

    let mut part2 = 0;
    for line in input.lines() {
        let ascii_line = line.as_ascii().unwrap();
        let mut first = u32::MAX;
        let mut last = u32::MAX;

        let mut i = 0;
        while i < line.len() && first == u32::MAX {
            match ascii_line[i].to_char() {
                'o' => {
                    if let Some("one") = line.get(i..i + 3) {
                        first = 1;
                    }
                }
                't' => {
                    if let Some("two") = line.get(i..i + 3) {
                        first = 2;
                    }
                    if let Some("three") = line.get(i..i + 5) {
                        first = 3;
                    }
                }
                'f' => {
                    if let Some("four") = line.get(i..i + 4) {
                        first = 4;
                    }
                    if let Some("five") = line.get(i..i + 4) {
                        first = 5;
                    }
                }
                's' => {
                    if let Some("six") = line.get(i..i + 3) {
                        first = 6;
                    }
                    if let Some("seven") = line.get(i..i + 5) {
                        first = 7;
                    }
                }
                'e' => {
                    if let Some("eight") = line.get(i..i + 5) {
                        first = 8;
                    }
                }
                'n' => {
                    if let Some("nine") = line.get(i..i + 4) {
                        first = 9;
                    }
                }
                c @ '1'..='9' => {
                    first = c.to_digit(10).unwrap();
                }
                _ => (),
            }
            i += 1;
        }

        let mut i = line.len();
        while i != 0 && last == u32::MAX {
            i -= 1;

            match ascii_line[i].to_char() {
                'e' => {
                    if let Some("one") = &line.get(i.saturating_sub(2)..=i) {
                        last = 1;
                    } else if let Some("three") = &line.get(i.saturating_sub(4)..=i) {
                        last = 3;
                    } else if let Some("five") = &line.get(i.saturating_sub(3)..=i) {
                        last = 5;
                    } else if let Some("nine") = &line.get(i.saturating_sub(3)..=i) {
                        last = 9;
                    }
                }
                'o' => {
                    if let Some("two") = &line.get(i.saturating_sub(2)..=i) {
                        last = 2;
                    }
                }
                'r' => {
                    if let Some("four") = &line.get(i.saturating_sub(3)..=i) {
                        last = 4;
                    }
                }
                'x' => {
                    if let Some("six") = &line.get(i.saturating_sub(2)..=i) {
                        last = 6;
                    }
                }
                'n' => {
                    if let Some("seven") = &line.get(i.saturating_sub(4)..=i) {
                        last = 7;
                    }
                }
                't' => {
                    if let Some("eight") = &line.get(i.saturating_sub(4)..=i) {
                        last = 8;
                    }
                }
                c @ '1'..='9' => last = c.to_digit(10).unwrap(),
                _ => (),
            }
        }
        part2 += u64::from(first * 10 + last);
    }

    Solution::from((part1, part2))
}
