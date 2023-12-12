use std::collections::HashMap;

use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut part1 = 0;
    let mut part2 = 0;
    let mut cache = HashMap::new();
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

        part1 += possible_arrangements(&mut cache, &records, &criteria, 0, 0, 0);
        cache.clear();

        let criteria = criteria.repeat(5);
        records.push(Kind::Unknown);
        records = records.repeat(5);
        records.pop();

        part2 += possible_arrangements(&mut cache, &records, &criteria, 0, 0, 0);
        cache.clear();
    }

    Solution::from((part1, part2))
}

fn possible_arrangements(
    cache: &mut HashMap<(u32, u32, u32), u64>,
    records: &[Kind],
    criteria: &[u32],
    record_idx: usize,
    criteria_i: usize,
    accumulated: u32,
) -> u64 {
    if let Some(&count) = cache.get(&(record_idx as u32, criteria_i as u32, accumulated)) {
        return count;
    }

    if criteria_i == criteria.len() {
        if !records[record_idx..].contains(&Kind::Damaged) {
            return 1;
        }
        return 0;
    }

    let Some(&kind) = records.get(record_idx) else {
        if accumulated == criteria[criteria_i]
            && criteria_i + 1 == criteria.len()
            && !records[record_idx..].contains(&Kind::Damaged)
        {
            return 1;
        }
        return 0;
    };

    if accumulated > criteria[criteria_i] {
        return 0;
    }

    let needed = criteria.iter().skip(criteria_i).sum::<u32>() as usize;
    if needed > records.len() - record_idx + accumulated as usize {
        return 0;
    }

    let result = if kind == Kind::Damaged {
        possible_arrangements(
            cache,
            records,
            criteria,
            record_idx + 1,
            criteria_i,
            accumulated + 1,
        )
    } else if kind == Kind::Operational {
        if accumulated == criteria[criteria_i] {
            possible_arrangements(cache, records, criteria, record_idx + 1, criteria_i + 1, 0)
        } else if accumulated == 0 {
            possible_arrangements(cache, records, criteria, record_idx + 1, criteria_i, 0)
        } else {
            0
        }
    } else {
        // Unknown
        if accumulated == criteria[criteria_i] {
            possible_arrangements(cache, records, criteria, record_idx + 1, criteria_i + 1, 0)
        } else {
            let count = possible_arrangements(
                cache,
                records,
                criteria,
                record_idx + 1,
                criteria_i,
                accumulated + 1,
            );
            if accumulated == 0 {
                count
                    + possible_arrangements(cache, records, criteria, record_idx + 1, criteria_i, 0)
            } else {
                count
            }
        }
    };
    cache.insert((record_idx as u32, criteria_i as u32, accumulated), result);
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Operational,
    Damaged,
    Unknown,
}
