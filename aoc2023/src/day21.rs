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

    let mut next_count = 0;
    let mut current_count = 0;
    let mut next = Vec::new();
    let mut current = vec![(start_x, start_y)];
    let mut visiteds: [HashSet<_>; 3] = core::array::from_fn(|_| HashSet::new());
    for _ in 0..=64 {
        visiteds[0].clear();
        while let Some((x, y)) = current.pop() {
            if visiteds[0].contains(&(x, y)) || visiteds[2].contains(&(x, y)) {
                continue;
            }
            visiteds[0].insert((x, y));

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
        current_count += visiteds[0].len() as u64;

        std::mem::swap(&mut current, &mut next);
        std::mem::swap(&mut current_count, &mut next_count);

        visiteds.swap(0, 2);
        visiteds.swap(1, 2);
    }
    let part1 = next_count;

    next_count = 0;
    current_count = 0;
    next.clear();
    current.clear();
    current.push((start_x, start_y));
    visiteds[0].clear();
    visiteds[1].clear();
    visiteds[2].clear();
    let mut priors = Vec::new();
    let dim_lcm = lcm(grid.width as u64, grid.height as u64);
    for _ in 0..=2 * dim_lcm + 1 {
        visiteds[0].clear();
        while let Some((x, y)) = current.pop() {
            if visiteds[0].contains(&(x, y)) || visiteds[2].contains(&(x, y)) {
                continue;
            }
            visiteds[0].insert((x, y));

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
        current_count += visiteds[0].len() as u64;
        priors.push(current_count);

        std::mem::swap(&mut current, &mut next);
        std::mem::swap(&mut current_count, &mut next_count);

        visiteds.swap(0, 2);
        visiteds.swap(1, 2);
    }

    // Compute the difference between the number of cells able to be visited. We can
    // then use this to compute the difference between the offsets for each step,
    // which will allow us to skip storing the entire array.
    let mut i = 2 * dim_lcm + 1;
    let mut offsets = Vec::new();
    #[allow(clippy::identity_op)]
    while i <= 4 * dim_lcm + 1 {
        let x3 = priors[priors.len() - 1 * dim_lcm as usize - 0];
        let x2 = priors[priors.len() - 1 * dim_lcm as usize - 1];

        let y1 = x3 - x2;

        let x1 = priors[priors.len() - 2 * dim_lcm as usize - 0];
        let x0 = priors[priors.len() - 2 * dim_lcm as usize - 1];

        let y0 = x1 - x0;

        let offset = 2 * y1 - y0;
        let v0 = priors.last().unwrap() + offset;

        offsets.push(offset);
        priors.push(v0);

        i += 1;
    }

    let mut offset_diffs = Vec::new();
    for i in 0..dim_lcm as usize {
        let y0 = offsets[i];
        let y1 = offsets[dim_lcm as usize + i];
        offset_diffs.push(y1 - y0);
    }

    let mut current_num = *priors.last().unwrap();
    while i <= 26501365 {
        let idx = ((i - 1) % dim_lcm) as usize;
        let o1 = offsets[idx] + ((i - 1) / dim_lcm - 2) * offset_diffs[idx];
        current_num += o1;
        i += 1;
    }
    let part2 = current_num;

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
