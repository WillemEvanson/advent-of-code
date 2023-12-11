use util::bit_set::BitSet;
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

        let mut rows_bits = BitSet::new(height as u32);
        for y in 0..height {
            if tiles[y * width..(y + 1) * width].contains(&Tile::Galaxy) {
                rows_bits.set(y as u32);
            }
        }

        let mut cols_bits = BitSet::new(width as u32);
        'cols: for x in 0..width {
            for y in 0..height {
                if let Tile::Galaxy = tiles[y * width + x] {
                    cols_bits.set(x as u32);
                    continue 'cols;
                }
            }
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

        let occupied_x = self.cols_bits.between(min_x, max_x);
        let empty_x = (max_x - min_x) - occupied_x;

        let occupied_y = self.rows_bits.between(min_y, max_y);
        let empty_y = (max_y - min_y) - occupied_y;

        (occupied_x + occupied_y, empty_x + empty_y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Galaxy,
    Empty,
}
