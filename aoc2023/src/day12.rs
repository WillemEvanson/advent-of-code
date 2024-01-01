use std::collections::HashMap;

use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut part1 = 0;
    let mut part2 = 0;

    let mut hashmap = HashMap::with_capacity(100_000);
    for line in input.lines() {
        let (records, criteria) = line.split_once(' ').unwrap();
        let criteria = criteria
            .split(',')
            .map(|str| str.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let mut iter = records.chars();
        let mut records = Vec::new();
        while let Some(c) = iter.next() {
            records.push(match c {
                '.' => {
                    while let Some('.') = iter.clone().next() {
                        iter.next();
                    }
                    Kind::Operational
                }
                '#' => Kind::Damaged,
                '?' => Kind::Unknown,
                _ => panic!(),
            });
        }

        part1 += calc_rec(&mut hashmap, &records, &criteria, 0, 0, 0);
        hashmap.clear();

        let criteria = criteria.repeat(5);
        records.push(Kind::Unknown);
        records = records.repeat(5);
        records.pop();

        part2 += calc_rec(&mut hashmap, &records, &criteria, 0, 0, 0);
        hashmap.clear();
    }

    Solution::from((part1, part2))
}

fn calc_rec(
    cache: &mut HashMap<(usize, u32, usize), u64>,
    records: &[Kind],
    criteria: &[u32],
    i: usize,
    accum: u32,
    crit_i: usize,
) -> u64 {
    if let Some(&count) = cache.get(&(i, accum, crit_i)) {
        return count;
    }

    if crit_i == criteria.len() {
        if !records[i..].contains(&Kind::Damaged) {
            return 1;
        }
        return 0;
    }
    let Some(&kind) = records.get(i) else {
        if accum == criteria[crit_i] {
            if crit_i + 1 == criteria.len() {
                if !records[i..].contains(&Kind::Damaged) {
                    return 1;
                }
            }
        }
        return 0;
    };
    if accum > criteria[crit_i] {
        return 0;
    }
    let needed = criteria.iter().skip(crit_i).sum::<u32>() as usize;
    if needed > records.len() - i + accum as usize {
        return 0;
    }

    let result = if kind == Kind::Damaged {
        calc_rec(cache, records, criteria, i + 1, accum + 1, crit_i)
    } else if kind == Kind::Operational {
        if accum == criteria[crit_i] {
            calc_rec(cache, records, criteria, i + 1, 0, crit_i + 1)
        } else if accum == 0 {
            calc_rec(cache, records, criteria, i + 1, 0, crit_i)
        } else {
            0
        }
    } else {
        if accum == criteria[crit_i] {
            calc_rec(cache, records, criteria, i + 1, 0, crit_i + 1)
        } else {
            let count = calc_rec(cache, records, criteria, i + 1, accum + 1, crit_i);
            if accum == 0 {
                count + calc_rec(cache, records, criteria, i + 1, 0, crit_i)
            } else {
                count
            }
        }
    };
    cache.insert((i, accum, crit_i), result);
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Operational,
    Damaged,
    Unknown,
}
