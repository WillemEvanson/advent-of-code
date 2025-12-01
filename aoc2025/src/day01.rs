use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut part1 = 0;
    let mut part2 = 0;
    let mut current: i32 = 50;
    for line in input.lines() {
        let dir = line.chars().next().unwrap();
        let count = line
            .trim_matches(char::is_alphabetic)
            .parse::<u32>()
            .unwrap() as i32;
        match dir {
            'L' => {
                let hundreds = count / 100;
                let rem = count % 100;

                if current <= rem && current != 0 {
                    part2 += 1;
                }
                part2 += hundreds as u64;

                current = (current - count).rem_euclid(100);
            }
            'R' => {
                let hundreds = count / 100;
                let rem = count % 100;

                if (99 - current) < rem {
                    part2 += 1;
                }
                part2 += hundreds as u64;

                current = (current + count).rem_euclid(100);
            }
            _ => panic!(),
        };
        if current == 0 {
            part1 += 1;
        }
    }

    Solution::from((part1, part2))
}
