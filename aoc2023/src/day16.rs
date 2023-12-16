use util::bit_set::BitSet;
use util::direction::Direction;
use util::grid::Grid;
use util::Solution;

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
                    '.' => Tile::Empty,
                    '|' => Tile::VSplitter,
                    '-' => Tile::HSplitter,
                    '/' => Tile::RMirror,
                    '\\' => Tile::LMirror,
                    _ => panic!(),
                },
            );
        }
    }

    let part1 = traverse(&grid, 0, 0, Direction::Right);

    let mut max = 0;
    for i in 0..grid.width() {
        let energized = traverse(&grid, i, 0, Direction::Down);
        max = u64::max(max, energized);
    }
    for i in 0..grid.width() {
        let energized = traverse(&grid, i, grid.height() - 1, Direction::Up);
        max = u64::max(max, energized);
    }

    for i in 0..grid.height() {
        let energized = traverse(&grid, 0, i, Direction::Right);
        max = u64::max(max, energized);
    }
    for i in 0..grid.height() {
        let energized = traverse(&grid, grid.width() - 1, i, Direction::Left);
        max = u64::max(max, energized);
    }
    let part2 = max;

    Solution::from((part1, part2))
}

fn traverse(grid: &Grid<Tile>, start_x: u32, start_y: u32, start_direction: Direction) -> u64 {
    let mut energized = BitSet::new(grid.width() * grid.height());
    let mut visited = BitSet::new(4 * grid.width() * grid.height());
    let mut rays = vec![(start_x, start_y, start_direction)];
    while let Some((mut ray_x, mut ray_y, mut direction)) = rays.pop() {
        let mut offset_x = 0;
        let mut offset_y = 0;

        while let Some((new_x, new_y)) = grid.get_offset(ray_x, ray_y, offset_x, offset_y) {
            ray_x = new_x;
            ray_y = new_y;

            let tile_idx = grid.get_index_unchecked(ray_x, ray_y) as u32;
            let visited_idx = 4 * tile_idx + direction as u32;
            if visited.get(visited_idx) {
                break;
            }
            visited.set(visited_idx);
            energized.set(tile_idx);

            match grid.get_at(ray_x, ray_y) {
                Some(Tile::Empty) => (offset_x, offset_y) = direction.offset(),
                Some(Tile::HSplitter) => {
                    if let Direction::Left | Direction::Right = direction {
                        (offset_x, offset_y) = direction.offset()
                    } else {
                        rays.push((ray_x, ray_y, Direction::Left));
                        rays.push((ray_x, ray_y, Direction::Right));
                        break;
                    }
                }
                Some(Tile::VSplitter) => {
                    if let Direction::Up | Direction::Down = direction {
                        (offset_x, offset_y) = direction.offset()
                    } else {
                        rays.push((ray_x, ray_y, Direction::Up));
                        rays.push((ray_x, ray_y, Direction::Down));
                        break;
                    }
                }
                Some(Tile::RMirror) => match direction {
                    Direction::Right => {
                        direction = Direction::Up;
                        offset_y = -1;
                        offset_x = 0;
                    }
                    Direction::Down => {
                        direction = Direction::Left;
                        offset_x = -1;
                        offset_y = 0;
                    }
                    Direction::Left => {
                        direction = Direction::Down;
                        offset_y = 1;
                        offset_x = 0;
                    }
                    Direction::Up => {
                        direction = Direction::Right;
                        offset_x = 1;
                        offset_y = 0;
                    }
                },
                Some(Tile::LMirror) => match direction {
                    Direction::Right => {
                        direction = Direction::Down;
                        offset_y = 1;
                        offset_x = 0;
                    }
                    Direction::Down => {
                        direction = Direction::Right;
                        offset_x = 1;
                        offset_y = 0;
                    }
                    Direction::Left => {
                        direction = Direction::Up;
                        offset_y = -1;
                        offset_x = 0;
                    }
                    Direction::Up => {
                        direction = Direction::Left;
                        offset_x = -1;
                        offset_y = 0;
                    }
                },
                None => break,
            }
        }
    }
    energized.count() as u64
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Tile {
    VSplitter,
    HSplitter,
    RMirror,
    LMirror,
    #[default]
    Empty,
}
