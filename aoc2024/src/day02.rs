use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        let nums = line
            .split_ascii_whitespace()
            .map(|str| str.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let mut safe = true;
        let mut safe_until = 0;
        let ascending = nums[0] < nums[1];
        for (i, &[x0, x1]) in nums.array_windows().enumerate() {
            let diff = x0.abs_diff(x1);
            if (ascending && x0 >= x1) || (!ascending && x0 <= x1) || !(1..=3).contains(&diff) {
                safe_until = i;
                safe = false;
                break;
            }
        }

        if safe {
            // Both parts are fine if the report is safe without removing levels
            part1 += 1;
            part2 += 1;
            continue;
        }

        'part2: for i in safe_until.saturating_sub(1)..=usize::min(safe_until + 1, nums.len()) {
            let mut nums = nums.clone();
            nums.remove(i);

            let ascending = nums[0] < nums[1];
            for &[x0, x1] in nums.array_windows() {
                let diff = x0.abs_diff(x1);
                if (ascending && x0 >= x1) || (!ascending && x0 <= x1) || !(1..=3).contains(&diff) {
                    continue 'part2;
                }
            }

            part2 += 1;
            break;
        }
    }

    Solution::from((part1, part2))
}
