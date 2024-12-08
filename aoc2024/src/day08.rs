use std::collections::HashMap;

use util::bit_set::BitSet;
use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut map = HashMap::new();
    let mut frequencies: Vec<Vec<_>> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                let i = if let Some(i) = map.get(&c) {
                    *i
                } else {
                    let i = map.len();
                    frequencies.push(Vec::new());
                    map.insert(c, i as u32);
                    i as u32
                };
                frequencies[i as usize].push((x as i32, y as i32));
            }
        }
    }
    let width = input.lines().next().unwrap().chars().count() as i32;
    let height = input.lines().count() as i32;

    let mut antinodes = BitSet::new(width as u32 * height as u32);
    for emitters in frequencies.iter() {
        for i in 0..emitters.len() {
            for j in i + 1..emitters.len() {
                let (i_x, i_y) = emitters[i];
                let (j_x, j_y) = emitters[j];

                let diff_x = i_x - j_x;
                let diff_y = i_y - j_y;

                let v0_x = i_x + diff_x;
                let v0_y = i_y + diff_y;

                let v1_x = j_x - diff_x;
                let v1_y = j_y - diff_y;

                if 0 <= v0_x && v0_x < width && 0 <= v0_y && v0_y < height {
                    antinodes.set((v0_y * width + v0_x) as u32);
                }

                if 0 <= v1_x && v1_x < width && 0 <= v1_y && v1_y < height {
                    antinodes.set((v1_y * width + v1_x) as u32);
                }
            }
        }
    }
    let part1 = antinodes.count() as u64;

    antinodes.clear();
    for emitters in frequencies.iter() {
        for i in 0..emitters.len() {
            for j in i + 1..emitters.len() {
                let (i_x, i_y) = emitters[i];
                let (j_x, j_y) = emitters[j];

                let diff_x = i_x - j_x;
                let diff_y = i_y - j_y;

                let mut current_x = i_x;
                let mut current_y = i_y;
                while 0 <= current_x && current_x < width && 0 <= current_y && current_y < height {
                    antinodes.set((current_y * width + current_x) as u32);
                    current_x += diff_x;
                    current_y += diff_y;
                }

                let mut current_x = j_x;
                let mut current_y = j_y;
                while 0 <= current_x && current_x < width && 0 <= current_y && current_y < height {
                    antinodes.set((current_y * width + current_x) as u32);
                    current_x -= diff_x;
                    current_y -= diff_y;
                }
            }
        }
    }
    let part2 = antinodes.count() as u64;

    Solution::from((part1, part2))
}
