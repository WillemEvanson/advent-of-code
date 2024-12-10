use util::bit_set::BitSet;
use util::grid::Grid;
use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut trailheads = Vec::new();
    let mut grid = Grid::new(
        input.lines().next().unwrap().len() as u32,
        input.lines().count() as u32,
    );

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                let digit = c.to_digit(10).unwrap();
                grid.set(x as u32, y as u32, Some(digit));
                if digit == 0 {
                    trailheads.push((x as u32, y as u32));
                }
            } else {
                grid.set(x as u32, y as u32, None);
            }
        }
    }

    let mut part1 = 0;
    for &(x, y) in trailheads.iter() {
        let mut stack = vec![(x, y)];
        let mut visited = BitSet::new(grid.width() * grid.height());
        while let Some((x, y)) = stack.pop() {
            let visited_idx = grid.get_index_unchecked(x, y) as u32;
            if visited.get(visited_idx) {
                continue;
            }
            visited.set(visited_idx);

            let current_height = grid.get_at(x, y).unwrap().unwrap();
            if current_height == 9 {
                part1 += 1;
                continue;
            }

            if let Some((new_x, new_y)) = grid.get_offset(x, y, -1, 0) {
                if let Some(height) = grid.get_at(new_x, new_y).unwrap() {
                    if current_height + 1 == *height {
                        stack.push((new_x, new_y));
                    }
                }
            }

            if let Some((new_x, new_y)) = grid.get_offset(x, y, 1, 0) {
                if let Some(height) = grid.get_at(new_x, new_y).unwrap() {
                    if current_height + 1 == *height {
                        stack.push((new_x, new_y));
                    }
                }
            }

            if let Some((new_x, new_y)) = grid.get_offset(x, y, 0, -1) {
                if let Some(height) = grid.get_at(new_x, new_y).unwrap() {
                    if current_height + 1 == *height {
                        stack.push((new_x, new_y));
                    }
                }
            }

            if let Some((new_x, new_y)) = grid.get_offset(x, y, 0, 1) {
                if let Some(height) = grid.get_at(new_x, new_y).unwrap() {
                    if current_height + 1 == *height {
                        stack.push((new_x, new_y));
                    }
                }
            }
        }
    }

    let mut part2 = 0;
    let mut stack = trailheads.clone();
    while let Some((x, y)) = stack.pop() {
        let current_height = grid.get_at(x, y).unwrap().unwrap();
        if current_height == 9 {
            part2 += 1;
            continue;
        }

        if let Some((new_x, new_y)) = grid.get_offset(x, y, -1, 0) {
            if let Some(height) = grid.get_at(new_x, new_y).unwrap() {
                if current_height + 1 == *height {
                    stack.push((new_x, new_y));
                }
            }
        }

        if let Some((new_x, new_y)) = grid.get_offset(x, y, 1, 0) {
            if let Some(height) = grid.get_at(new_x, new_y).unwrap() {
                if current_height + 1 == *height {
                    stack.push((new_x, new_y));
                }
            }
        }

        if let Some((new_x, new_y)) = grid.get_offset(x, y, 0, -1) {
            if let Some(height) = grid.get_at(new_x, new_y).unwrap() {
                if current_height + 1 == *height {
                    stack.push((new_x, new_y));
                }
            }
        }

        if let Some((new_x, new_y)) = grid.get_offset(x, y, 0, 1) {
            if let Some(height) = grid.get_at(new_x, new_y).unwrap() {
                if current_height + 1 == *height {
                    stack.push((new_x, new_y));
                }
            }
        }
    }

    Solution::from((part1, part2))
}
