use std::cmp::Ordering;

use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut bids = Vec::new();
    for line in input.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        bids.push((
            hand,
            classify_part1(hand),
            classify_part2(hand),
            bid.parse::<u64>().unwrap(),
        ));
    }

    bids.sort_by(|(a_str, a_kind, _, _), (b_str, b_kind, _, _)| {
        match a_kind.cmp(b_kind).reverse() {
            Ordering::Equal => {
                for (a, b) in a_str.chars().zip(b_str.chars()) {
                    let order = stronger_v0(a, b);
                    if order != Ordering::Equal {
                        return order;
                    }
                }
                Ordering::Equal
            }
            result => result,
        }
    });

    let mut part1 = 0;
    for (i, (_, _, _, bid)) in bids.iter().enumerate() {
        part1 += (i as u64 + 1) * bid;
    }

    bids.sort_by(|(a_str, _, a_kind, _), (b_str, _, b_kind, _)| {
        match a_kind.cmp(b_kind).reverse() {
            Ordering::Equal => {
                for (a, b) in a_str.chars().zip(b_str.chars()) {
                    let order = stronger_v1(a, b);
                    if order != Ordering::Equal {
                        return order;
                    }
                }
                Ordering::Equal
            }
            result => result,
        }
    });

    let mut part2 = 0;
    for (i, (_, _, _, bid)) in bids.iter().enumerate() {
        part2 += (i as u64 + 1) * bid;
    }

    Solution::from((part1, part2))
}

fn stronger_v0(first: char, second: char) -> Ordering {
    const ARRAY: [char; 13] = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];

    let first = ARRAY.iter().position(|c| *c == first).unwrap();
    let second = ARRAY.iter().position(|c| *c == second).unwrap();
    first.cmp(&second).reverse()
}

#[inline(never)]
fn stronger_v1(first: char, second: char) -> Ordering {
    const ARRAY: [char; 13] = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    let first = ARRAY.iter().position(|c| *c == first).unwrap();
    let second = ARRAY.iter().position(|c| *c == second).unwrap();
    first.cmp(&second).reverse()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    Five,
    Four,
    Full,
    Three,
    Two,
    One,
    High,
}

fn classify_part1(input: &str) -> Kind {
    let mut results = [('!', 0); 5];
    'str: for c in input.chars() {
        let mut i = 0;
        while results[i].1 != 0 {
            if results[i].0 == c {
                results[i].1 += 1;
                continue 'str;
            }
            i += 1;
        }
        results[i] = (c, 1);
    }
    let mut counts = [0; 5];
    for (i, (_, count)) in results.into_iter().enumerate() {
        counts[i] = count;
    }

    let mut i = 0;
    while i < 5 && counts[i] != 0 {
        i += 1;
    }
    let slice = if i == 0 {
        return Kind::Five;
    } else {
        &mut counts[..i]
    };
    slice.sort_unstable();

    match &*slice {
        [5] => Kind::Five,
        [1, 4] => Kind::Four,
        [2, 3] => Kind::Full,
        [1, 1, 3] => Kind::Three,
        [1, 2, 2] => Kind::Two,
        [1, 1, 1, 2] => Kind::One,
        _ => Kind::High,
    }
}

fn classify_part2(input: &str) -> Kind {
    let mut j_count = 0;
    let mut results = [('!', 0); 5];
    'str: for c in input.chars() {
        if c == 'J' {
            j_count += 1;
            continue;
        }

        let mut i = 0;
        while results[i].1 != 0 {
            if results[i].0 == c {
                results[i].1 += 1;
                continue 'str;
            }
            i += 1;
        }
        results[i] = (c, 1);
    }
    let mut counts = [0; 5];
    for (i, (_, count)) in results.into_iter().enumerate() {
        counts[i] = count;
    }

    let mut i = 0;
    while i < 5 && counts[i] != 0 {
        i += 1;
    }
    let slice = if i == 0 {
        return Kind::Five;
    } else {
        &mut counts[..i]
    };
    slice.sort_unstable();
    *slice.last_mut().unwrap() += j_count;

    match &*slice {
        [5] => Kind::Five,
        [1, 4] => Kind::Four,
        [2, 3] => Kind::Full,
        [1, 1, 3] => Kind::Three,
        [1, 2, 2] => Kind::Two,
        [1, 1, 1, 2] => Kind::One,
        _ => Kind::High,
    }
}
