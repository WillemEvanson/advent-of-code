use std::collections::HashMap;

const ITERATIONS: u64 = 1_000_000_000;

use util::Solution;

pub fn solve(input: &str) -> Solution {
    // Part One
    let mut grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Tile::Round,
                    '#' => Tile::Cube,
                    '.' => Tile::Empty,
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    tilt_north(&mut grid);

    let mut part1 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if let Tile::Round = grid[y][x] {
                part1 += (grid.len() - y) as u64;
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
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if let Tile::Round = grid[y][x] {
                part2 += (grid.len() - y) as u64;
            }
        }
    }

    Solution::from((part1, part2))
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Round,
    Cube,
    Empty,
}

fn tilt_north(grid: &mut [Vec<Tile>]) {
    for i in 1..grid.len() {
        for j in 0..grid.len() {
            if let Tile::Round = grid[i][j] {
                let mut current = i - 1;

                loop {
                    if grid[current][j] != Tile::Empty {
                        grid[i][j] = Tile::Empty;
                        grid[current + 1][j] = Tile::Round;
                        break;
                    }

                    if current > 0 {
                        current -= 1;
                    } else {
                        grid[i][j] = Tile::Empty;
                        grid[current][j] = Tile::Round;
                        break;
                    }
                }
            }
        }
    }
}

fn tilt_south(grid: &mut [Vec<Tile>]) {
    for i in (0..grid.len() - 1).rev() {
        for j in 0..grid.len() {
            if let Tile::Round = grid[i][j] {
                let mut current = i + 1;

                loop {
                    if grid[current][j] != Tile::Empty {
                        grid[i][j] = Tile::Empty;
                        grid[current - 1][j] = Tile::Round;
                        break;
                    }

                    if current < grid.len() - 1 {
                        current += 1;
                    } else {
                        grid[i][j] = Tile::Empty;
                        grid[current][j] = Tile::Round;
                        break;
                    }
                }
            }
        }
    }
}

fn tilt_east(grid: &mut [Vec<Tile>]) {
    for i in 0..grid.len() {
        for j in (0..grid.len() - 1).rev() {
            if let Tile::Round = grid[i][j] {
                let mut current = j + 1;

                loop {
                    if grid[i][current] != Tile::Empty {
                        grid[i][j] = Tile::Empty;
                        grid[i][current - 1] = Tile::Round;
                        break;
                    }

                    if current < grid.len() - 1 {
                        current += 1;
                    } else {
                        grid[i][j] = Tile::Empty;
                        grid[i][current] = Tile::Round;

                        break;
                    }
                }
            }
        }
    }
}

fn tilt_west(grid: &mut [Vec<Tile>]) {
    for i in 0..grid.len() {
        for j in 1..grid.len() {
            if let Tile::Round = grid[i][j] {
                let mut current = j - 1;

                loop {
                    if grid[i][current] != Tile::Empty {
                        grid[i][j] = Tile::Empty;
                        grid[i][current + 1] = Tile::Round;
                        break;
                    }

                    if current > 0 {
                        current -= 1;
                    } else {
                        grid[i][j] = Tile::Empty;
                        grid[i][current] = Tile::Round;
                        break;
                    }
                }
            }
        }
    }
}
