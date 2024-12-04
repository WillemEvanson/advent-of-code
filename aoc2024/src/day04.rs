use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut grid = util::grid::Grid::new(
        input.lines().next().unwrap().len() as u32,
        input.lines().count() as u32,
    );

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.set(x as u32, y as u32, c);
        }
    }

    let horizontal = input;
    let mut vertical = String::new();
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            vertical.push(*grid.get_at(x, y).unwrap());
        }
        vertical.push('\n');
    }

    let mut down_right = String::new();
    for i in 0..grid.height() {
        let mut j = 0;
        while i + j < grid.height() && j < grid.width() {
            down_right.push(*grid.get_at(j, i + j).unwrap());
            j += 1;
        }
        down_right.push('\n');
    }
    for i in 1..grid.width() {
        let mut j = 0;
        while i + j < grid.width() && j < grid.height() {
            down_right.push(*grid.get_at(i + j, j).unwrap());
            j += 1;
        }
        down_right.push('\n');
    }

    let mut down_left = String::new();
    for i in 0..grid.height() {
        let mut j = 0;
        while i.checked_sub(j).is_some() && grid.width().checked_sub(j).is_some() {
            down_left.push(*grid.get_at(j, i - j).unwrap());
            j += 1;
        }
        down_left.push('\n');
    }
    for i in 1..grid.height() {
        let mut j = 0;
        while i + j < grid.height() && grid.width().checked_sub(j).is_some() {
            down_left.push(*grid.get_at(grid.width() - 1 - j, i + j).unwrap());
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
    for y in 1..grid.height() - 1 {
        for x in 1..grid.width() - 1 {
            if *grid.get_at(x, y).unwrap() == 'A' {
                continue;
            }

            if !((*grid.get_at(x - 1, y - 1).unwrap() == 'M'
                && *grid.get_at(x + 1, y + 1).unwrap() == 'S')
                || (*grid.get_at(x - 1, y - 1).unwrap() == 'S'
                    && *grid.get_at(x + 1, y + 1).unwrap() == 'M'))
            {
                continue;
            }

            if !((*grid.get_at(x - 1, y + 1).unwrap() == 'M'
                && *grid.get_at(x + 1, y - 1).unwrap() == 'S')
                || (*grid.get_at(x - 1, y + 1).unwrap() == 'S'
                    && *grid.get_at(x + 1, y - 1).unwrap() == 'M'))
            {
                continue;
            }
            part2 += 1;
        }
    }

    Solution::from((part1, part2))
}
