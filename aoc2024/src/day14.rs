use util::Solution;

pub fn solve(input: &str) -> Solution {
    let width = 101;
    let height = 103;

    let middle_x = width / 2;
    let middle_y = height / 2;

    let mut quadrant_count = [0; 4];
    let mut robots = Vec::new();
    for line in input.lines() {
        let (position, velocity) = line.split_once(' ').unwrap();
        let (p_x, p_y) = position
            .trim_matches(|c: char| !c.is_ascii_digit() && c != '-')
            .split_once(',')
            .unwrap();
        let (v_x, v_y) = velocity
            .trim_matches(|c: char| !c.is_ascii_digit() && c != '-')
            .split_once(',')
            .unwrap();

        let p_x = p_x.parse::<i32>().unwrap();
        let p_y = p_y.parse::<i32>().unwrap();
        let v_x = v_x.parse::<i32>().unwrap();
        let v_y = v_y.parse::<i32>().unwrap();
        robots.push((p_x, p_y, v_x, v_y));

        let mut c_x = p_x;
        let mut c_y = p_y;
        for _ in 0..100 {
            c_x = (c_x + v_x).rem_euclid(width);
            c_y = (c_y + v_y).rem_euclid(height);
        }

        if c_x < middle_x && c_y < middle_y {
            quadrant_count[0] += 1;
        } else if c_x > middle_x && c_y < middle_y {
            quadrant_count[1] += 1;
        } else if c_x < middle_x && c_y > middle_y {
            quadrant_count[2] += 1;
        } else if c_x > middle_x && c_y > middle_y {
            quadrant_count[3] += 1;
        }
    }
    let part1 = quadrant_count.iter().product::<u64>();

    let mut i = 0;
    let mut lowest_variance = f32::MAX;
    let mut current_candidate = 0;
    while i < 10_000 {
        i += 1;

        let mut sum_x = 0;
        let mut sum_y = 0;
        for (p_x, p_y, v_x, v_y) in robots.iter_mut() {
            *p_x = (*p_x + *v_x).rem_euclid(width);
            *p_y = (*p_y + *v_y).rem_euclid(height);

            sum_x += *p_x;
            sum_y += *p_y;
        }

        let mean_x = sum_x as f32 / robots.len() as f32;
        let mean_y = sum_y as f32 / robots.len() as f32;

        let variance = robots
            .iter()
            .map(|&(x, y, _, _)| (x as f32, y as f32))
            .fold(0.0, |variance, (x, y)| {
                variance + ((x - mean_x).abs() + (y - mean_y).abs())
            });

        if lowest_variance > variance {
            current_candidate = i;
            lowest_variance = variance;
        }
    }
    let part2 = current_candidate;

    Solution::from((part1, part2))
}
