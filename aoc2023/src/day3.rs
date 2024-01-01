use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut grid = Grid::new(input.lines().next().unwrap().len(), input.lines().count());
    for (y, line) in input.lines().enumerate() {
        let mut iter = line.chars().enumerate();
        while let Some((x, block)) = iter.next() {
            match block {
                '.' => {
                    grid.insert(x as i32, y as i32, GridSpace::None);
                }
                c if c.is_ascii_digit() => {
                    let digit = c.to_digit(10).unwrap() as u64;

                    let mut end = x;
                    let mut number = digit;
                    let mut cloned = iter.clone();
                    while let Some((cur, digit @ ('0'..='9'))) = cloned.next() {
                        number = number * 10 + digit.to_digit(10).unwrap() as u64;
                        iter.next();
                        end = cur;
                    }

                    for i in x..=end {
                        grid.insert(
                            i as i32,
                            y as i32,
                            GridSpace::Num {
                                val: number,
                                start: x as i32,
                                end: end as i32,
                            },
                        );
                    }
                }
                '*' => {
                    grid.insert(x as i32, y as i32, GridSpace::Gear);
                }
                _ => {
                    grid.insert(x as i32, y as i32, GridSpace::Symbol);
                }
            };
        }
    }

    let mut part1 = 0;
    let mut part2 = 0;
    for y in 0..input.lines().count() as i32 {
        let mut x = 0;
        while let Some(block) = grid.get(x, y) {
            match block {
                GridSpace::None | GridSpace::Symbol => (),
                GridSpace::Gear => {
                    let mut numbers = Vec::new();
                    for x in x - 1..=x + 1 {
                        if let Some(&GridSpace::Num { val, .. }) = grid.get(x, y - 1) {
                            numbers.push(val);
                        }
                    }
                    if let Some(&GridSpace::Num { val, .. }) = grid.get(x - 1, y) {
                        numbers.push(val);
                    }
                    if let Some(&GridSpace::Num { val, .. }) = grid.get(x + 1, y) {
                        numbers.push(val);
                    }
                    for x in x - 1..=x + 1 {
                        if let Some(&GridSpace::Num { val, .. }) = grid.get(x, y + 1) {
                            numbers.push(val);
                        }
                    }
                    numbers.sort();
                    numbers.dedup();

                    if numbers.len() == 2 {
                        part2 += numbers[0] * numbers[1];
                    }
                }
                &GridSpace::Num { val, start, end } => {
                    let mut symbol = false;
                    for x in start - 1..=end + 1 {
                        if let Some(GridSpace::Symbol | GridSpace::Gear) = grid.get(x, y - 1) {
                            symbol = true;
                            break;
                        }
                    }
                    if let Some(GridSpace::Symbol | GridSpace::Gear) = grid.get(start - 1, y) {
                        symbol = true;
                    }
                    if let Some(GridSpace::Symbol | GridSpace::Gear) = grid.get(end + 1, y) {
                        symbol = true;
                    }
                    for x in start - 1..=end + 1 {
                        if let Some(GridSpace::Symbol | GridSpace::Gear) = grid.get(x, y + 1) {
                            symbol = true;
                            break;
                        }
                    }
                    if symbol {
                        part1 += val;
                    }

                    x = end;
                }
            }
            x += 1;
        }
    }

    Solution::from((part1, part2))
}

#[derive(Debug, Clone)]
pub struct Grid {
    tiles: Box<[GridSpace]>,
    height: usize,
    width: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            tiles: vec![GridSpace::None; width * height].into_boxed_slice(),
            width,
            height,
        }
    }

    pub fn insert(&mut self, x: i32, y: i32, val: GridSpace) {
        if (0 <= x && x < self.width as i32) && (0 <= y && y < self.height as i32) {
            self.tiles[y as usize * self.width + x as usize] = val;
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<&GridSpace> {
        if (0 <= x && x < self.width as i32) && (0 <= y && y < self.height as i32) {
            Some(&self.tiles[y as usize * self.width + x as usize])
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridSpace {
    Num { val: u64, start: i32, end: i32 },
    Symbol,
    Gear,
    None,
}
