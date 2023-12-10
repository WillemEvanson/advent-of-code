use std::collections::HashSet;

use util::Solution;

pub fn solve(input: &str) -> Solution {
    let (mut grid, (start_x, start_y)) = Grid::parse(input);
    let mut lgrid = LGrid::new(grid.width(), grid.height());

    let mut depth = 0;
    let mut visited = HashSet::new();
    let mut next = Vec::new();
    let mut current = [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(|(diff_x, diff_y)| {
            start_x
                .checked_add_signed(diff_x)
                .zip(start_y.checked_add_signed(diff_y))
        })
        .filter(|&(x, y)| {
            let tile = grid.get_tile(x, y);
            let first = if let Some((x, y)) = tile.first(x, y) {
                x == start_x && y == start_y
            } else {
                false
            };
            let second = if let Some((x, y)) = tile.second(x, y) {
                x == start_x && y == start_y
            } else {
                false
            };
            first || second
        })
        .collect::<Vec<_>>();

    for kind in [
        Tile::Vertical,
        Tile::Horizontal,
        Tile::NE,
        Tile::NW,
        Tile::SW,
        Tile::SE,
    ] {
        if let Some((first, second)) = kind
            .first(start_x, start_y)
            .zip(kind.second(start_x, start_y))
        {
            if current.contains(&first) && current.contains(&second) {
                grid.set_tile(start_x, start_y, kind);
                break;
            }
        }
    }

    while !current.is_empty() {
        while let Some((x, y)) = current.pop() {
            let tile = grid.get_tile(x, y);
            lgrid.set_tile(x, y, tile);
            grid.set_loop(x, y);

            if let Some(first) = tile.first(x, y) {
                if !visited.contains(&first) {
                    visited.insert(first);
                    next.push(first);
                }
            }
            if let Some(second) = tile.second(x, y) {
                if !visited.contains(&second) {
                    visited.insert(second);
                    next.push(second);
                }
            }
        }

        std::mem::swap(&mut current, &mut next);
        depth += 1;
    }
    let part1 = depth;

    let mut visited = HashSet::new();
    let mut enclosed = HashSet::new();
    loop {
        let mut start = None;
        'find: for y in 0..lgrid.height as u32 {
            for x in 0..lgrid.width as u32 {
                if let Some(false) = lgrid.get(x, y) {
                    if !visited.contains(&(x, y)) {
                        start = Some((x, y));
                        break 'find;
                    }
                }
            }
        }

        let mut inside = true;
        let mut visited_internal = HashSet::new();
        if let Some((x, y)) = start {
            let mut next = Vec::new();
            let mut current = vec![(x as i32, y as i32)];
            while !current.is_empty() {
                while let Some((x, y)) = current.pop() {
                    if let Some(true) = lgrid.get(x as u32, y as u32) {
                        continue;
                    } else if lgrid.get(x as u32, y as u32).is_none() {
                        inside = false;
                        continue;
                    }

                    if !visited_internal.contains(&(x as u32, y as u32)) {
                        visited_internal.insert((x as u32, y as u32));
                    } else {
                        continue;
                    }

                    current.push((x - 1, y));
                    current.push((x + 1, y));
                    current.push((x, y - 1));
                    current.push((x, y + 1));
                }
                std::mem::swap(&mut current, &mut next);
            }
        } else {
            break;
        }

        if inside {
            enclosed.extend(visited_internal.iter().copied());
        }
        visited.extend(visited_internal);
    }
    let part2 = enclosed
        .into_iter()
        .filter(|(x, y)| x % 3 == 0 && y % 3 == 0)
        .count() as u64;

    Solution::from((part1, part2))
}

pub struct LGrid {
    is_loop: Box<[bool]>,
    height: usize,
    width: usize,
}

impl LGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            is_loop: vec![false; (width * 3) * (height * 3)].into_boxed_slice(),
            height: height * 3,
            width: width * 3,
        }
    }

    pub fn get(&self, x: u32, y: u32) -> Option<bool> {
        if x < self.width as u32 && y < self.height as u32 {
            Some(self.is_loop[y as usize * self.width + x as usize])
        } else {
            None
        }
    }

    pub fn set_tile(&mut self, x: u32, y: u32, tile: Tile) {
        let x = x * 3;
        let y = y * 3;

        let (x, y, z) = match tile {
            Tile::Vertical => (
                *tile.first(x, y).get_or_insert((x, y)),
                (x, y),
                *tile.second(x, y).get_or_insert((x, y)),
            ),
            Tile::Horizontal => (
                *tile.first(x, y).get_or_insert((x, y)),
                (x, y),
                *tile.second(x, y).get_or_insert((x, y)),
            ),
            Tile::NE => (
                *tile.first(x, y).get_or_insert((x, y)),
                (x, y),
                *tile.second(x, y).get_or_insert((x, y)),
            ),
            Tile::NW => (
                *tile.first(x, y).get_or_insert((x, y)),
                (x, y),
                *tile.second(x, y).get_or_insert((x, y)),
            ),
            Tile::SE => (
                *tile.first(x, y).get_or_insert((x, y)),
                (x, y),
                *tile.second(x, y).get_or_insert((x, y)),
            ),
            Tile::SW => (
                *tile.first(x, y).get_or_insert((x, y)),
                (x, y),
                *tile.second(x, y).get_or_insert((x, y)),
            ),
            Tile::Ground => (
                *tile.first(x, y).get_or_insert((x, y)),
                (x, y),
                *tile.second(x, y).get_or_insert((x, y)),
            ),
            Tile::Start => (
                *tile.first(x, y).get_or_insert((x, y)),
                (x, y),
                *tile.second(x, y).get_or_insert((x, y)),
            ),
        };

        self.set(x.0, x.1);
        self.set(y.0, y.1);
        self.set(z.0, z.1);
    }

    fn set(&mut self, x: u32, y: u32) {
        self.is_loop[y as usize * self.width + x as usize] = true;
    }
}

pub struct Grid {
    is_loop: Box<[bool]>,
    tiles: Box<[Tile]>,
    height: usize,
    width: usize,
}

impl Grid {
    pub fn parse(input: &str) -> (Self, (u32, u32)) {
        let mut width = 0;
        let mut height = 0;
        let mut tiles = Vec::new();
        let mut start = (u32::MAX, u32::MAX);
        for (y, line) in input.lines().enumerate() {
            for (x, tile) in line.chars().enumerate() {
                let tile = match tile {
                    '|' => Tile::Vertical,
                    '-' => Tile::Horizontal,
                    'L' => Tile::NE,
                    'J' => Tile::NW,
                    '7' => Tile::SW,
                    'F' => Tile::SE,
                    '.' => Tile::Ground,
                    'S' => {
                        start = (x as u32, y as u32);
                        Tile::Start
                    }
                    _ => panic!("Invalid char"),
                };
                tiles.push(tile);
                width = x + 1;
            }
            height = y + 1;
        }

        (
            Self {
                is_loop: vec![false; tiles.len()].into_boxed_slice(),
                tiles: tiles.into_boxed_slice(),
                width,
                height,
            },
            start,
        )
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Tile {
        assert!(x < self.width as u32 && y < self.height as u32);
        self.tiles[y as usize * self.width + x as usize]
    }

    pub fn get_loop(&self, x: u32, y: u32) -> bool {
        assert!(x < self.width as u32 && y < self.height as u32);
        self.is_loop[y as usize * self.width + x as usize]
    }

    pub fn set_tile(&mut self, x: u32, y: u32, tile: Tile) {
        assert!(x < self.width as u32 && y < self.height as u32);
        self.tiles[y as usize * self.width + x as usize] = tile;
    }

    pub fn set_loop(&mut self, x: u32, y: u32) {
        assert!(x < self.width as u32 && y < self.height as u32);
        self.is_loop[y as usize * self.width + x as usize] = true;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Vertical,
    Horizontal,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl Tile {
    pub fn first(&self, x: u32, y: u32) -> Option<(u32, u32)> {
        let (x_diff, y_diff) = self.first_diff()?;
        Self::map_diff(x, y, x_diff, y_diff)
    }

    pub fn second(&self, x: u32, y: u32) -> Option<(u32, u32)> {
        let (x_diff, y_diff) = self.second_diff()?;
        Self::map_diff(x, y, x_diff, y_diff)
    }

    pub fn first_diff(&self) -> Option<(i32, i32)> {
        match self {
            Self::Vertical => Some((0, -1)),
            Self::Horizontal => Some((-1, 0)),
            Self::NE => Some((0, -1)),
            Self::NW => Some((0, -1)),
            Self::SW => Some((0, 1)),
            Self::SE => Some((0, 1)),
            Self::Ground => None,
            Self::Start => None,
        }
    }

    pub fn second_diff(&self) -> Option<(i32, i32)> {
        match self {
            Self::Vertical => Some((0, 1)),
            Self::Horizontal => Some((1, 0)),
            Self::NE => Some((1, 0)),
            Self::NW => Some((-1, 0)),
            Self::SW => Some((-1, 0)),
            Self::SE => Some((1, 0)),
            Self::Ground => None,
            Self::Start => None,
        }
    }

    fn map_diff(x: u32, y: u32, x_diff: i32, y_diff: i32) -> Option<(u32, u32)> {
        let x = x.checked_add_signed(x_diff);
        let y = y.checked_add_signed(y_diff);
        x.zip(y)
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Vertical => '|',
                Tile::Horizontal => '-',
                Tile::NE => 'L',
                Tile::NW => 'J',
                Tile::SW => '7',
                Tile::SE => 'F',
                Tile::Ground => '.',
                Tile::Start => 'S',
            }
        )
    }
}
