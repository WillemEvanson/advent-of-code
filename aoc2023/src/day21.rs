use std::collections::HashSet;

use util::{lcm, Solution};

pub fn solve(input: &str) -> Solution {
    let grid = Grid::from_str(input);
    let (mut start_x, mut start_y) = (0, 0);
    'outer: for y in 0..grid.height as i64 {
        for x in 0..grid.width as i64 {
            if let Some(Tile::Start) = grid.get(x, y) {
                start_x = x;
                start_y = y;
                break 'outer;
            }
        }
    }

    let mut visited = HashSet::new();
    let mut next = Vec::new();
    let mut current = vec![(start_x, start_y)];
    for _ in 0..64 {
        visited.clear();
        while let Some((x, y)) = current.pop() {
            if visited.contains(&(x, y)) {
                continue;
            }

            visited.insert((x, y));
            if let Some(Tile::Garden | Tile::Start) = grid.get(x - 1, y) {
                next.push((x - 1, y));
            }
            if let Some(Tile::Garden | Tile::Start) = grid.get(x + 1, y) {
                next.push((x + 1, y));
            }
            if let Some(Tile::Garden | Tile::Start) = grid.get(x, y - 1) {
                next.push((x, y - 1));
            }
            if let Some(Tile::Garden | Tile::Start) = grid.get(x, y + 1) {
                next.push((x, y + 1));
            }
        }

        std::mem::swap(&mut current, &mut next);
    }
    current.sort();
    current.dedup();
    let part1 = current.len() as u64;

    visited.clear();
    next.clear();

    current.clear();
    current.push((start_x, start_y));
    let mut priors = Vec::new();

    let lcm = lcm(grid.width as u64, grid.height as u64);

    for i in 0..=lcm * 10 {
        if i >= 3 * lcm {
            let mut i = i;
            #[allow(clippy::identity_op)]
            while i <= 26501365 {
                let x3 = priors[priors.len() - 1 * lcm as usize - 0];
                let x2 = priors[priors.len() - 1 * lcm as usize - 1];

                let y1 = x3 - x2;

                let x1 = priors[priors.len() - 2 * lcm as usize - 0];
                let x0 = priors[priors.len() - 2 * lcm as usize - 1];

                let y0 = x1 - x0;

                let v0 = priors.last().unwrap() + 2 * y1 - y0;
                priors.push(v0);

                i += 1;
            }
            break;
        }

        while let Some((x, y)) = current.pop() {
            if visited.contains(&(x, y)) {
                continue;
            }

            visited.insert((x, y));
            if let Some(Tile::Garden | Tile::Start) = grid.get(
                (x - 1).rem_euclid(grid.width as i64),
                y.rem_euclid(grid.height as i64),
            ) {
                next.push((x - 1, y));
            }
            if let Some(Tile::Garden | Tile::Start) = grid.get(
                (x + 1).rem_euclid(grid.width as i64),
                y.rem_euclid(grid.height as i64),
            ) {
                next.push((x + 1, y));
            }
            if let Some(Tile::Garden | Tile::Start) = grid.get(
                x.rem_euclid(grid.width as i64),
                (y - 1).rem_euclid(grid.height as i64),
            ) {
                next.push((x, y - 1));
            }
            if let Some(Tile::Garden | Tile::Start) = grid.get(
                x.rem_euclid(grid.width as i64),
                (y + 1).rem_euclid(grid.height as i64),
            ) {
                next.push((x, y + 1));
            }
        }
        priors.push(visited.len() as u64);

        std::mem::swap(&mut current, &mut next);
        visited.clear();
    }
    let part2 = *priors.last().unwrap();

    Solution::from((part1, part2))
}

#[derive(Debug, Clone)]
struct Grid {
    tiles: Box<[Tile]>,
    height: usize,
    width: usize,
}

impl Grid {
    fn from_str(input: &str) -> Self {
        let tiles = input
            .lines()
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '.' => Tile::Garden,
                    '#' => Tile::Rocks,
                    'S' => Tile::Start,
                    _ => panic!(),
                })
            })
            .collect::<Box<_>>();
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();

        Self {
            tiles,
            height,
            width,
        }
    }

    pub fn get(&self, x: i64, y: i64) -> Option<&Tile> {
        if (0 <= x && x < self.width as i64) && (0 <= y && y < self.height as i64) {
            Some(&self.tiles[y as usize * self.width + x as usize])
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Garden,
    Rocks,
    Start,
}
