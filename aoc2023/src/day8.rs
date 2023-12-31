use bimap::BiMap;

fn add<'a>(map: &mut BiMap<&'a str, u32>, node: &'a str) -> u32 {
    if let Some(i) = map.get_by_left(node) {
        *i
    } else {
        let i = map.len() as u32;
        map.insert(node, i);
        i
    }
}

use util::Solution;

pub fn solve(input: &str) -> Solution {
    let (sequence, nodes) = input.split_once("\n\n").unwrap();

    let mut map = BiMap::with_capacity(1024);
    let mut graph = nodes
        .lines()
        .map(|str| {
            let (node, children) = str.split_once(" = ").unwrap();
            let (child0, child1) = children.split_once(',').unwrap();

            let child0 = child0.trim_matches(|c: char| !c.is_ascii_alphanumeric());
            let child1 = child1.trim_matches(|c: char| !c.is_ascii_alphanumeric());

            let node = add(&mut map, node);
            let child0 = add(&mut map, child0);
            let child1 = add(&mut map, child1);

            (node, (child0, child1))
        })
        .collect::<Vec<(u32, (u32, u32))>>();
    graph.sort_by_key(|(i, _)| *i);

    let mut part1 = 0;
    let mut current = *map.get_by_left("AAA").unwrap();
    for char in sequence.chars().cycle() {
        if let Some(&"ZZZ") = map.get_by_right(&current) {
            break;
        }

        let (child0, child1) = graph[current as usize].1;
        if char == 'L' {
            current = child0;
        } else {
            current = child1;
        }
        part1 += 1;
    }

    let mut part2 = 1;
    for mut current in map
        .iter()
        .filter(|(str, _)| &str[2..] == "A")
        .map(|(_, i)| *i)
    {
        let mut steps = 0;
        for char in sequence.chars().cycle() {
            if let Some("Z") = map.get_by_right(&current).map(|str| &str[2..]) {
                break;
            }

            let (child0, child1) = graph[current as usize].1;
            if char == 'L' {
                current = child0;
            } else {
                current = child1;
            }
            steps += 1;
        }
        part2 = lcm(part2, steps);
    }

    Solution::from((part1, part2))
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    if a > b {
        (a / gcd(a, b)) * b
    } else {
        (b / gcd(a, b)) * a
    }
}
