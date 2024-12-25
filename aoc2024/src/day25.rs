use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for schematic in input.split("\n\n") {
        let grid = schematic
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        if grid[0].iter().all(|c| *c == '#') {
            let mut heights = Vec::new();
            for x in 0..grid[0].len() {
                for (y, row) in grid.iter().enumerate().skip(1) {
                    if row[x] != '#' {
                        heights.push(y - 1);
                        break;
                    }
                }
            }
            locks.push(heights);
        } else {
            let mut heights = Vec::new();
            for x in 0..grid[0].len() {
                for offset_y in 0..grid.len() {
                    let y = grid.len() - 1 - offset_y;
                    if grid[y][x] != '#' {
                        heights.push(offset_y - 1);
                        break;
                    }
                }
            }
            keys.push(heights);
        }
    }

    let mut solution = 0;
    for lock in locks.iter() {
        'outer: for key in keys.iter() {
            for i in 0..lock.len() {
                if key[i] + lock[i] >= 6 {
                    continue 'outer;
                }
            }
            solution += 1;
        }
    }

    Solution::from(solution)
}
