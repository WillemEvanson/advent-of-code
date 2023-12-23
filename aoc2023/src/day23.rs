use std::collections::{HashMap, HashSet};

use util::direction::Direction;
use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut grid = util::grid::Grid::new(
        input.lines().next().unwrap().len() as u32,
        input.lines().count() as u32,
    );

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '#' => Tile::Forest,
                '.' => Tile::Path,
                '<' => Tile::Slope(Direction::Left),
                '>' => Tile::Slope(Direction::Right),
                '^' => Tile::Slope(Direction::Up),
                'v' => Tile::Slope(Direction::Down),
                _ => panic!(),
            };
            grid.set(x as u32, y as u32, tile);
        }
    }

    let mut start_x = 0;
    let start_y = 0;
    for i in 0..grid.width() {
        if let Some(Tile::Path) = grid.get_at(i, start_y) {
            start_x = i;
            break;
        }
    }

    let mut end_x = 0;
    let end_y = grid.height() - 1;
    for i in 0..grid.width() {
        if let Some(Tile::Path) = grid.get_at(i, end_y) {
            end_x = i;
            break;
        }
    }

    let mut part1 = 0;
    let mut to_visit = vec![(start_x, start_y, Direction::Down, 0, HashSet::new())];
    while let Some((x, y, direction, count, mut visited)) = to_visit.pop() {
        if x == end_x && y == end_y {
            part1 = u64::max(part1, count);
            continue;
        }

        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        if let Some(&Tile::Slope(new_direction)) = grid.get_at(x, y) {
            let (offset_x, offset_y) = new_direction.offset();
            let (new_x, new_y) = grid.get_offset(x, y, offset_x, offset_y).unwrap();
            to_visit.push((new_x, new_y, new_direction, count + 1, visited.clone()));
        } else if let Some(Tile::Path) = grid.get_at(x, y) {
            let (offset_x, offset_y) = direction.offset();
            let (new_x, new_y) = grid.get_offset(x, y, offset_x, offset_y).unwrap();
            to_visit.push((new_x, new_y, direction, count + 1, visited.clone()));

            let new_direction = direction.left();
            let (offset_x, offset_y) = new_direction.offset();
            let (new_x, new_y) = grid.get_offset(x, y, offset_x, offset_y).unwrap();
            to_visit.push((new_x, new_y, new_direction, count + 1, visited.clone()));

            let new_direction = direction.right();
            let (offset_x, offset_y) = new_direction.offset();
            let (new_x, new_y) = grid.get_offset(x, y, offset_x, offset_y).unwrap();
            to_visit.push((new_x, new_y, new_direction, count + 1, visited.clone()));
        }
    }

    let mut visited = HashSet::new();
    let mut paths: HashMap<_, HashMap<_, _>> = HashMap::new();
    let mut todo = vec![(start_x, start_y, Direction::Down)];
    while let Some((mut x, mut y, mut direction)) = todo.pop() {
        if visited.contains(&(x, y, direction)) {
            continue;
        }
        visited.insert((x, y, direction));

        let start_x = x;
        let start_y = y;

        let (offset_x, offset_y) = direction.offset();
        (x, y) = grid.get_offset(x, y, offset_x, offset_y).unwrap();

        let mut path_length = 0;
        loop {
            let mut straight = false;
            let (offset_x, offset_y) = direction.offset();
            if let Some(Tile::Path | Tile::Slope(_)) = grid.get_at_offset(x, y, offset_x, offset_y)
            {
                straight = true;
            }

            let mut left = false;
            let (offset_x, offset_y) = direction.left().offset();
            if let Some(Tile::Path | Tile::Slope(_)) = grid.get_at_offset(x, y, offset_x, offset_y)
            {
                left = true;
            }

            let mut right = false;
            let (offset_x, offset_y) = direction.right().offset();
            if let Some(Tile::Path | Tile::Slope(_)) = grid.get_at_offset(x, y, offset_x, offset_y)
            {
                right = true;
            }

            let count = straight as u8 + left as u8 + right as u8;
            match count.cmp(&1) {
                std::cmp::Ordering::Equal => {
                    visited.insert((x, y, direction));
                    path_length += 1;
                    if straight {
                        let (offset_x, offset_y) = direction.offset();
                        (x, y) = grid.get_offset(x, y, offset_x, offset_y).unwrap();
                    }
                    if left {
                        direction = direction.left();
                        let (offset_x, offset_y) = direction.offset();
                        (x, y) = grid.get_offset(x, y, offset_x, offset_y).unwrap();
                    }
                    if right {
                        direction = direction.right();
                        let (offset_x, offset_y) = direction.offset();
                        (x, y) = grid.get_offset(x, y, offset_x, offset_y).unwrap();
                    }
                }
                std::cmp::Ordering::Greater => {
                    if straight {
                        todo.push((x, y, direction))
                    }
                    if left {
                        todo.push((x, y, direction.left()))
                    }
                    if right {
                        todo.push((x, y, direction.right()))
                    }
                    break;
                }
                std::cmp::Ordering::Less => break,
            }
        }

        if path_length != 0 {
            let start = (start_x, start_y);
            let end = (x, y);

            paths.entry(start).or_default().insert(end, path_length + 1);
            paths.entry(end).or_default().insert(start, path_length + 1);
        }
    }

    let mut part2 = 0;
    let mut to_visit = vec![(start_x, start_y, 0, HashSet::new())];
    while let Some((x, y, count, mut visited)) = to_visit.pop() {
        if x == end_x && y == end_y {
            part2 = u64::max(part2, count);
            continue;
        }

        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        for (&(end_x, end_y), length) in paths.get(&(x, y)).unwrap() {
            to_visit.push((end_x, end_y, count + length, visited.clone()));
        }
    }

    Solution::from((part1, part2))
}

#[derive(Debug, Default, Hash, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Slope(Direction),
    Forest,
    #[default]
    Path,
}
