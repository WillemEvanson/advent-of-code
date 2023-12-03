use util::grid::Grid;
use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut grid = Grid::new(
        input.lines().next().unwrap().len() as u32,
        input.lines().count() as u32,
    );

    for (y, line) in input.lines().enumerate() {
        let mut iter = line.chars().enumerate();
        while let Some((x, block)) = iter.next() {
            match block {
                '0'..='9' => {
                    let digit = block.to_digit(10).unwrap();

                    let mut end = x;
                    let mut val = digit;
                    let mut cloned = iter.clone();
                    while let Some((cur, digit @ ('0'..='9'))) = cloned.next() {
                        val = val * 10 + digit.to_digit(10).unwrap();
                        iter.next();
                        end = cur;
                    }

                    let len = end as u32 - x as u32;
                    for i in x..=end {
                        grid.set(i as u32, y as u32, Tile::Num { val, len });
                    }
                }
                '*' => {
                    grid.set(x as u32, y as u32, Tile::Gear);
                }
                '.' => {
                    grid.set(x as u32, y as u32, Tile::None);
                }
                _ => {
                    grid.set(x as u32, y as u32, Tile::Symbol);
                }
            };
        }
    }

    let mut part1 = 0;
    let mut part2 = 0;
    for y in 0..grid.height() {
        let mut x = 0;
        while let Some(&tile) = grid.get_at(x, y) {
            match tile {
                Tile::None | Tile::Symbol => (),
                Tile::Num { val, len } => {
                    let mut symbol = false;
                    for offset_x in -1..=(len + 1) as i32 {
                        if let Some(Tile::Symbol | Tile::Gear) =
                            grid.get_at_offset(x, y, offset_x, -1)
                        {
                            symbol = true;
                            break;
                        }
                    }
                    if let Some(Tile::Symbol | Tile::Gear) = grid.get_at_offset(x, y, -1, 0) {
                        symbol = true;
                    }
                    if let Some(Tile::Symbol | Tile::Gear) = grid.get_at_offset(x + len, y, 1, 0) {
                        symbol = true;
                    }
                    for offset_x in -1..=(len + 1) as i32 {
                        if let Some(Tile::Symbol | Tile::Gear) =
                            grid.get_at_offset(x, y, offset_x, 1)
                        {
                            symbol = true;
                            break;
                        }
                    }
                    if symbol {
                        part1 += val as u64;
                    }

                    x += len;
                }
                Tile::Gear => {
                    let mut numbers = Vec::new();
                    for x in x.saturating_sub(1)..=x + 1 {
                        if let Some(&Tile::Num { val, .. }) = grid.get_at(x, y.saturating_sub(1)) {
                            numbers.push(val);
                        }
                    }
                    if let Some(&Tile::Num { val, .. }) = grid.get_at(x.saturating_sub(1), y) {
                        numbers.push(val);
                    }
                    if let Some(&Tile::Num { val, .. }) = grid.get_at(x + 1, y) {
                        numbers.push(val);
                    }
                    for x in x.saturating_sub(1)..=x + 1 {
                        if let Some(&Tile::Num { val, .. }) = grid.get_at(x, y + 1) {
                            numbers.push(val);
                        }
                    }
                    numbers.sort_unstable();
                    numbers.dedup();

                    if numbers.len() == 2 {
                        part2 += numbers[0] as u64 * numbers[1] as u64;
                    }
                }
            }
            x += 1;
        }
    }

    Solution::from((part1, part2))
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Num {
        val: u32,
        len: u32,
    },
    Symbol,
    Gear,
    #[default]
    None,
}
