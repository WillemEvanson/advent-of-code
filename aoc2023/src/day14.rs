use std::collections::HashMap;

use util::grid::Grid;
use util::Solution;

const ITERATIONS: u64 = 1_000_000_000;

pub fn solve(input: &str) -> Solution {
    let mut grid = Grid::new(
        input.lines().next().unwrap().len() as u32,
        input.lines().count() as u32,
    );
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.set(
                x as u32,
                y as u32,
                match c {
                    'O' => Tile::Round,
                    '#' => Tile::Cube,
                    '.' => Tile::Empty,
                    _ => panic!(),
                },
            );
        }
    }

    tilt_north(&mut grid);

    let mut part1 = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if let Some(Tile::Round) = grid.get_at(x, y) {
                part1 += (grid.height() - y) as u64;
            }
        }
    }

    // Part Two

    // Already tilted north
    tilt_west(&mut grid);
    tilt_south(&mut grid);
    tilt_east(&mut grid);

    let mut prior = HashMap::new();

    let mut found = false;
    let mut i: u64 = 1;
    while i < ITERATIONS {
        if !found {
            if let Some(&prior) = prior.get(&grid) {
                found = true;

                let diff = i - prior;
                let remaining = ITERATIONS - i;
                i += (remaining / diff) * diff;
            }

            prior.insert(grid.clone(), i);
        }

        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);

        i += 1;
    }

    let mut part2 = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if let Some(Tile::Round) = grid.get_at(x, y) {
                part2 += (grid.height() - y) as u64;
            }
        }
    }

    Solution::from((part1, part2))
}

fn tilt_north(grid: &mut Grid<Tile>) {
    for y in 1..grid.height() {
        for x in 0..grid.height() {
            if let Some(Tile::Round) = grid.get_at(x, y) {
                let mut current_y = y - 1;

                loop {
                    if let Some(Tile::Round | Tile::Cube) = grid.get_at(x, current_y) {
                        grid.set(x, y, Tile::Empty);
                        grid.set(x, current_y + 1, Tile::Round);
                        break;
                    }

                    if current_y > 0 {
                        current_y -= 1;
                    } else {
                        grid.set(x, y, Tile::Empty);
                        grid.set(x, current_y, Tile::Round);
                        break;
                    }
                }
            }
        }
    }
}

fn tilt_south(grid: &mut Grid<Tile>) {
    for y in (0..grid.height() - 1).rev() {
        for x in 0..grid.height() {
            if let Some(Tile::Round) = grid.get_at(x, y) {
                let mut current_y = y + 1;

                loop {
                    if let Some(Tile::Round | Tile::Cube) = grid.get_at(x, current_y) {
                        grid.set(x, y, Tile::Empty);
                        grid.set(x, current_y - 1, Tile::Round);
                        break;
                    }

                    if current_y < grid.height() - 1 {
                        current_y += 1;
                    } else {
                        grid.set(x, y, Tile::Empty);
                        grid.set(x, current_y, Tile::Round);
                        break;
                    }
                }
            }
        }
    }
}

fn tilt_east(grid: &mut Grid<Tile>) {
    for y in 0..grid.height() {
        for x in (0..grid.width() - 1).rev() {
            if let Some(Tile::Round) = grid.get_at(x, y) {
                let mut current_x = x + 1;

                loop {
                    if let Some(Tile::Round | Tile::Cube) = grid.get_at(current_x, y) {
                        grid.set(x, y, Tile::Empty);
                        grid.set(current_x - 1, y, Tile::Round);
                        break;
                    }

                    if current_x < grid.width() - 1 {
                        current_x += 1;
                    } else {
                        grid.set(x, y, Tile::Empty);
                        grid.set(current_x, y, Tile::Round);
                        break;
                    }
                }
            }
        }
    }
}

fn tilt_west(grid: &mut Grid<Tile>) {
    for y in 0..grid.height() {
        for x in 1..grid.width() {
            if let Some(Tile::Round) = grid.get_at(x, y) {
                let mut current_x = x - 1;

                loop {
                    if let Some(Tile::Round | Tile::Cube) = grid.get_at(current_x, y) {
                        grid.set(x, y, Tile::Empty);
                        grid.set(current_x + 1, y, Tile::Round);
                        break;
                    }

                    if current_x > 0 {
                        current_x -= 1;
                    } else {
                        grid.set(x, y, Tile::Empty);
                        grid.set(current_x, y, Tile::Round);
                        break;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Default, Hash, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Round,
    Cube,
    #[default]
    Empty,
}
