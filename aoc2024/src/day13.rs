use util::Solution;

const OFFSET: u64 = 10000000000000;

pub fn solve(input: &str) -> Solution {
    let mut part1 = 0;
    let mut part2 = 0;
    for machine in input.split("\n\n") {
        let (button_a, rest) = machine.split_once('\n').unwrap();
        let (button_b, prize) = rest.split_once('\n').unwrap();

        let (ax, ay) = button_a.split_once(',').unwrap();
        let ax = ax
            .trim_matches(|c: char| !c.is_ascii_digit())
            .parse::<u64>()
            .unwrap();
        let ay = ay
            .trim_matches(|c: char| !c.is_ascii_digit())
            .parse::<u64>()
            .unwrap();

        let (bx, by) = button_b.split_once(',').unwrap();
        let bx = bx
            .trim_matches(|c: char| !c.is_ascii_digit())
            .parse::<u64>()
            .unwrap();
        let by = by
            .trim_matches(|c: char| !c.is_ascii_digit())
            .parse::<u64>()
            .unwrap();

        let (px, py) = prize.split_once(',').unwrap();
        let px = px
            .trim_matches(|c: char| !c.is_ascii_digit())
            .parse::<u64>()
            .unwrap();
        let py = py
            .trim_matches(|c: char| !c.is_ascii_digit())
            .parse::<u64>()
            .unwrap();

        if let Some((a, b)) = solve_equation(ax, ay, bx, by, px, py) {
            part1 += 3 * a + b;
        }
        if let Some((a, b)) = solve_equation(ax, ay, bx, by, px + OFFSET, py + OFFSET) {
            part2 += 3 * a + b;
        }
    }

    Solution::from((part1, part2))
}

/// Solves a system of two linear equations with the form:
///
/// - `a * x_1 + b * x_2 = p_x`
/// - `a * y_1 + b * y_2 = p_y`
///
/// The function computes the coefficients `a` and `b` such that the equations
/// are satisfied.
fn solve_equation(ax: u64, ay: u64, bx: u64, by: u64, px: u64, py: u64) -> Option<(u64, u64)> {
    let numerator = (py * ax).abs_diff(px * ay);
    let denominator = (by * ax).abs_diff(bx * ay);
    if numerator % denominator != 0 {
        return None;
    }

    let b = numerator / denominator;
    let a = (px - b * bx) / ax;

    let result_x = a * ax + b * bx;
    let result_y = a * ay + b * by;

    if result_x == px && result_y == py {
        Some((a, b))
    } else {
        None
    }
}
