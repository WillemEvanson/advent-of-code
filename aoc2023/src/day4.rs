use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut part1 = 0;

    let mut copies = vec![1; input.lines().count()];
    for (i, line) in input.lines().enumerate() {
        let (_, game) = line.split_once(':').unwrap();
        let (winning, mine) = game.split_once('|').unwrap();

        let winning = winning
            .split_whitespace()
            .map(|str| str.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let mut count = 0;
        for my in mine
            .split_whitespace()
            .map(|str| str.parse::<u64>().unwrap())
        {
            if winning.contains(&my) {
                count += 1;
            }
        }
        part1 += 2u64.pow(count as u32) / 2;

        for j in i + 1..i + 1 + count {
            copies[j] += copies[i];
        }
    }
    let part2: u64 = copies.iter().sum();

    Solution::from((part1, part2))
}
