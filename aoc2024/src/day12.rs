use std::collections::HashMap;

use util::grid::Grid;
use util::union_find::UnionFind;
use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut grid = Grid::new(
        input.lines().next().unwrap().len() as u32,
        input.lines().count() as u32,
    );

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.set(x as u32, y as u32, c);
        }
    }

    let mut regions = UnionFind::new(grid.width() * grid.height());
    let mut perimeters = vec![0; (grid.width() * grid.height()) as usize];
    let mut chars = vec![char::default(); (grid.width() * grid.height()) as usize];
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let current = *grid.get_at(x, y).unwrap();
            let current_idx = grid.get_index_unchecked(x, y) as u32;

            let mut perimeter = 0;
            if let Some((new_x, new_y)) = grid
                .get_offset(x, y, -1, 0)
                .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(current))
            {
                let new_idx = grid.get_index_unchecked(new_x, new_y) as u32;
                regions.merge(current_idx, new_idx);
            } else {
                perimeter += 1;
            }

            if let Some((new_x, new_y)) = grid
                .get_offset(x, y, 1, 0)
                .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(current))
            {
                let new_idx = grid.get_index_unchecked(new_x, new_y) as u32;
                regions.merge(current_idx, new_idx);
            } else {
                perimeter += 1;
            }

            if let Some((new_x, new_y)) = grid
                .get_offset(x, y, 0, -1)
                .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(current))
            {
                let new_idx = grid.get_index_unchecked(new_x, new_y) as u32;
                regions.merge(current_idx, new_idx);
            } else {
                perimeter += 1;
            }

            if let Some((new_x, new_y)) = grid
                .get_offset(x, y, 0, 1)
                .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(current))
            {
                let new_idx = grid.get_index_unchecked(new_x, new_y) as u32;
                regions.merge(current_idx, new_idx);
            } else {
                perimeter += 1;
            }

            perimeters[current_idx as usize] = perimeter;
            chars[current_idx as usize] = current;
        }
    }

    // Find all of the horizontal sides
    let mut sides = vec![0; (grid.width() * grid.height()) as usize];
    for y in 0..grid.height() {
        let mut upper_edge = false;
        let mut bottom_edge = false;
        let mut current = *grid.get_at(0, y).unwrap();
        for x in 0..grid.width() {
            let current_idx = grid.get_index_unchecked(x, y);
            if let Some(&new) = grid.get_at(x, y) {
                if new != current {
                    bottom_edge = false;
                    upper_edge = false;
                    current = new;
                }
            }

            // Check upper neighbor
            if grid
                .get_at_offset(x, y, 0, -1)
                .filter(|&&neighbor| neighbor == current)
                .is_some()
            {
                upper_edge = false;
            } else if !upper_edge {
                sides[current_idx] += 1;
                upper_edge = true;
            }

            // Check bottom neighbor
            if grid
                .get_at_offset(x, y, 0, 1)
                .filter(|&&neighbor| neighbor == current)
                .is_some()
            {
                bottom_edge = false;
            } else if !bottom_edge {
                sides[current_idx] += 1;
                bottom_edge = true;
            }
        }
    }

    for x in 0..grid.width() {
        let mut left_edge = false;
        let mut right_edge = false;
        let mut current = *grid.get_at(x, 0).unwrap();
        for y in 0..grid.height() {
            let current_idx = grid.get_index_unchecked(x, y);
            if let Some(&new) = grid.get_at(x, y) {
                if new != current {
                    right_edge = false;
                    left_edge = false;
                    current = new;
                }
            }

            // Check left neighbor
            if grid
                .get_at_offset(x, y, -1, 0)
                .filter(|&&neighbor| neighbor == current)
                .is_some()
            {
                left_edge = false;
            } else if !left_edge {
                sides[current_idx] += 1;
                left_edge = true;
            }

            // Check right neighbor
            if grid
                .get_at_offset(x, y, 1, 0)
                .filter(|&&neighbor| neighbor == current)
                .is_some()
            {
                right_edge = false;
            } else if !right_edge {
                sides[current_idx] += 1;
                right_edge = true;
            }
        }
    }

    let mut map: HashMap<u32, (u32, u32, u32)> = HashMap::new();
    for i in 0..grid.width() * grid.height() {
        let perimeter = perimeters[i as usize];
        let sides = sides[i as usize];
        let root = regions.find(i);

        let (area_count, perimeter_count, sides_count) = map.entry(root).or_default();
        *perimeter_count += perimeter;
        *sides_count += sides;
        *area_count += 1;
    }

    let mut part1 = 0;
    let mut part2 = 0;
    for &(area, perimeter, sides) in map.values() {
        part1 += area as u64 * perimeter as u64;
        part2 += area as u64 * sides as u64;
    }

    Solution::from((part1, part2))
}
