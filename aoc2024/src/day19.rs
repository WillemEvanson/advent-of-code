use std::collections::HashMap;

use util::Solution;

pub fn solve(input: &str) -> Solution {
    let (available, desired) = input.split_once("\n\n").unwrap();
    let available = available.split(", ").collect::<Vec<_>>();
    let desired = desired.lines().collect::<Vec<_>>();

    let mut part1 = 0;
    let mut part2 = 0;
    let mut cache = HashMap::new();
    for desired in desired.iter() {
        let count = find_permutations(&mut cache, 0, desired, &available);
        if count != 0 {
            part1 += 1;
        }
        part2 += count;
        cache.clear();
    }

    Solution::from((part1, part2))
}

fn find_permutations(
    cache: &mut HashMap<usize, u64>,
    i: usize,
    desired: &str,
    available: &[&str],
) -> u64 {
    if i == desired.len() {
        return 1;
    }

    if let Some(&count) = cache.get(&i) {
        return count;
    }

    let mut possible = 0;
    let desired_remaining = &desired[i..];
    for pattern in available.iter() {
        if desired_remaining.starts_with(pattern) {
            possible += find_permutations(cache, i + pattern.len(), desired, available);
        }
    }
    cache.insert(i, possible);
    possible
}
