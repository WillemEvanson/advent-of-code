use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut part1 = 0;
    'puzzle: for puzzle in input.split("\n\n") {
        let grid = puzzle
            .lines()
            .map(|str| str.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // Horizontal
        'line: for i in 1..grid.len() {
            let mut current_lower = i - 1;
            let mut current_upper = i;
            loop {
                for i in 0..grid[current_lower].len() {
                    if grid[current_lower][i] != grid[current_upper][i] {
                        continue 'line;
                    }
                }

                if current_lower > 0 && current_upper < grid.len() - 1 {
                    current_lower -= 1;
                    current_upper += 1;
                } else {
                    break;
                }
            }
            part1 += 100 * i as u64;
            continue 'puzzle;
        }

        // Vertical
        'line: for i in 1..grid[0].len() {
            let mut current_lower = i - 1;
            let mut current_upper = i;

            loop {
                for tile in grid.iter() {
                    if tile[current_lower] != tile[current_upper] {
                        continue 'line;
                    }
                }

                if current_lower > 0 && current_upper < grid[0].len() - 1 {
                    current_lower -= 1;
                    current_upper += 1;
                } else {
                    break;
                }
            }
            part1 += i as u64;
            continue 'puzzle;
        }
    }

    let mut part2 = 0;
    'puzzle: for puzzle in input.split("\n\n") {
        let grid = puzzle
            .lines()
            .map(|str| str.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // Horizontal
        'line: for i in 1..grid.len() {
            let mut current_lower = i - 1;
            let mut current_upper = i;

            let mut corrected = false;
            loop {
                for i in 0..grid[current_lower].len() {
                    if grid[current_lower][i] != grid[current_upper][i] {
                        if !corrected {
                            corrected = true;
                        } else {
                            continue 'line;
                        }
                    }
                }

                if current_lower > 0 && current_upper < grid.len() - 1 {
                    current_lower -= 1;
                    current_upper += 1;
                } else {
                    break;
                }
            }

            if corrected {
                part2 += 100 * i as u64;
                continue 'puzzle;
            }
        }

        // Vertical
        'line: for i in 1..grid[0].len() {
            let mut current_lower = i - 1;
            let mut current_upper = i;

            let mut corrected = false;
            loop {
                for tile in grid.iter() {
                    if tile[current_lower] != tile[current_upper] {
                        if !corrected {
                            corrected = true;
                        } else {
                            continue 'line;
                        }
                    }
                }

                if current_lower > 0 && current_upper < grid[0].len() - 1 {
                    current_lower -= 1;
                    current_upper += 1;
                } else {
                    break;
                }
            }
            if corrected {
                part2 += i as u64;
                continue 'puzzle;
            }
        }
    }

    Solution::from((part1, part2))
}
