use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut part1 = 0;
    for part in input.split(',') {
        part1 += hash(part);
    }

    let mut boxes: Vec<Vec<(&str, &str)>> = vec![Vec::new(); 256];
    for part in input.split(',') {
        if let Some((label, length)) = part.split_once('=') {
            let hash = hash(label);

            if let Some(i) = boxes[hash as usize].iter().position(|(l, _)| *l == label) {
                boxes[hash as usize][i] = (label, length);
            } else {
                boxes[hash as usize].push((label, length));
            }
        } else {
            // Remove
            let label = &part[..part.len() - 1];
            let hash = hash(label);

            boxes[hash as usize].retain(|(l, _)| *l != label);
        }
    }

    let mut part2 = 0;
    for (i, slot) in boxes.iter().enumerate() {
        for (j, (_, length)) in slot.iter().enumerate() {
            part2 += ((1 + i) * (1 + j) * length.parse::<usize>().unwrap()) as u64;
        }
    }

    Solution::from((part1, part2))
}

fn hash(str: &str) -> u64 {
    let mut current: u64 = 0;
    for c in str.chars() {
        current = current.wrapping_add(c as u64);
        current = current.wrapping_mul(17);
        current %= 256;
    }
    current
}
