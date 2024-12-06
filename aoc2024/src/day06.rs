use std::collections::HashMap;

use util::bit_set::BitSet;
use util::direction::Direction;
use util::grid::Grid;
use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut start_x = 0;
    let mut start_y = 0;
    let mut grid = Grid::new(
        input.lines().next().unwrap().len() as u32,
        input.lines().count() as u32,
    );
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let empty = match c {
                '.' => true,
                '^' => {
                    start_x = x as u32;
                    start_y = y as u32;
                    true
                }
                '#' => false,
                _ => panic!(),
            };
            grid.set(x as u32, y as u32, empty);
        }
    }

    // Part 1
    let mut guard_x = start_x;
    let mut guard_y = start_y;
    let mut guard_direction = Direction::Up;

    let mut jump_set = HashMap::new();
    let mut visited = BitSet::new(grid.width() * grid.height());
    let mut path = Vec::new();
    'outer: loop {
        // Compute how much to jump forward
        let old_x = guard_x;
        let old_y = guard_y;
        let old_direction = guard_direction;
        loop {
            visited.set(grid.get_index_unchecked(guard_x, guard_y) as u32);
            let (offset_x, offset_y) = guard_direction.offset();
            if let Some((new_x, new_y)) = grid.get_offset(guard_x, guard_y, offset_x, offset_y) {
                if *grid.get_at(new_x, new_y).unwrap() {
                    path.push((guard_x, guard_y, guard_direction));
                    guard_x = new_x;
                    guard_y = new_y;
                } else {
                    break;
                }
            } else {
                jump_set.insert(
                    (old_x, old_y, old_direction),
                    (guard_x, guard_y, guard_direction),
                );
                break 'outer;
            }
        }

        // Turn to new direction
        loop {
            guard_direction = guard_direction.right();
            let (offset_x, offset_y) = guard_direction.offset();
            if let Some((new_x, new_y)) = grid.get_offset(guard_x, guard_y, offset_x, offset_y) {
                if *grid.get_at(new_x, new_y).unwrap() {
                    break;
                }
            } else {
                break;
            }
        }
        jump_set.insert(
            (old_x, old_y, old_direction),
            (guard_x, guard_y, guard_direction),
        );
    }
    let part1 = visited.count() as u64;

    // Part 2
    let mut part2 = 0;
    for i in visited
        .iter()
        .enumerate()
        .filter_map(|(i, visited)| if visited { Some(i as u32) } else { None })
    {
        let x = i % grid.width();
        let y = i / grid.width();
        if x == start_x && y == start_y {
            continue;
        }

        grid.set(x, y, false);

        let mut guard_x = start_x;
        let mut guard_y = start_y;
        let mut guard_direction = Direction::Up;
        let mut visited = BitSet::new(4 * grid.width() * grid.height());
        let found_loop = loop {
            let visited_index =
                4 * grid.get_index_unchecked(guard_x, guard_y) as u32 + guard_direction as u32;
            if visited.get(visited_index) {
                break true;
            }
            visited.set(visited_index);

            if guard_x != x && guard_y != y {
                if let Some(&(new_x, new_y, new_direction)) =
                    jump_set.get(&(guard_x, guard_y, guard_direction))
                {
                    if grid.within(new_x, new_y) {
                        guard_x = new_x;
                        guard_y = new_y;
                        guard_direction = new_direction;
                        continue;
                    } else {
                        break false;
                    }
                }
            }

            let (offset_x, offset_y) = guard_direction.offset();
            if let Some((new_x, new_y)) = grid.get_offset(guard_x, guard_y, offset_x, offset_y) {
                if *grid.get_at(new_x, new_y).unwrap() {
                    guard_x = new_x;
                    guard_y = new_y;
                } else {
                    guard_direction = guard_direction.right();
                }
            } else {
                break false;
            }
        };

        grid.set(x, y, true);
        if found_loop {
            part2 += 1;
        }
    }

    Solution::from((part1, part2))
}
