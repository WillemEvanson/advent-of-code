use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut part1 = 0;
    let mut map = [[[[0; 19]; 19]; 19]; 19];
    for secret in input.lines() {
        let mut secret = secret.parse::<u64>().unwrap();

        let mut prices = Vec::with_capacity(2000);
        for _ in 0..2000 {
            prices.push(secret % 10);
            secret = evolve(secret);
        }
        part1 += secret;

        let mut visited = [[[[false; 19]; 19]; 19]; 19];
        for &[x0, x1, x2, x3, x4] in prices.array_windows() {
            let offset0 = x1 as i64 - x0 as i64;
            let offset1 = x2 as i64 - x1 as i64;
            let offset2 = x3 as i64 - x2 as i64;
            let offset3 = x4 as i64 - x3 as i64;

            if !visited[(offset0 + 9) as usize][(offset1 + 9) as usize][(offset2 + 9) as usize]
                [(offset3 + 9) as usize]
            {
                visited[(offset0 + 9) as usize][(offset1 + 9) as usize][(offset2 + 9) as usize]
                    [(offset3 + 9) as usize] = true;
                map[(offset0 + 9) as usize][(offset1 + 9) as usize][(offset2 + 9) as usize]
                    [(offset3 + 9) as usize] += x4;
            }
        }
    }
    let part2 = *map
        .as_flattened()
        .as_flattened()
        .as_flattened()
        .iter()
        .max()
        .unwrap();

    Solution::from((part1, part2))
}

fn evolve(mut secret_number: u64) -> u64 {
    let result = secret_number * 64;
    secret_number = mix(secret_number, result);
    secret_number = prune(secret_number);

    let result = secret_number / 32;
    secret_number = mix(secret_number, result);
    secret_number = prune(secret_number);

    let result = secret_number * 2048;
    secret_number = mix(secret_number, result);
    secret_number = prune(secret_number);

    secret_number
}

fn mix(secret: u64, other: u64) -> u64 {
    secret ^ other
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}
