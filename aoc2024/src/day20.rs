use std::collections::HashMap;

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
                '.' => false,
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
                _ => panic!(),
            };
            grid.set(x as u32, y as u32, tile);
        }
    }

    let end_direction = if !*grid.get_at_offset(end_x, end_y, 0, -1).unwrap() {
        Direction::Up
    } else if !*grid.get_at_offset(end_x, end_y, 0, 1).unwrap() {
        Direction::Down
    } else if !*grid.get_at_offset(end_x, end_y, -1, 0).unwrap() {
        Direction::Left
    } else {
        Direction::Right
    };

    let mut part1 = 0;
    let mut part2 = 0;
    let mut x = end_x;
    let mut y = end_y;
    let mut direction = end_direction;
    let mut legal_time_to_end = 0;
    let mut cache = HashMap::new();
    loop {
        cache.insert((x, y), legal_time_to_end);
        legal_time_to_end += 1;

        // Compute all paths from here
        if legal_time_to_end >= 100 {
            let min_x = x.saturating_sub(21);
            let max_x = u32::min(x + 21, grid.width());

            let min_y = y.saturating_sub(21);
            let max_y = u32::min(y + 21, grid.height());

            for grid_y in min_y..max_y {
                for grid_x in min_x..max_x {
                    let manhattan_distance = grid_x.abs_diff(x) + grid_y.abs_diff(y);
                    if manhattan_distance > 20 {
                        continue;
                    }

                    if let Some(&remaining_to_end) = cache.get(&(grid_x, grid_y)) {
                        if legal_time_to_end - (remaining_to_end + manhattan_distance) < 100 {
                            continue;
                        }

                        if manhattan_distance == 2 {
                            part1 += 1;
                        }
                        part2 += 1;
                    }
                }
            }
        }

        if x == start_x && y == start_y {
            break;
        }

        let new_direction = direction;
        let (offset_x, offset_y) = new_direction.offset();
        if let Some((new_x, new_y)) = grid
            .get_offset(x, y, offset_x, offset_y)
            .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
        {
            direction = new_direction;
            x = new_x;
            y = new_y;
            continue;
        }

        let new_direction = direction.right();
        let (offset_x, offset_y) = new_direction.offset();
        if let Some((new_x, new_y)) = grid
            .get_offset(x, y, offset_x, offset_y)
            .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
        {
            direction = new_direction;
            x = new_x;
            y = new_y;
            continue;
        }

        let new_direction = direction.left();
        let (offset_x, offset_y) = new_direction.offset();
        if let Some((new_x, new_y)) = grid
            .get_offset(x, y, offset_x, offset_y)
            .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
        {
            direction = new_direction;
            x = new_x;
            y = new_y;
            continue;
        }
    }

    Solution::from((part1, part2))
}
