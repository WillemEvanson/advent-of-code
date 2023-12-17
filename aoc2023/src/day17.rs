use std::cmp::Reverse;
use std::collections::BinaryHeap;

use util::bit_set::BitSet;
use util::grid::Grid;
use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut grid = Grid::new(
        input.lines().next().unwrap().len() as u32,
        input.lines().count() as u32,
    );

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.set(x as u32, y as u32, c.to_digit(10).unwrap());
        }
    }

    let end_x = grid.width() - 1;
    let end_y = grid.height() - 1;

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

        let bitset_idx = 2 * grid.get_index_unchecked(x, y) as u32 + vertical as u32;
        if visited.get(bitset_idx) {
            continue;
        }
        visited.set(bitset_idx);

        let x_mult = !vertical as i32;
        let y_mult = vertical as i32;
        let mut loss = 0;
        for i in 1..=3 {
            let offset_x = i * x_mult;
            let offset_y = i * y_mult;
            if let Some((new_x, new_y)) = grid.get_offset(x, y, offset_x, offset_y) {
                loss += *grid.get_at(new_x, new_y).unwrap();
                heap.push((Reverse(lost + loss), new_x, new_y, !vertical));
            } else {
                break;
            }
        }

        let mut loss = 0;
        for i in 1..=3 {
            let offset_x = -i * x_mult;
            let offset_y = -i * y_mult;
            if let Some((new_x, new_y)) = grid.get_offset(x, y, offset_x, offset_y) {
                loss += *grid.get_at(new_x, new_y).unwrap();
                heap.push((Reverse(lost + loss), new_x, new_y, !vertical));
            } else {
                break;
            }
        }
    }

    visited.clear();
    heap.clear();
    heap.push((Reverse(0), 0, 0, false));
    heap.push((Reverse(0), 0, 0, true));

    let mut part2 = u64::MAX;
    while let Some((Reverse(lost), x, y, vertical)) = heap.pop() {
        if x == end_x && y == end_y {
            part2 = lost as u64;
            break;
        }

        let bitset_idx = 2 * grid.get_index_unchecked(x, y) as u32 + vertical as u32;
        if visited.get(bitset_idx) {
            continue;
        }
        visited.set(bitset_idx);

        let x_mult = !vertical as i32;
        let y_mult = vertical as i32;
        let mut loss = 0;
        for i in 1..=10 {
            let offset_x = i * x_mult;
            let offset_y = i * y_mult;
            if let Some((new_x, new_y)) = grid.get_offset(x, y, offset_x, offset_y) {
                loss += *grid.get_at(new_x, new_y).unwrap();
                if i >= 4 {
                    heap.push((Reverse(lost + loss), new_x, new_y, !vertical));
                }
            } else {
                break;
            }
        }

        let mut loss = 0;
        for i in 1..=10 {
            let offset_x = -i * x_mult;
            let offset_y = -i * y_mult;
            if let Some((new_x, new_y)) = grid.get_offset(x, y, offset_x, offset_y) {
                loss += *grid.get_at(new_x, new_y).unwrap();
                if i >= 4 {
                    heap.push((Reverse(lost + loss), new_x, new_y, !vertical));
                }
            } else {
                break;
            }
        }
    }

    Solution::from((part1, part2))
}
