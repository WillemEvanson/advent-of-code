use std::collections::HashSet;

use util::grid::Grid;
use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut bytes = Vec::new();
    for line in input.lines() {
        let (x, y) = line.split_once(',').unwrap();
        let x = x.parse::<u32>().unwrap();
        let y = y.parse::<u32>().unwrap();
        bytes.push((x, y));
    }

    let mut grid = Grid::new(71, 71);
    for &(x, y) in bytes.iter().take(1024) {
        grid.set(x, y, true);
    }

    let start_x = 0;
    let start_y = 0;
    let end_x = 70;
    let end_y = 70;

    let mut steps = 0;
    let mut visited = HashSet::new();
    let mut next = Vec::new();
    let mut current = vec![(start_x, start_y)];
    'steps: loop {
        while let Some((x, y)) = current.pop() {
            if x == end_x && y == end_y {
                break 'steps;
            }

            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));

            if let Some((new_x, new_y)) = grid
                .get_offset(x, y, -1, 0)
                .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
            {
                next.push((new_x, new_y));
            }

            if let Some((new_x, new_y)) = grid
                .get_offset(x, y, 1, 0)
                .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
            {
                next.push((new_x, new_y));
            }

            if let Some((new_x, new_y)) = grid
                .get_offset(x, y, 0, -1)
                .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
            {
                next.push((new_x, new_y));
            }

            if let Some((new_x, new_y)) = grid
                .get_offset(x, y, 0, 1)
                .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
            {
                next.push((new_x, new_y));
            }
        }

        std::mem::swap(&mut current, &mut next);
        steps += 1;
    }
    let part1 = steps;

    let mut part2 = String::new();
    'steps: for &(byte_x, byte_y) in bytes.iter().skip(1024) {
        grid.set(byte_x, byte_y, true);

        let mut visited = HashSet::new();
        let mut next = Vec::new();
        let mut current = vec![(start_x, start_y)];
        while !current.is_empty() {
            while let Some((x, y)) = current.pop() {
                if x == end_x && y == end_y {
                    continue 'steps;
                }

                if visited.contains(&(x, y)) {
                    continue;
                }
                visited.insert((x, y));

                if let Some((new_x, new_y)) = grid
                    .get_offset(x, y, -1, 0)
                    .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
                {
                    next.push((new_x, new_y));
                }

                if let Some((new_x, new_y)) = grid
                    .get_offset(x, y, 1, 0)
                    .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
                {
                    next.push((new_x, new_y));
                }

                if let Some((new_x, new_y)) = grid
                    .get_offset(x, y, 0, -1)
                    .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
                {
                    next.push((new_x, new_y));
                }

                if let Some((new_x, new_y)) = grid
                    .get_offset(x, y, 0, 1)
                    .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
                {
                    next.push((new_x, new_y));
                }
            }

            std::mem::swap(&mut current, &mut next);
        }

        part2 = format!("{byte_x},{byte_y}");
        break;
    }

    Solution::from((part1, part2))
}
