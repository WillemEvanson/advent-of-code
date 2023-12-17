use std::cmp::Reverse;
use std::collections::BinaryHeap;

use util::{BitSet, Solution};

pub fn solve(input: &str) -> Solution {
    let grid = Grid::from_str(input);
    let end_x = (grid.width() as i32) - 1;
    let end_y = (grid.height() as i32) - 1;

    let mut visited = BitSet::new(2 * grid.width() * grid.height());
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0, 0, false));
    heap.push((Reverse(0), 0, 0, true));

    let mut part1 = u64::MAX;
    while let Some((Reverse(lost), x, y, vertical)) = heap.pop() {
        if x == end_x && y == end_y {
            part1 = lost as u64;
            break;
        }

        let bitmap_idx = 2 * (y as usize * grid.width() + x as usize) + vertical as usize;
        if visited.get(bitmap_idx) {
            continue;
        }
        visited.set(bitmap_idx);

        let x_mult = !vertical as i32;
        let y_mult = vertical as i32;
        let mut loss = 0;
        for i in 1..=3 {
            let next_x = x + i * x_mult;
            let next_y = y + i * y_mult;
            if let Some(tile_loss) = grid.get_tile(next_x, next_y) {
                loss += tile_loss;
                heap.push((Reverse(lost + loss), next_x, next_y, !vertical));
            } else {
                break;
            }
        }

        let mut loss = 0;
        for i in 1..=3 {
            let next_x = x - i * x_mult;
            let next_y = y - i * y_mult;
            if let Some(tile_loss) = grid.get_tile(next_x, next_y) {
                loss += tile_loss;
                heap.push((Reverse(lost + loss), next_x, next_y, !vertical));
            } else {
                break;
            }
        }
    }

    let mut visited = BitSet::new(2 * grid.width() * grid.height());
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0, 0, false));
    heap.push((Reverse(0), 0, 0, true));

    let mut part2 = u64::MAX;
    while let Some((Reverse(lost), x, y, vertical)) = heap.pop() {
        if x == end_x && y == end_y {
            part2 = lost as u64;
            break;
        }

        let bitmap_idx = 2 * (y as usize * grid.width() + x as usize) + vertical as usize;
        if visited.get(bitmap_idx) {
            continue;
        }
        visited.set(bitmap_idx);

        let x_mult = !vertical as i32;
        let y_mult = vertical as i32;
        let mut loss = 0;
        for i in 1..=10 {
            let next_x = x + i * x_mult;
            let next_y = y + i * y_mult;
            if let Some(tile_loss) = grid.get_tile(next_x, next_y) {
                loss += tile_loss;
                if i >= 4 {
                    heap.push((Reverse(lost + loss), next_x, next_y, !vertical));
                }
            } else {
                break;
            }
        }

        let mut loss = 0;
        for i in 1..=10 {
            let next_x = x - i * x_mult;
            let next_y = y - i * y_mult;
            if let Some(tile_loss) = grid.get_tile(next_x, next_y) {
                loss += tile_loss;
                if i >= 4 {
                    heap.push((Reverse(lost + loss), next_x, next_y, !vertical));
                }
            } else {
                break;
            }
        }
    }

    Solution::from((part1, part2))
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
