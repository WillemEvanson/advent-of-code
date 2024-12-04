use util::Solution;

pub fn solve(input: &str) -> Solution {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let horizontal = input;
    let mut vertical = String::new();
    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            vertical.push(grid[y][x]);
        }
        vertical.push('\n');
    }

    let mut down_right = String::new();
    for i in 0..grid.len() {
        let mut j = 0;
        while i + j < grid.len() && j < grid[0].len() {
            down_right.push(grid[i + j][j]);
            j += 1;
        }
        down_right.push('\n');
    }
    for i in 1..grid[0].len() {
        let mut j = 0;
        while i + j < grid[0].len() && j < grid.len() {
            down_right.push(grid[j][i + j]);
            j += 1;
        }
        down_right.push('\n');
    }

    let mut down_left = String::new();
    for i in 0..grid.len() as i32 {
        let mut j = 0;
        while i - j >= 0 && grid[0].len() as i32 - j >= 0 {
            down_left.push(grid[(i - j) as usize][j as usize]);
            j += 1;
        }
        down_left.push('\n');
    }
    for i in 1..grid.len() as i32 {
        let mut j = 0;
        while i + j < grid.len() as i32 && grid[0].len() as i32 - j >= 0 {
            down_left.push(grid[(i + j) as usize][grid[0].len() - 1 - j as usize]);
            j += 1;
        }
        down_left.push('\n');
    }

    let mut part1 = 0;
    part1 += horizontal.match_indices("XMAS").count() as u64;
    part1 += horizontal.match_indices("SAMX").count() as u64;
    part1 += vertical.match_indices("XMAS").count() as u64;
    part1 += vertical.match_indices("SAMX").count() as u64;

    part1 += down_right.match_indices("XMAS").count() as u64;
    part1 += down_right.match_indices("SAMX").count() as u64;
    part1 += down_left.match_indices("XMAS").count() as u64;
    part1 += down_left.match_indices("SAMX").count() as u64;

    let mut part2 = 0;
    for y in 1..grid.len() - 1 {
        for x in 1..grid[y].len() - 1 {
            if grid[x][y] != 'A' {
                continue;
            }

            if !(grid[x - 1][y - 1] == 'M' && grid[x + 1][y + 1] == 'S')
                && !(grid[x - 1][y - 1] == 'S' && grid[x + 1][y + 1] == 'M')
            {
                continue;
            }

            if !(grid[x - 1][y + 1] == 'M' && grid[x + 1][y - 1] == 'S')
                && !(grid[x - 1][y + 1] == 'S' && grid[x + 1][y - 1] == 'M')
            {
                continue;
            }
            part2 += 1;
        }
    }

    Solution::from((part1, part2))
}
