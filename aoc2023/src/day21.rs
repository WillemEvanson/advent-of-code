use std::collections::HashSet;

use util::grid::Grid;
use util::math::lcm;
use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut grid = Grid::new(
        input.lines().next().unwrap().len() as u32,
        input.lines().count() as u32,
    );

    let mut start_x = 0;
    let mut start_y = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '#' => Tile::Rocks,
                '.' => Tile::Garden,
                'S' => {
                    start_x = x as u32;
                    start_y = y as u32;
                    Tile::Garden
                }
                _ => panic!(),
            };
            grid.set(x as u32, y as u32, tile);
        }
    }

    let mut next = Vec::new();
    let mut current = vec![(start_x, start_y)];
    let mut visited = HashSet::new();
    for _ in 0..64 {
        visited.clear();
        while let Some((x, y)) = current.pop() {
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));

            if let Some(Tile::Garden) = grid.get_at_offset(x, y, -1, 0) {
                next.push((x - 1, y));
            }
            if let Some(Tile::Garden) = grid.get_at_offset(x, y, 1, 0) {
                next.push((x + 1, y));
            }
            if let Some(Tile::Garden) = grid.get_at_offset(x, y, 0, -1) {
                next.push((x, y - 1));
            }
            if let Some(Tile::Garden) = grid.get_at_offset(x, y, 0, 1) {
                next.push((x, y + 1));
            }
        }

        std::mem::swap(&mut current, &mut next);
    }
    current.sort();
    current.dedup();
    let part1 = current.len() as u64;

    let mut priors = Vec::new();
    let mut next = Vec::new();
    let mut current = vec![(start_x as i64, start_y as i64)];
    let mut visited = HashSet::new();
    let dim_lcm = lcm(grid.width() as u64, grid.height() as u64);
    let iter_until = 2 * dim_lcm + 1;
    for _ in 0..=iter_until {
        visited.clear();
        while let Some((x, y)) = current.pop() {
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));

            let new_y = y.rem_euclid(grid.height() as i64) as u32;
            let new_x = (x - 1).rem_euclid(grid.width() as i64) as u32;
            if let Some(Tile::Garden) = grid.get_at(new_x, new_y) {
                next.push((x - 1, y));
            }

            let new_x = (x + 1).rem_euclid(grid.width() as i64) as u32;
            if let Some(Tile::Garden) = grid.get_at(new_x, new_y) {
                next.push((x + 1, y));
            }

            let new_x = x.rem_euclid(grid.width() as i64) as u32;
            let new_y = (y - 1).rem_euclid(grid.height() as i64) as u32;
            if let Some(Tile::Garden) = grid.get_at(new_x, new_y) {
                next.push((x, y - 1));
            }

            let new_y = (y + 1).rem_euclid(grid.height() as i64) as u32;
            if let Some(Tile::Garden) = grid.get_at(new_x, new_y) {
                next.push((x, y + 1));
            }
        }
        priors.push(visited.len() as u64);

        std::mem::swap(&mut current, &mut next);
    }

    let mut i = iter_until;
    #[allow(clippy::identity_op)]
    while i < 26501365 {
        let x3 = priors[priors.len() - 1 * dim_lcm as usize - 0];
        let x2 = priors[priors.len() - 1 * dim_lcm as usize - 1];

        let y1 = x3 - x2;

        let x1 = priors[priors.len() - 2 * dim_lcm as usize - 0];
        let x0 = priors[priors.len() - 2 * dim_lcm as usize - 1];

        let y0 = x1 - x0;

        let v0 = priors.last().unwrap() + 2 * y1 - y0;
        priors.push(v0);

        i += 1;
    }
    let part2 = *priors.last().unwrap();

    Solution::from((part1, part2))
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Tile {
    #[default]
    Garden,
    Rocks,
}
