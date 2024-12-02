use std::collections::{HashMap, HashSet, VecDeque};

use fxhash::FxHashMap;
use util::Solution;

pub fn solve(input: &str) -> Solution {
    let grid = Grid::from_str(input);

    let start_y = 0;
    let start_x = (0..grid.width as i32)
        .find(|&i| !matches!(grid.get_tile(i, start_y), Some(Tile::Forest)))
        .unwrap();

    let end_y = grid.height as i32 - 1;
    let end_x = (0..grid.width as i32)
        .find(|&i| !matches!(grid.get_tile(i, end_y), Some(Tile::Forest)))
        .unwrap();

    let mut to_visit = VecDeque::new();
    to_visit.push_front((start_x, start_y, Dir::Down, 0, HashSet::new()));

    let mut part1 = u64::MIN;
    while let Some((x, y, dir, count, mut visited)) = to_visit.pop_front() {
        if x == end_x && y == end_y {
            part1 = u64::max(count, part1);
            continue;
        }

        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        // Must follow tile dir
        if let Some(Tile::Slope(dir)) = grid.get_tile(x, y) {
            let (new_x, new_y) = dir.offset(x, y);
            to_visit.push_back((new_x, new_y, dir, count + 1, visited));
        } else if let Some(Tile::Path) = grid.get_tile(x, y) {
            let (new_x, new_y) = dir.offset(x, y);
            to_visit.push_back((new_x, new_y, dir, count + 1, visited.clone()));

            let new_dir = dir.left();
            let (new_x, new_y) = new_dir.offset(x, y);
            to_visit.push_back((new_x, new_y, new_dir, count + 1, visited.clone()));

            let new_dir = dir.right();
            let (new_x, new_y) = new_dir.offset(x, y);
            to_visit.push_back((new_x, new_y, new_dir, count + 1, visited));
        }
    }

    let mut visited = HashSet::new();
    let mut todo = vec![(start_x, start_y, Dir::Down)];
    let mut paths: FxHashMap<(i32, i32), HashMap<(i32, i32), u32>> = HashMap::default();
    while let Some((mut x, mut y, mut dir)) = todo.pop() {
        if visited.contains(&(x, y, dir)) {
            continue;
        }
        visited.insert((x, y, dir));

        let s_x = x;
        let s_y = y;

        let (new_x, new_y) = dir.offset(x, y);
        x = new_x;
        y = new_y;

        let mut path_len = 0;
        loop {
            let mut straight = false;
            let (new_x, new_y) = dir.offset(x, y);
            if let Some(Tile::Path | Tile::Slope(_)) = grid.get_tile(new_x, new_y) {
                straight = true;
            }

            let mut left = false;
            let (new_x, new_y) = dir.left().offset(x, y);
            if let Some(Tile::Path | Tile::Slope(_)) = grid.get_tile(new_x, new_y) {
                left = true;
            }

            let mut right = false;
            let (new_x, new_y) = dir.right().offset(x, y);
            if let Some(Tile::Path | Tile::Slope(_)) = grid.get_tile(new_x, new_y) {
                right = true;
            }

            let count = straight as u8 + left as u8 + right as u8;
            if count == 1 {
                visited.insert((x, y, dir));
                path_len += 1;
                if straight {
                    let (new_x, new_y) = dir.offset(x, y);
                    x = new_x;
                    y = new_y;
                } else if left {
                    let new_dir = dir.left();
                    let (new_x, new_y) = new_dir.offset(x, y);
                    dir = new_dir;
                    x = new_x;
                    y = new_y;
                } else if right {
                    let new_dir = dir.right();
                    let (new_x, new_y) = new_dir.offset(x, y);
                    dir = new_dir;
                    x = new_x;
                    y = new_y;
                }
            } else if count > 1 {
                if straight {
                    todo.push((x, y, dir));
                }
                if left {
                    let new_dir = dir.left();
                    todo.push((x, y, new_dir));
                }
                if right {
                    let new_dir = dir.right();
                    todo.push((x, y, new_dir));
                }
                break;
            } else {
                break;
            }
        }

        if path_len != 0 {
            let first = (s_x, s_y);
            let last = (x, y);

            paths.entry(first).or_default().insert(last, path_len + 1);
            paths.entry(last).or_default().insert(first, path_len + 1);
        }
    }

    let (&(real_end_x, real_end_y), &cost_to_exit) = match paths.get(&(end_x, end_y)) {
        Some(edges) => {
            assert!(edges.len() == 1, "maze exit must only be reachable via one path");
            edges.iter().next().unwrap()
        },
        None => panic!("maze exit must be reachable"),
    };

    let mut part2 = 0;
    let mut to_visit = vec![(start_x, start_y, Dir::Down, 0, HashSet::new())];
    while let Some((x, y, _dir, count, mut visited)) = to_visit.pop() {
        if x == real_end_x && y == real_end_y {
            part2 = u64::max(count + cost_to_exit as u64, part2);
            continue;
        }
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        for ((end_x, end_y), path_len) in paths.get(&(x, y)).unwrap() {
            to_visit.push((
                *end_x,
                *end_y,
                _dir,
                count + *path_len as u64,
                visited.clone(),
            ));
        }
    }

    Solution::from((part1, part2))
}

#[derive(Debug, Clone)]
struct Grid {
    tiles: Box<[Tile]>,
    height: usize,
    width: usize,
}

impl Grid {
    pub fn from_str(input: &str) -> Self {
        let grid = input
            .lines()
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '#' => Tile::Forest,
                    '.' => Tile::Path,
                    '<' => Tile::Slope(Dir::Left),
                    '>' => Tile::Slope(Dir::Right),
                    '^' => Tile::Slope(Dir::Up),
                    'v' => Tile::Slope(Dir::Down),
                    _ => panic!(),
                })
            })
            .collect::<Box<_>>();
        Self {
            tiles: grid,
            height: input.lines().count(),
            width: input.lines().next().unwrap().len(),
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<Tile> {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            Some(self.tiles[y as usize * self.width + x as usize])
        } else {
            None
        }
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Slope(Dir),
    Path,
    Forest,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
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

    pub fn left(&self) -> Dir {
        match self {
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up,
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
