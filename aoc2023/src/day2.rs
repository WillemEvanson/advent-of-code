use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut part1 = 0;

    'outer: for line in input.lines() {
        let (id, s) = line.split_once(':').unwrap();
        let id = id
            .trim_matches(|c: char| !c.is_ascii_digit())
            .parse::<u64>()
            .unwrap();

        for round in s.split(';') {
            for element in round.split(',').map(str::trim) {
                let (count, kind) = element.split_once(' ').unwrap();
                let count: u64 = count.parse().unwrap();

                let fail = match kind {
                    "red" => count > 12,
                    "green" => count > 13,
                    "blue" => count > 14,
                    _ => false,
                };
                if fail {
                    continue 'outer;
                }
            }
        }
        part1 += id;
    }

    let mut part2 = 0;
    for line in input.lines() {
        let (_, s) = line.split_once(':').unwrap();

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for round in s.split(';') {
            for element in round.split(',').map(str::trim) {
                let (count, kind) = element.split_once(' ').unwrap();
                let count: u64 = count.parse().unwrap();

                match kind {
                    "red" => {
                        if count > min_red {
                            min_red = count;
                        }
                    }
                    "green" => {
                        if count > min_green {
                            min_green = count;
                        }
                    }
                    "blue" => {
                        if count > min_blue {
                            min_blue = count;
                        }
                    }
                    _ => (),
                };
            }
        }
        part2 += min_red * min_green * min_blue;
    }

    Solution::from((part1, part2))
}
