use std::collections::HashSet;

use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut trailheads = Vec::new();
    let mut grid = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                let digit = c.to_digit(10).unwrap();
                row.push(Some(c.to_digit(10).unwrap()));
                if digit == 0 {
                    trailheads.push((x as i32, y as i32));
                }
            } else {
                row.push(None);
            }
        }
        grid.push(row);
    }
    let grid_width = grid[0].len() as i32;
    let grid_height = grid.len() as i32;

    let mut part1 = 0;
    for &(x, y) in trailheads.iter() {
        let mut stack = vec![(x, y)];
        let mut visited = HashSet::new();
        while let Some((x, y)) = stack.pop() {
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));

            let current_height = grid[y as usize][x as usize].unwrap();
            if current_height == 9 {
                part1 += 1;
                continue;
            }

            let new_x = x - 1;
            let new_y = y;
            if (0..grid_width).contains(&new_x) && (0..grid_height).contains(&new_y) {
                if let Some(height) = grid[new_y as usize][new_x as usize] {
                    if current_height + 1 == height {
                        stack.push((new_x, new_y));
                    }
                }
            }

            let new_x = x + 1;
            let new_y = y;
            if (0..grid_width).contains(&new_x) && (0..grid_height).contains(&new_y) {
                if let Some(height) = grid[new_y as usize][new_x as usize] {
                    if current_height + 1 == height {
                        stack.push((new_x, new_y));
                    }
                }
            }

            let new_x = x;
            let new_y = y - 1;
            if (0..grid_width).contains(&new_x) && (0..grid_height).contains(&new_y) {
                if let Some(height) = grid[new_y as usize][new_x as usize] {
                    if current_height + 1 == height {
                        stack.push((new_x, new_y));
                    }
                }
            }

            let new_x = x;
            let new_y = y + 1;
            if (0..grid_width).contains(&new_x) && (0..grid_height).contains(&new_y) {
                if let Some(height) = grid[new_y as usize][new_x as usize] {
                    if current_height + 1 == height {
                        stack.push((new_x, new_y));
                    }
                }
            }
        }
    }

    let mut part2 = 0;
    let mut stack = trailheads.clone();
    while let Some((x, y)) = stack.pop() {
        let current_height = grid[y as usize][x as usize].unwrap();
        if current_height == 9 {
            part2 += 1;
            continue;
        }

        let new_x = x - 1;
        let new_y = y;
        if (0..grid_width).contains(&new_x) && (0..grid_height).contains(&new_y) {
            if let Some(height) = grid[new_y as usize][new_x as usize] {
                if current_height + 1 == height {
                    stack.push((new_x, new_y));
                }
            }
        }

        let new_x = x + 1;
        let new_y = y;
        if (0..grid_width).contains(&new_x) && (0..grid_height).contains(&new_y) {
            if let Some(height) = grid[new_y as usize][new_x as usize] {
                if current_height + 1 == height {
                    stack.push((new_x, new_y));
                }
            }
        }

        let new_x = x;
        let new_y = y - 1;
        if (0..grid_width).contains(&new_x) && (0..grid_height).contains(&new_y) {
            if let Some(height) = grid[new_y as usize][new_x as usize] {
                if current_height + 1 == height {
                    stack.push((new_x, new_y));
                }
            }
        }

        let new_x = x;
        let new_y = y + 1;
        if (0..grid_width).contains(&new_x) && (0..grid_height).contains(&new_y) {
            if let Some(height) = grid[new_y as usize][new_x as usize] {
                if current_height + 1 == height {
                    stack.push((new_x, new_y));
                }
            }
        }
    }

    Solution::from((part1, part2))
}
