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

        for i in first..=second {
            let log = i.ilog10();
            for splits in 1..=log {
                let power = (log + 1) / (splits + 1);
                if (log + 1) % (splits + 1) == 0 {
                    let start = i % (10u64.pow(power));

                    let mut valid = true;
                    let mut cur = i / (10u64.pow(power));
                    for _ in 0..splits {
                        let next = cur % (10u64.pow(power));
                        if next != start {
                            valid = false;
                            break;
                        }
                        cur = cur / (10u64.pow(power));
                    }

                    if valid {
                        part2 += i;
                        break;
                    } else {
                    }
                }
            }
        }
    }

    Solution::from((part1, part2))
}
