use std::collections::HashMap;

use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut part1 = 0;
    let mut part2 = 0;

    let mut cache = HashMap::new();
    for line in input.lines() {
        let mut total_cost_part1 = 0;
        let mut total_cost_part2 = 0;
        let mut current = NumericPad::Push;
        for c in line.chars() {
            let next = match c {
                '0' => NumericPad::Num0,
                '1' => NumericPad::Num1,
                '2' => NumericPad::Num2,
                '3' => NumericPad::Num3,
                '4' => NumericPad::Num4,
                '5' => NumericPad::Num5,
                '6' => NumericPad::Num6,
                '7' => NumericPad::Num7,
                '8' => NumericPad::Num8,
                '9' => NumericPad::Num9,
                'A' => NumericPad::Push,
                _ => panic!(),
            };
            let paths = numeric_paths(current, next);

            let mut min_cost = u64::MAX;
            for path in paths.iter() {
                let cost = find(&mut cache, path, 2);
                min_cost = u64::min(min_cost, cost);
            }
            total_cost_part1 += min_cost;

            let mut min_cost = u64::MAX;
            for path in paths.iter() {
                let cost = find(&mut cache, path, 25);
                min_cost = u64::min(min_cost, cost);
            }
            total_cost_part2 += min_cost;

            current = next;
        }

        let numeric_part = line
            .trim_matches(|c: char| !c.is_ascii_digit())
            .parse::<u64>()
            .unwrap();

        part1 += total_cost_part1 * numeric_part;
        part2 += total_cost_part2 * numeric_part;
    }

    Solution::from((part1, part2))
}

// This can be called on each segment (where a segment is a series of Actions
// with an A) as the sequence of moves to make the robot perform those actions
// must be terminated by an A as well.
fn find(
    cache: &mut HashMap<(Action, Action, u64), u64>,
    mut actions: &[Action],
    remaining: u64,
) -> u64 {
    if remaining == 0 {
        return actions.len() as u64;
    }

    let mut total_cost = 0;
    while !actions.is_empty() {
        // Find segment
        let mut i = 0;
        while i < actions.len() {
            if actions[i] == Action::Push {
                i += 1;
                break;
            }
            i += 1;
        }

        // Process segment
        let mut current = Action::Push;
        for &next in &actions[..i] {
            let cost = if let Some(&cost) = cache.get(&(current, next, remaining)) {
                cost
            } else {
                let paths = directional_paths(current, next);

                let mut min_cost = u64::MAX;
                for path in paths.iter() {
                    let cost = find(cache, path, remaining - 1);
                    min_cost = u64::min(min_cost, cost);
                }

                cache.insert((current, next, remaining), min_cost);

                min_cost
            };

            total_cost += cost;

            current = next;
        }

        // End processing

        actions = &actions[i..];
    }

    total_cost
}

fn numeric_paths(from: NumericPad, to: NumericPad) -> Vec<Vec<Action>> {
    let (from_x, from_y) = from.position();
    let (to_x, to_y) = to.position();
    find_sequences(from_x, from_y, to_x, to_y, 0, 3)
}

fn directional_paths(from: Action, to: Action) -> Vec<Vec<Action>> {
    let (from_x, from_y) = from.position();
    let (to_x, to_y) = to.position();
    find_sequences(from_x, from_y, to_x, to_y, 0, 0)
}

fn find_sequences(
    from_x: u32,
    from_y: u32,
    to_x: u32,
    to_y: u32,
    invalid_x: u32,
    invalid_y: u32,
) -> Vec<Vec<Action>> {
    if from_x == to_x && from_y == to_y {
        return vec![vec![Action::Push]];
    }

    let mut sequences = Vec::new();

    let horizontal = |actions: &mut Vec<Action>, from_x: u32, to_x: u32| {
        for _ in 0..to_x.abs_diff(from_x) {
            if to_x > from_x {
                actions.push(Action::Right);
            } else {
                actions.push(Action::Left);
            }
        }
    };

    let vertical = |actions: &mut Vec<Action>, from_y: u32, to_y: u32| {
        for _ in 0..to_y.abs_diff(from_y) {
            if to_y > from_y {
                actions.push(Action::Down);
            } else {
                actions.push(Action::Up);
            }
        }
    };

    // Horizontal -> Vertical
    if !(to_x == invalid_x && from_y == invalid_y) {
        let mut actions = Vec::new();

        horizontal(&mut actions, from_x, to_x);
        vertical(&mut actions, from_y, to_y);
        actions.push(Action::Push);

        sequences.push(actions);
    }

    if sequences.len() == 1 && (from_x == to_x || from_y == to_y) {
        return sequences;
    }

    // Vertical -> Horizontal
    if !(from_x == invalid_x && to_y == invalid_y) {
        let mut actions = Vec::new();

        vertical(&mut actions, from_y, to_y);
        horizontal(&mut actions, from_x, to_x);
        actions.push(Action::Push);

        sequences.push(actions);
    }

    sequences
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NumericPad {
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Push,
}

impl NumericPad {
    fn position(self) -> (u32, u32) {
        match self {
            Self::Push => (2, 3),
            Self::Num0 => (1, 3),
            Self::Num1 => (0, 2),
            Self::Num2 => (1, 2),
            Self::Num3 => (2, 2),
            Self::Num4 => (0, 1),
            Self::Num5 => (1, 1),
            Self::Num6 => (2, 1),
            Self::Num7 => (0, 0),
            Self::Num8 => (1, 0),
            Self::Num9 => (2, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Action {
    Left,
    Right,
    Up,
    Down,
    Push,
}

impl Action {
    pub fn position(self) -> (u32, u32) {
        match self {
            Self::Left => (0, 1),
            Self::Right => (2, 1),
            Self::Up => (1, 0),
            Self::Down => (1, 1),
            Self::Push => (2, 0),
        }
    }
}
