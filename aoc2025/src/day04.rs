use util::grid::Grid;
use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut part1 = 0;
    let mut part2 = 0;

    let height = input.lines().count() as u32;
    let width = input.lines().next().unwrap().len() as u32;

    let mut grid = Grid::new(width, height);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                grid.set(x as u32, y as u32, true);
            }
        }
    }

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if let Some(false) = grid.get_at(x, y) {
                continue;
            }

            let mut rolls = 0;
            for y_offset in -1..=1 {
                for x_offset in -1..=1 {
                    if x_offset == 0 && y_offset == 0 {
                        continue;
                    }

                    if let Some(true) = grid.get_at_offset(x, y, x_offset, y_offset) {
                        rolls += 1;
                    }
                }
            }
            if rolls < 4 {
                part1 += 1;
            }
        }
    }

    let mut changed = true;
    while changed {
        changed = false;

        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if let Some(false) = grid.get_at(x, y) {
                    continue;
                }

                let mut rolls = 0;
                for y_offset in -1..=1 {
                    for x_offset in -1..=1 {
                        if x_offset == 0 && y_offset == 0 {
                            continue;
                        }

                        if let Some(true) = grid.get_at_offset(x, y, x_offset, y_offset) {
                            rolls += 1;
                        }
                    }
                }
                if rolls < 4 {
                    grid.set(x, y, false);
                    changed = true;
                    part2 += 1;
                }
            }
        }
    }

    Solution::from((part1, part2))
}
