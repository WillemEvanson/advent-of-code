use util::bit_set::BitSet;
use util::grid::Grid;
use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut grid = Grid::new(
        input.lines().next().unwrap().len() as u32,
        input.lines().count() as u32,
    );

    let mut start_x = 0;
    let mut start_y = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '|' => Tile::Vertical,
                '-' => Tile::Horizontal,
                'L' => Tile::NE,
                'J' => Tile::NW,
                '7' => Tile::SW,
                'F' => Tile::SE,
                'S' => {
                    start_x = x as u32;
                    start_y = y as u32;
                    Tile::Ground
                }
                '.' => Tile::Ground,
                _ => panic!("invalid character in input"),
            };
            grid.set(x as u32, y as u32, tile);
        }
    }

    // Determine what type of pipe is underneath the starting tile
    for kind in [
        Tile::Vertical,
        Tile::Horizontal,
        Tile::NE,
        Tile::NW,
        Tile::SW,
        Tile::SE,
    ] {
        if let Some((first_offset, second_offset)) = kind.first_offset().zip(kind.second_offset()) {
            let mut found_first = false;

            if let Some((tile_x, tile_y)) =
                grid.get_offset(start_x, start_y, first_offset.0, first_offset.1)
            {
                if let Some((x, y)) = grid
                    .get_at(tile_x, tile_y)
                    .and_then(|tile| (tile.first_offset().zip(tile.second_offset())))
                {
                    if let Some((new_x, new_y)) = grid.get_offset(tile_x, tile_y, x.0, x.1) {
                        if new_x == start_x && new_y == start_y {
                            found_first = true;
                        }
                    }

                    if let Some((new_x, new_y)) = grid.get_offset(tile_x, tile_y, y.0, y.1) {
                        if new_x == start_x && new_y == start_y {
                            found_first = true;
                        }
                    }
                }
            }

            let mut found_second = false;
            if let Some((tile_x, tile_y)) =
                grid.get_offset(start_x, start_y, second_offset.0, second_offset.1)
            {
                if let Some((x, y)) = grid
                    .get_at(tile_x, tile_y)
                    .and_then(|tile| (tile.first_offset().zip(tile.second_offset())))
                {
                    if let Some((new_x, new_y)) = grid.get_offset(tile_x, tile_y, x.0, x.1) {
                        if new_x == start_x && new_y == start_y {
                            found_second = true;
                        }
                    }

                    if let Some((new_x, new_y)) = grid.get_offset(tile_x, tile_y, y.0, y.1) {
                        if new_x == start_x && new_y == start_y {
                            found_second = true;
                        }
                    }
                }
            }

            if found_first && found_second {
                grid.set(start_x, start_y, kind);
                break;
            }
        }
    }
    assert!(!matches!(grid.get_at(start_x, start_y), Some(Tile::Ground)));

    let mut depth = 0;
    let mut next = Vec::new();
    let mut current = vec![(start_x, start_y)];
    let mut visited = BitSet::new(grid.width() * grid.height());
    while !current.is_empty() {
        while let Some((x, y)) = current.pop() {
            let Some(tile) = grid.get_at(x, y) else {
                continue;
            };

            for &(offset_x, offset_y) in tile
                .first_offset()
                .iter()
                .chain(tile.second_offset().iter())
            {
                let Some((new_x, new_y)) = grid.get_offset(x, y, offset_x, offset_y) else {
                    continue;
                };

                let visited_idx = grid.get_index_unchecked(new_x, new_y) as u32;
                if !visited.get(visited_idx) {
                    visited.set(visited_idx);
                    next.push((new_x, new_y));
                }
            }
        }
        std::mem::swap(&mut current, &mut next);
        depth += 1;
    }
    let part1 = depth - 1;

    // We can find the total area enclosed by the walls
    let mut enclosed = 0;
    let in_loop = visited;
    for y in 0..grid.height() {
        let mut x = 0;
        let mut inside = false;
        let mut reverse_on_up = false;
        while x < grid.width() {
            if in_loop.get(grid.get_index_unchecked(x, y) as u32) {
                if let Some(tile) = grid.get_at(x, y) {
                    match tile {
                        Tile::NE => {
                            reverse_on_up = false;
                        }
                        Tile::SE => {
                            reverse_on_up = true;
                        }
                        Tile::NW => {
                            if reverse_on_up {
                                inside = !inside;
                            }
                        }
                        Tile::SW => {
                            if !reverse_on_up {
                                inside = !inside;
                            }
                        }
                        Tile::Vertical => inside = !inside,
                        Tile::Horizontal | Tile::Ground => (),
                    }
                }
            } else if inside {
                enclosed += 1;
            }

            x += 1;
        }
    }
    let part2 = enclosed;

    Solution::from((part1, part2))
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Vertical,
    Horizontal,
    NE,
    NW,
    SW,
    SE,
    #[default]
    Ground,
}

impl Tile {
    fn first_offset(&self) -> Option<(i32, i32)> {
        match self {
            Self::Vertical => Some((0, -1)),
            Self::Horizontal => Some((-1, 0)),
            Self::NE => Some((0, -1)),
            Self::NW => Some((0, -1)),
            Self::SW => Some((0, 1)),
            Self::SE => Some((0, 1)),
            Self::Ground => None,
        }
    }

    fn second_offset(&self) -> Option<(i32, i32)> {
        match self {
            Self::Vertical => Some((0, 1)),
            Self::Horizontal => Some((1, 0)),
            Self::NE => Some((1, 0)),
            Self::NW => Some((-1, 0)),
            Self::SW => Some((-1, 0)),
            Self::SE => Some((1, 0)),
            Self::Ground => None,
        }
    }
}
