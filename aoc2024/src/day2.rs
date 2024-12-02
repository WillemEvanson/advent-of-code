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
        let ascending = nums[0] < nums[1];
        for &[x0, x1] in nums.array_windows() {
            if ascending && x0 >= x1 {
                safe = false;
                break;
            } else if !ascending && x0 <= x1 {
                safe = false;
                break;
            }

            let diff = x0.abs_diff(x1);
            if diff < 1 || 3 < diff {
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

        'part2: for i in 0..nums.len() {
            let mut nums = nums.clone();
            nums.remove(i);

            let ascending = nums[0] < nums[1];
            for &[x0, x1] in nums.array_windows() {
                if ascending && x0 >= x1 {
                    continue 'part2;
                } else if !ascending && x0 <= x1 {
                    continue 'part2;
                }

                let diff = x0.abs_diff(x1);
                if diff < 1 || 3 < diff {
                    continue 'part2;
                }
            }

            part2 += 1;
            break;
        }
    }

    Solution::from((part1, part2))
}
