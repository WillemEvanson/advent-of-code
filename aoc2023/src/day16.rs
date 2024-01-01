use util::Solution;

pub fn solve(input: &str) -> Solution {
    let grid = Grid::from_str(input);
    let part1 = traverse(grid.clone(), 0, 0, Dir::Right);

    let mut max = 0;
    for i in 0..grid.width() {
        let energized = traverse(grid.clone(), i as i32, 0, Dir::Down);
        max = u64::max(max, energized);
    }
    for i in 0..grid.width() {
        let energized = traverse(grid.clone(), i as i32, grid.height() as i32 - 1, Dir::Up);
        max = u64::max(max, energized);
    }

    for i in 0..grid.height() {
        let energized = traverse(grid.clone(), 0, i as i32, Dir::Right);
        max = u64::max(max, energized);
    }
    for i in 0..grid.height() {
        let energized = traverse(grid.clone(), grid.width() as i32 - 1, i as i32, Dir::Left);
        max = u64::max(max, energized);
    }
    let part2 = max;

    Solution::from((part1, part2))
}

fn traverse(grid: Grid, start_x: i32, start_y: i32, start_dir: Dir) -> u64 {
    let mut grid = grid;

    let mut rays = vec![(start_x, start_y, start_dir)];
    while let Some((mut ray_x, mut ray_y, mut dir)) = rays.pop() {
        loop {
            if grid.get_covered(ray_x, ray_y, dir) {
                break;
            }
            grid.set_covered(ray_x, ray_y, dir);
            grid.energize(ray_x, ray_y);

            match grid.get_tile(ray_x, ray_y) {
                Some(Tile::Empty) => {
                    let (offset_x, offset_y) = dir.offset();
                    ray_x += offset_x;
                    ray_y += offset_y;
                }
                Some(Tile::HSplitter) => {
                    if let Dir::Left | Dir::Right = dir {
                        let (offset_x, offset_y) = dir.offset();
                        ray_x += offset_x;
                        ray_y += offset_y;
                    } else {
                        rays.push((ray_x, ray_y, Dir::Left));
                        rays.push((ray_x, ray_y, Dir::Right));
                        break;
                    }
                }
                Some(Tile::VSplitter) => {
                    if let Dir::Up | Dir::Down = dir {
                        let (offset_x, offset_y) = dir.offset();
                        ray_x += offset_x;
                        ray_y += offset_y;
                    } else {
                        rays.push((ray_x, ray_y, Dir::Up));
                        rays.push((ray_x, ray_y, Dir::Down));
                        break;
                    }
                }
                Some(Tile::RMirror) => match dir {
                    Dir::Down => {
                        dir = Dir::Left;
                        ray_x -= 1;
                    }
                    Dir::Right => {
                        dir = Dir::Up;
                        ray_y -= 1;
                    }
                    Dir::Up => {
                        dir = Dir::Right;
                        ray_x += 1;
                    }
                    Dir::Left => {
                        dir = Dir::Down;
                        ray_y += 1;
                    }
                },
                Some(Tile::LMirror) => match dir {
                    Dir::Down => {
                        dir = Dir::Right;
                        ray_x += 1;
                    }
                    Dir::Right => {
                        dir = Dir::Down;
                        ray_y += 1;
                    }
                    Dir::Up => {
                        dir = Dir::Left;
                        ray_x -= 1;
                    }
                    Dir::Left => {
                        dir = Dir::Up;
                        ray_y -= 1;
                    }
                },
                None => break,
            }
        }
    }
    grid.energized()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    VSplitter,
    HSplitter,
    RMirror,
    LMirror,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
}

impl Dir {
    pub fn offset(&self) -> (i32, i32) {
        match self {
            Self::Right => (1, 0),
            Self::Left => (-1, 0),
            Self::Up => (0, -1),
            Self::Down => (0, 1),
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    covered: Box<[(bool, bool, bool, bool)]>,
    energized: Box<[bool]>,
    tiles: Box<[Tile]>,
    height: usize,
    width: usize,
}

impl Grid {
    pub fn from_str(input: &str) -> Self {
        let grid = input
            .lines()
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '.' => Tile::Empty,
                    '|' => Tile::VSplitter,
                    '-' => Tile::HSplitter,
                    '/' => Tile::RMirror,
                    '\\' => Tile::LMirror,
                    c => panic!("{c}"),
                })
            })
            .collect::<Box<_>>();
        Self {
            energized: vec![false; grid.len()].into_boxed_slice(),
            covered: vec![(false, false, false, false); grid.len()].into_boxed_slice(),
            tiles: grid,
            height: input.lines().count(),
            width: input.lines().next().unwrap().len(),
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn energize(&mut self, x: i32, y: i32) {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            self.energized[y as usize * self.width + x as usize] = true;
        }
    }

    pub fn energized(&self) -> u64 {
        self.energized.iter().map(|bool| *bool as u64).sum::<u64>()
    }

    pub fn set_covered(&mut self, x: i32, y: i32, dir: Dir) {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            let x = &mut self.covered[y as usize * self.width + x as usize];
            match dir {
                Dir::Down => x.0 = true,
                Dir::Up => x.1 = true,
                Dir::Left => x.2 = true,
                Dir::Right => x.3 = true,
            };
        }
    }

    pub fn get_covered(&self, x: i32, y: i32, dir: Dir) -> bool {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            let x = &self.covered[y as usize * self.width + x as usize];
            match dir {
                Dir::Down => x.0,
                Dir::Up => x.1,
                Dir::Left => x.2,
                Dir::Right => x.3,
            }
        } else {
            true
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<Tile> {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            Some(self.tiles[y as usize * self.width + x as usize])
        } else {
            None
        }
    }
}
