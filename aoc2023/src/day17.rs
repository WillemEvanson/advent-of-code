use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

use util::Solution;

pub fn solve(input: &str) -> Solution {
    let grid = Grid::from_str(input);

    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0, 0, Dir::North, 0));
    heap.push((Reverse(0), 0, 0, Dir::East, 0));
    heap.push((Reverse(0), 0, 0, Dir::South, 0));
    heap.push((Reverse(0), 0, 0, Dir::West, 0));

    let mut part1 = u64::MAX;
    while let Some((Reverse(lost), x, y, dir, line)) = heap.pop() {
        if x == (grid.width() as i32 - 1) && y == (grid.height() as i32 - 1) {
            part1 = u64::min(part1, lost as u64);
            break;
        }

        if visited.contains(&(x, y, dir, line)) {
            continue;
        } else {
            visited.insert((x, y, dir, line));
        }

        if line < 3 {
            let (next_x, next_y) = dir.offset(x, y);
            if let Some(loss) = grid.get_tile(next_x, next_y) {
                heap.push((Reverse(lost + loss), next_x, next_y, dir, line + 1));
            }
        }

        let next_dir = dir.left();
        let (next_x, next_y) = next_dir.offset(x, y);
        if let Some(loss) = grid.get_tile(next_x, next_y) {
            heap.push((Reverse(lost + loss), next_x, next_y, next_dir, 1));
        }

        let next_dir = dir.right();
        let (next_x, next_y) = next_dir.offset(x, y);
        if let Some(loss) = grid.get_tile(next_x, next_y) {
            heap.push((Reverse(lost + loss), next_x, next_y, next_dir, 1));
        }
    }

    visited.clear();
    heap.clear();

    heap.push((Reverse(0), 0, 0, Dir::North, 0));
    heap.push((Reverse(0), 0, 0, Dir::East, 0));
    heap.push((Reverse(0), 0, 0, Dir::South, 0));
    heap.push((Reverse(0), 0, 0, Dir::West, 0));

    let mut part2 = u64::MAX;
    while let Some((Reverse(lost), x, y, dir, line)) = heap.pop() {
        if x == (grid.width() as i32 - 1) && y == (grid.height() as i32 - 1) && line > 3 {
            part2 = lost as u64;
            break;
        }

        if visited.contains(&(x, y, dir, line)) {
            continue;
        } else {
            visited.insert((x, y, dir, line));
        }

        if line < 10 {
            let (next_x, next_y) = dir.offset(x, y);
            if let Some(loss) = grid.get_tile(next_x, next_y) {
                heap.push((Reverse(lost + loss), next_x, next_y, dir, line + 1));
            }
        }

        if line > 3 {
            let next_dir = dir.left();
            let (next_x, next_y) = next_dir.offset(x, y);
            if let Some(loss) = grid.get_tile(next_x, next_y) {
                heap.push((Reverse(lost + loss), next_x, next_y, next_dir, 1));
            }

            let next_dir = dir.right();
            let (next_x, next_y) = next_dir.offset(x, y);
            if let Some(loss) = grid.get_tile(next_x, next_y) {
                heap.push((Reverse(lost + loss), next_x, next_y, next_dir, 1));
            }
        }
    }

    Solution::from((part1, part2))
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    pub fn offset(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Self::North => (x, y - 1),
            Self::South => (x, y + 1),
            Self::East => (x + 1, y),
            Self::West => (x - 1, y),
        }
    }

    pub fn left(&self) -> Dir {
        match self {
            Dir::North => Dir::West,
            Dir::West => Dir::South,
            Dir::South => Dir::East,
            Dir::East => Dir::North,
        }
    }

    pub fn right(&self) -> Dir {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    tiles: Box<[u32]>,
    height: usize,
    width: usize,
}

impl Grid {
    pub fn from_str(input: &str) -> Self {
        let grid = input
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
            .collect::<Box<_>>();
        Self {
            tiles: grid,
            height: input.lines().count(),
            width: input.lines().next().unwrap().len(),
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<u32> {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            Some(self.tiles[y as usize * self.width + x as usize])
        } else {
            None
        }
    }
}
