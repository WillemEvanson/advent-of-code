use std::collections::{HashMap, HashSet};

use util::direction::Direction;
use util::grid::Grid;
use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut start_x = 0;
    let mut start_y = 0;
    let mut end_x = 0;
    let mut end_y = 0;
    let mut grid = Grid::new(
        input.lines().next().unwrap().len() as u32,
        input.lines().count() as u32,
    );
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '#' => true,
                'S' => {
                    start_x = x as u32;
                    start_y = y as u32;
                    false
                }
                'E' => {
                    end_x = x as u32;
                    end_y = y as u32;
                    false
                }
                '.' => false,
                _ => panic!(),
            };
            grid.set(x as u32, y as u32, tile);
        }
    }

    let mut part1 = u64::MAX;
    let mut visited = HashMap::new();
    let mut stack = vec![(start_x, start_y, Direction::Right, 0)];
    while let Some((x, y, direction, cost)) = stack.pop() {
        if cost >= part1 {
            continue;
        }

        if x == end_x && y == end_y {
            part1 = cost;
            continue;
        }

        if let Some(&old_cost) = visited.get(&(x, y, direction)) {
            if old_cost <= cost {
                continue;
            }
        }
        visited.insert((x, y, direction), cost);

        let new_direction = direction.left();
        let (offset_x, offset_y) = new_direction.offset();
        if let Some((new_x, new_y)) = grid
            .get_offset(x, y, offset_x, offset_y)
            .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
        {
            stack.push((new_x, new_y, new_direction, cost + 1001));
        }

        let new_direction = direction.right();
        let (offset_x, offset_y) = new_direction.offset();
        if let Some((new_x, new_y)) = grid
            .get_offset(x, y, offset_x, offset_y)
            .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
        {
            stack.push((new_x, new_y, new_direction, cost + 1001));
        }

        let (offset_x, offset_y) = direction.offset();
        if let Some((new_x, new_y)) = grid
            .get_offset(x, y, offset_x, offset_y)
            .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
        {
            stack.push((new_x, new_y, direction, cost + 1));
        }
    }

    let mut best_paths = Vec::new();
    let mut visited = HashMap::new();
    let mut stack = vec![(
        start_x,
        start_y,
        Direction::Right,
        0,
        vec![(start_x, start_y)],
    )];
    while let Some((x, y, direction, cost, path)) = stack.pop() {
        if cost > part1 {
            continue;
        }

        if x == end_x && y == end_y {
            best_paths.push(path);
            continue;
        }

        if let Some(&old_cost) = visited.get(&(x, y, direction)) {
            if old_cost < cost {
                continue;
            }
        }
        visited.insert((x, y, direction), cost);

        let new_direction = direction.left();
        let (offset_x, offset_y) = new_direction.offset();
        if let Some((new_x, new_y)) = grid
            .get_offset(x, y, offset_x, offset_y)
            .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
        {
            let mut new_path = path.clone();
            new_path.push((new_x, new_y));
            stack.push((new_x, new_y, new_direction, cost + 1001, new_path));
        }

        let new_direction = direction.right();
        let (offset_x, offset_y) = new_direction.offset();
        if let Some((new_x, new_y)) = grid
            .get_offset(x, y, offset_x, offset_y)
            .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
        {
            let mut new_path = path.clone();
            new_path.push((new_x, new_y));
            stack.push((new_x, new_y, new_direction, cost + 1001, new_path));
        }

        let new_direction = direction;
        let (offset_x, offset_y) = new_direction.offset();
        if let Some((new_x, new_y)) = grid
            .get_offset(x, y, offset_x, offset_y)
            .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
        {
            let mut new_path = path.clone();
            new_path.push((new_x, new_y));
            stack.push((new_x, new_y, new_direction, cost + 1, new_path));
        }
    }

    let mut visited = HashSet::new();
    for &(x, y) in best_paths.iter().flatten() {
        visited.insert((x, y));
    }
    let part2 = visited.len() as u64;

    Solution::from((part1, part2))
}
