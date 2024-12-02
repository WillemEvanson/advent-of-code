use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut lefts = Vec::new();
    let mut rights = Vec::new();

    for line in input.lines() {
        let (left, right) = line.split_once("   ").unwrap();
        let left = left.parse::<u32>().unwrap();
        let right = right.parse::<u32>().unwrap();

        lefts.push(left);
        rights.push(right);
    }

    lefts.sort_unstable();
    rights.sort_unstable();

    let mut part1 = 0;
    for (&left, &right) in lefts.iter().zip(rights.iter()) {
        part1 += right.abs_diff(left) as u64;
    }

    let mut i = 0;
    let mut j = 0;
    let mut part2 = 0;
    while i < lefts.len() {
        // Count number of equal numbers on the left
        let mut left_count = 1;
        while i + 1 < lefts.len() && lefts[i] == lefts[i + 1] {
            left_count += 1;
            i += 1;
        }

        // Move forward to matching numbers
        while rights[j] < lefts[i] {
            j += 1;
        }

        // Count number of matching numbers on the right
        let mut right_count = 0;
        while rights[j] == lefts[i] {
            right_count += 1;
            j += 1;
        }

        part2 += left_count * (lefts[i] as u64 * right_count);
        i += 1;
    }

    Solution::from((part1, part2))
}
