use std::collections::HashMap;

use util::Solution;

pub fn solve(input: &str) -> Solution {
    let (rules, inputs) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once('{').unwrap();

            let mut conditions = Vec::new();
            for rule in rest[..rest.len() - 1].split(',') {
                if rule.contains(':') {
                    let (condition, workflow) = rule.split_once(':').unwrap();

                    if let Some((var, number)) = condition.split_once('<') {
                        conditions.push((
                            Some((var, Comparison::LessThan, number.parse::<u64>().unwrap())),
                            workflow,
                        ))
                    } else if let Some((var, number)) = condition.split_once('>') {
                        conditions.push((
                            Some((var, Comparison::GreaterThan, number.parse::<u64>().unwrap())),
                            workflow,
                        ))
                    } else {
                        panic!();
                    }
                } else {
                    conditions.push((None, rule))
                }
            }
            (name, conditions)
        })
        .collect::<HashMap<_, _>>();

    let mut part1 = 0;
    for input in inputs.lines() {
        let input = &input[1..input.len() - 1];
        let variables = input
            .split(',')
            .map(|str| str.split_once('=').unwrap())
            .map(|(var, num)| (var, num.parse::<u64>().unwrap()))
            .collect::<HashMap<_, _>>();

        let mut current = "in";
        while !matches!(current, "A" | "R") {
            let conditions = rules.get(current).unwrap();
            for &(condition, next) in conditions {
                if let Some((variable, comparison, number)) = condition {
                    let result = match comparison {
                        Comparison::GreaterThan => *variables.get(variable).unwrap() > number,
                        Comparison::LessThan => *variables.get(variable).unwrap() < number,
                    };
                    if result {
                        current = next;
                        break;
                    }
                } else {
                    current = next;
                    break;
                }
            }
        }

        if current == "A" {
            part1 += variables.values().sum::<u64>() as u64;
        }
    }

    let mut part2 = 0;
    let mut ranges = vec![(
        "in",
        [
            Range::new(1, 4000),
            Range::new(1, 4000),
            Range::new(1, 4000),
            Range::new(1, 4000),
        ],
    )];
    while let Some((current, inputs)) = ranges.pop() {
        if current == "A" {
            part2 += inputs
                .iter()
                .map(|range| range.len() as u64)
                .product::<u64>();
            continue;
        } else if current == "R" {
            continue;
        }

        let conditions = rules.get(current).unwrap();
        for &(condition, next) in conditions {
            if let Some((variable, comparison, number)) = condition {
                let range_idx = match variable {
                    "x" => 0,
                    "m" => 1,
                    "a" => 2,
                    "s" => 3,
                    _ => panic!(),
                };
                let range = inputs[range_idx];

                if let Comparison::LessThan = comparison {
                    if range.end < number {
                        ranges.push((next, inputs));
                    } else if range.start < number {
                        let invalid = Range::new(number, range.end);
                        let valid = Range::new(range.start, number - 1);

                        let mut invalid_inputs = inputs;
                        invalid_inputs[range_idx] = invalid;
                        ranges.push((current, invalid_inputs));

                        let mut valid_inputs = inputs;
                        valid_inputs[range_idx] = valid;
                        ranges.push((next, valid_inputs));
                    } else {
                        continue;
                    }
                } else {
                    if range.start > number {
                        ranges.push((next, inputs));
                    } else if range.end > number {
                        let invalid = Range::new(range.start, number);
                        let valid = Range::new(number + 1, range.end);

                        let mut invalid_inputs = inputs;
                        invalid_inputs[range_idx] = invalid;
                        ranges.push((current, invalid_inputs));

                        let mut valid_inputs = inputs;
                        valid_inputs[range_idx] = valid;
                        ranges.push((next, valid_inputs));
                    } else {
                        continue;
                    }
                }
                break;
            } else {
                ranges.push((next, inputs));
                break;
            }
        }
    }

    Solution::from((part1, part2))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Comparison {
    GreaterThan,
    LessThan,
}

impl std::fmt::Display for Comparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GreaterThan => write!(f, ">"),
            Self::LessThan => write!(f, "<"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    start: u64,
    end: u64,
}

impl Range {
    pub fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    pub fn is_empty(&self) -> bool {
        self.end.saturating_sub(self.start) == 0
    }

    pub fn len(&self) -> u64 {
        self.end.saturating_sub(self.start) + 1
    }

    pub fn contains(&self, other: Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn intersect(&self, other: Self) -> Self {
        Self {
            start: u64::max(self.start, other.start),
            end: u64::min(self.end, other.end),
        }
    }

    pub fn difference(&self, other: Range) -> (Range, Range) {
        let intersection = self.intersect(other);

        let lower = Self {
            start: u64::min(self.start, other.start),
            end: intersection.start,
        };
        let upper = Self {
            start: intersection.end,
            end: u64::max(self.end, other.end),
        };
        (lower, upper)
    }
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.start, self.end)
    }
}
