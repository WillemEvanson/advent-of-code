use util::Solution;

pub fn solve(input: &str) -> Solution {
    let (grid, pairs) = Grid::from_str(input);

    let mut part1 = 0;
    let mut part2 = 0;
    for i in 0..pairs.len() {
        for j in i + 1..pairs.len() {
            let a = pairs[i];
            let b = pairs[j];
            let (occupied, empty) = grid.occupied(a.0, a.1, b.0, b.1);

            part1 += occupied as u64 + empty as u64 * 2;
            part2 += occupied as u64 + empty as u64 * 1_000_000;
        }
    }

    Solution::from((part1, part2))
}

#[derive(Debug)]
struct Grid {
    rows_bits: BitSet,
    cols_bits: BitSet,
}

impl Grid {
    fn from_str(input: &str) -> (Self, Vec<(u32, u32)>) {
        let mut width = 0;
        let mut height = 0;
        let mut tiles = Vec::new();
        let mut pairs = Vec::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    pairs.push((x as u32, y as u32));
                    tiles.push(Tile::Galaxy);
                } else {
                    tiles.push(Tile::Empty);
                }
                width = x + 1;
            }
            height = y + 1;
        }

        let mut rows_bits = BitSet::new();
        for y in 0..height {
            rows_bits.push(tiles[y * width..(y + 1) * width].contains(&Tile::Galaxy));
        }

        let mut cols_bits = BitSet::new();
        'cols: for x in 0..width {
            for y in 0..height {
                if let Tile::Galaxy = tiles[y * width + x] {
                    cols_bits.push(true);
                    continue 'cols;
                }
            }
            cols_bits.push(false);
        }

        (
            Self {
                cols_bits,
                rows_bits,
            },
            pairs,
        )
    }

    fn occupied(&self, a_x: u32, a_y: u32, b_x: u32, b_y: u32) -> (u32, u32) {
        let min_x = u32::min(a_x, b_x);
        let max_x = u32::max(a_x, b_x);

        let min_y = u32::min(a_y, b_y);
        let max_y = u32::max(a_y, b_y);

        let occupied_x = self.cols_bits.between(min_x as usize, max_x as usize);
        let empty_x = (max_x - min_x) - occupied_x;

        let occupied_y = self.rows_bits.between(min_y as usize, max_y as usize);
        let empty_y = (max_y - min_y) - occupied_y;

        (occupied_x + occupied_y, empty_x + empty_y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Galaxy,
    Empty,
}

#[derive(Debug)]
struct BitSet {
    bits: Vec<u32>,
    valid: usize,
}

impl BitSet {
    pub fn new() -> Self {
        Self {
            bits: Vec::new(),
            valid: 0,
        }
    }

    pub fn push(&mut self, value: bool) {
        if self.valid % 32 == 0 {
            self.bits.push(0);
        }

        let word = self.bits.last_mut().unwrap();
        *word |= (value as u32) << (self.valid % 32);
        self.valid += 1;
    }

    #[allow(dead_code)]
    pub fn get(&self, idx: usize) -> bool {
        if idx < self.valid {
            let div = idx / 32;
            let rem = idx % 32;
            let word = self.bits[div];

            (word & (1 << rem)) != 0
        } else {
            false
        }
    }

    pub fn between(&self, start: usize, end: usize) -> u32 {
        let div_start = start / 32;
        let rem_start = start % 32;
        let div_end = end / 32;
        let rem_end = end % 32;

        if div_start == div_end {
            let mask = !((1 << rem_start) - 1) & ((1 << rem_end) - 1);
            let word = self.bits[div_start];
            let bits = word & mask;

            bits.count_ones()
        } else {
            let start_mask = (u32::MAX >> rem_start) << rem_start;
            let start_word = self.bits[div_start];
            let start_bits = start_word & start_mask;

            let mut bits = start_bits.count_ones();

            for i in div_start + 1..div_end {
                bits += self.bits[i].count_ones();
            }

            let end_mask = (1 << rem_end) - 1;
            let end_word = self.bits[div_end];
            let end_bits = end_word & end_mask;

            bits += end_bits.count_ones();

            bits
        }
    }
}
