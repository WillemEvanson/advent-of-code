use util::Solution;

pub fn solve(input: &str) -> Solution {
    // Part One
    let mut x = 0;
    let mut y = 0;
    let mut perimeter = 0;
    let mut points = vec![(0, 0)];
    for line in input.lines() {
        let (dir, rest) = line.split_once(' ').unwrap();
        let (count, _) = rest.split_once(' ').unwrap();
        let count = count.parse::<i64>().unwrap();

        match dir {
            "R" => x += count,
            "L" => x -= count,
            "D" => y += count,
            "U" => y -= count,
            _ => panic!(),
        };
        perimeter += count;
        points.push((x, y));
    }

    let mut area = 0;
    for &[(x0, y0), (x1, y1)] in points.array_windows::<2>() {
        area += (x0 * y1) - (x1 * y0);
    }
    area = area.abs() / 2;
    perimeter /= 2;

    let part1 = (area + perimeter + 1) as u64;

    // Part Two
    let mut x = 0;
    let mut y = 0;
    let mut perimeter = 0;
    let mut points = vec![(0, 0)];
    for line in input.lines() {
        let parameters = &line.split_whitespace().last().unwrap()[2..8];
        let mut count = 0;
        for c in parameters[..5].chars() {
            count = count * 16 + c.to_digit(16).unwrap() as i64;
        }

        match &parameters[5..] {
            "0" => x += count,
            "2" => x -= count,
            "1" => y += count,
            "3" => y -= count,
            _ => panic!(),
        };

        perimeter += count;
        points.push((x, y));
    }
    perimeter /= 2;

    let mut area = 0;
    for &[(x0, y0), (x1, y1)] in points.array_windows::<2>() {
        area += (x0 * y1) - (x1 * y0);
    }
    area = area.abs() / 2;

    let part2 = (area + perimeter + 1) as u64;

    Solution::from((part1, part2))
}
