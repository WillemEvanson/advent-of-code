use std::cmp::Reverse;
use std::collections::BinaryHeap;

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

    let start_direction = Direction::Right;
    let mut distances = vec![u64::MAX; 4 * (grid.width() * grid.height()) as usize];
    distances[4 * grid.get_index_unchecked(start_x, start_y) + start_direction as usize] = 0;

    let mut part1 = u64::MAX;
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), start_x, start_y, start_direction));
    while let Some((Reverse(cost), x, y, direction)) = queue.pop() {
        if x == end_x && y == end_y {
            part1 = cost;
        } else if part1 != u64::MAX {
            break;
        }

        let new_cost = cost + 1000;
        let new_direction = direction.left();
        let distance_idx = 4 * grid.get_index_unchecked(x, y) + new_direction as usize;
        if distances[distance_idx] > new_cost {
            queue.push((Reverse(new_cost), x, y, new_direction));
            distances[distance_idx] = new_cost;
        }

        let new_cost = cost + 1000;
        let new_direction = direction.right();
        let distance_idx = 4 * grid.get_index_unchecked(x, y) + new_direction as usize;
        if distances[distance_idx] > new_cost {
            queue.push((Reverse(new_cost), x, y, new_direction));
            distances[distance_idx] = new_cost;
        }

        let new_cost = cost + 1;
        let (offset_x, offset_y) = direction.offset();
        if let Some((new_x, new_y)) = grid
            .get_offset(x, y, offset_x, offset_y)
            .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
        {
            let distance_idx = 4 * grid.get_index_unchecked(new_x, new_y) + direction as usize;
            if distances[distance_idx] > new_cost {
                queue.push((Reverse(new_cost), new_x, new_y, direction));
                distances[distance_idx] = new_cost;
            }
        }
    }

    let mut todo = Vec::new();
    for direction in [
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ] {
        let end_idx = 4 * grid.get_index_unchecked(end_x, end_y) + direction as usize;
        if distances[end_idx] == part1 {
            todo.push((part1, end_x, end_y, direction));
        }
    }

    let mut paths = Vec::new();
    while let Some((cost, x, y, direction)) = todo.pop() {
        paths.push((x, y));
        if x == start_x && y == start_y {
            continue;
        }

        let new_cost = cost.wrapping_sub(1000);
        let new_direction = direction.left();
        let distance_idx = 4 * grid.get_index_unchecked(x, y) + new_direction as usize;
        if distances[distance_idx] == new_cost {
            todo.push((new_cost, x, y, new_direction));
            distances[distance_idx] = u64::MAX;
        }

        let new_cost = cost.wrapping_sub(1000);
        let new_direction = direction.right();
        let distance_idx = 4 * grid.get_index_unchecked(x, y) + new_direction as usize;
        if distances[distance_idx] == new_cost {
            todo.push((new_cost, x, y, new_direction));
            distances[distance_idx] = u64::MAX;
        }

        let new_cost = cost.wrapping_sub(1);
        let (offset_x, offset_y) = direction.back().offset();
        if let Some((new_x, new_y)) = grid
            .get_offset(x, y, offset_x, offset_y)
            .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
        {
            let distance_idx = 4 * grid.get_index_unchecked(new_x, new_y) + direction as usize;
            if distances[distance_idx] == new_cost {
                todo.push((new_cost, new_x, new_y, direction));
                distances[distance_idx] = u64::MAX;
            }
        }
    }
    paths.sort_unstable();
    paths.dedup();
    let part2 = paths.len() as u64;

    Solution::from((part1, part2))
}
