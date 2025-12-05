use util::Solution;

pub fn solve(input: &str) -> Solution {
    let (fresh_ranges, available) = input.split_once("\n\n").unwrap();
    let mut fresh_ranges = fresh_ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            (start, end)
        })
        .collect::<Vec<_>>();

    let mut part1 = 0;
    for item in available.lines() {
        let item = item.parse::<u64>().unwrap();

        for &(start, end) in fresh_ranges.iter() {
            if (start..=end).contains(&item) {
                part1 += 1;
                break;
            }
        }
    }

    fresh_ranges.sort_unstable();

    let mut part2 = 0;
    let mut highest = 0;
    for &(start, end) in fresh_ranges.iter() {
        if end <= highest {
            continue;
        }

        let count = if start <= highest {
            end - highest
        } else {
            end - start + 1
        };

        part2 += count;
        highest = end;
    }

    Solution::from((part1, part2))
}
