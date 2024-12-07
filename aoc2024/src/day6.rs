use std::collections::HashMap;
use std::collections::HashSet;

use util::{BitSet, Solution};

pub fn solve(input: &str) -> Solution {
    let mut grid = Vec::new();
    let mut initial_x = 0;
    let mut initial_y = 0;
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let empty = match c {
                '.' => true,
                '^' => {
                    initial_x = x as i32;
                    initial_y = y as i32;
                    true
                }
                '#' => false,
                _ => panic!(),
            };
            row.push(empty);
        }
        grid.push(row);
    }
    let max_x = grid[0].len() as i32;
    let max_y = grid.len() as i32;

    // Part 1
    let mut guard_x = initial_x;
    let mut guard_y = initial_y;
    let mut guard_dir = Dir::Up;

    let mut jump_set = HashMap::new();
    let mut visited = HashSet::new();
    'outer: loop {
        // Compute how much to jump forward
        let old_x = guard_x;
        let old_y = guard_y;
        let old_dir = guard_dir;
        loop {
            visited.insert((guard_x, guard_y));
            let (new_x, new_y) = guard_dir.offset(guard_x, guard_y);
            if (0..max_x).contains(&new_x) && (0..max_y).contains(&new_y) {
                if grid[new_y as usize][new_x as usize] {
                    guard_x = new_x;
                    guard_y = new_y;
                } else {
                    break;
                }
            } else {
                jump_set.insert((old_x, old_y, old_dir), (guard_x, guard_y, guard_dir));
                break 'outer;
            }
        }
        // Turn to new direction
        loop {
            guard_dir = guard_dir.right();
            let (new_x, new_y) = guard_dir.offset(guard_x, guard_y);
            if (0..max_x).contains(&new_x) && (0..max_y).contains(&new_y) {
                if grid[new_y as usize][new_x as usize] {
                    break;
                }
            } else {
                break;
            }
        }
        jump_set.insert((old_x, old_y, old_dir), (guard_x, guard_y, guard_dir));
    }
    let part1 = visited.len() as u64;

    // Part 2
    let mut count = 0;
    for &(x, y) in visited.iter() {
        if x == initial_x && y == initial_y {
            continue;
        }

        grid[y as usize][x as usize] = false;
        let mut guard_x = initial_x;
        let mut guard_y = initial_y;
        let mut guard_dir = Dir::Up;

        let mut visited = BitSet::new(4 * max_x as usize * max_y as usize);
        let found_loop = loop {
            let visited_idx =
                4 * (guard_y as usize * max_x as usize + guard_x as usize) + guard_dir as usize;
            if visited.get(visited_idx) {
                break true;
            }
            visited.set(visited_idx);

            if guard_x != x && guard_y != y {
                if let Some(&(new_x, new_y, new_dir)) = jump_set.get(&(guard_x, guard_y, guard_dir))
                {
                    if (0..max_x).contains(&new_x) && (0..max_y).contains(&new_y) {
                        guard_x = new_x;
                        guard_y = new_y;
                        guard_dir = new_dir;
                        continue;
                    } else {
                        break false;
                    }
                }
            }

            let (new_x, new_y) = guard_dir.offset(guard_x, guard_y);
            if (0..max_x).contains(&new_x) && (0..max_y).contains(&new_y) {
                if grid[new_y as usize][new_x as usize] {
                    guard_x = new_x;
                    guard_y = new_y;
                } else {
                    guard_dir = guard_dir.right();
                }
            } else {
                break false;
            }
        };

        grid[y as usize][x as usize] = true;
        if found_loop {
            count += 1;
        }
    }
    let part2 = count;

    Solution::from((part1, part2))
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Right = 0,
    Left = 1,
    Up = 2,
    Down = 3,
}

impl Dir {
    pub fn offset(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Self::Up => (x, y - 1),
            Self::Down => (x, y + 1),
            Self::Right => (x + 1, y),
            Self::Left => (x - 1, y),
        }
    }

    pub fn right(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}
