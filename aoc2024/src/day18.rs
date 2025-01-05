use std::collections::HashSet;

use util::grid::Grid;
use util::union_find::UnionFind;
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

    for &(x, y) in bytes.iter().skip(1024) {
        grid.set(x, y, true);
    }

    let start_idx = grid.get_index_unchecked(start_x, start_y) as u32;
    let end_idx = grid.get_index_unchecked(end_x, end_y) as u32;
    let mut unionfind = UnionFind::new(grid.width() * grid.height());
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if let Some(true) = grid.get_at(x, y) {
                continue;
            }
            let element_idx = grid.get_index(x, y).unwrap() as u32;

            if let Some((new_x, new_y)) = grid
                .get_offset(x, y, -1, 0)
                .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
            {
                let other_idx = grid.get_index(new_x, new_y).unwrap() as u32;
                unionfind.merge(element_idx, other_idx);
            }

            if let Some((new_x, new_y)) = grid
                .get_offset(x, y, 1, 0)
                .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
            {
                let other_idx = grid.get_index(new_x, new_y).unwrap() as u32;
                unionfind.merge(element_idx, other_idx);
            }

            if let Some((new_x, new_y)) = grid
                .get_offset(x, y, 0, -1)
                .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
            {
                let other_idx = grid.get_index(new_x, new_y).unwrap() as u32;
                unionfind.merge(element_idx, other_idx);
            }

            if let Some((new_x, new_y)) = grid
                .get_offset(x, y, 0, 1)
                .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
            {
                let other_idx = grid.get_index(new_x, new_y).unwrap() as u32;
                unionfind.merge(element_idx, other_idx);
            }
        }
    }

    let mut part2 = String::new();
    for &(x, y) in bytes.iter().skip(1024).rev() {
        let element_idx = grid.get_index(x, y).unwrap() as u32;
        grid.set(x, y, false);

        if let Some((new_x, new_y)) = grid
            .get_offset(x, y, -1, 0)
            .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
        {
            let other_idx = grid.get_index(new_x, new_y).unwrap() as u32;
            unionfind.merge(element_idx, other_idx);
        }

        if let Some((new_x, new_y)) = grid
            .get_offset(x, y, 1, 0)
            .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
        {
            let other_idx = grid.get_index(new_x, new_y).unwrap() as u32;
            unionfind.merge(element_idx, other_idx);
        }

        if let Some((new_x, new_y)) = grid
            .get_offset(x, y, 0, -1)
            .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
        {
            let other_idx = grid.get_index(new_x, new_y).unwrap() as u32;
            unionfind.merge(element_idx, other_idx);
        }

        if let Some((new_x, new_y)) = grid
            .get_offset(x, y, 0, 1)
            .filter(|&(new_x, new_y)| grid.get_at(new_x, new_y).copied() == Some(false))
        {
            let other_idx = grid.get_index(new_x, new_y).unwrap() as u32;
            unionfind.merge(element_idx, other_idx);
        }

        if unionfind.find(start_idx) == unionfind.find(end_idx) {
            part2 = format!("{x},{y}");
            break;
        }
    }

    Solution::from((part1, part2))
}
