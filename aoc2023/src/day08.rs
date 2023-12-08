use std::collections::HashMap;

use util::math::lcm;
use util::Solution;

fn add_<'a>(graph: &mut Vec<(u32, u32)>, map: &mut HashMap<&'a str, u32>, node: &'a str) -> u32 {
    if let Some(i) = map.get(node) {
        *i
    } else {
        let i = map.len() as u32;
        map.insert(node, i);
        graph.push((0, 0));
        i
    }
}

pub fn solve(input: &str) -> Solution {
    let (sequence, nodes) = input.split_once("\n\n").unwrap();

    let mut map = HashMap::new();
    let mut graph = Vec::new();
    for line in nodes.lines() {
        let (node, children) = line.split_once(" = ").unwrap();
        let (child0, child1) = children.split_once(',').unwrap();

        let child0 = child0.trim_matches(|c: char| !c.is_ascii_alphanumeric());
        let child1 = child1.trim_matches(|c: char| !c.is_ascii_alphanumeric());

        let node = add_(&mut graph, &mut map, node);
        let child0 = add_(&mut graph, &mut map, child0);
        let child1 = add_(&mut graph, &mut map, child1);

        graph[node as usize] = (child0, child1);
    }

    let mut part1 = 0;
    let end = *map.get("ZZZ").unwrap();
    let mut current = *map.get("AAA").unwrap();
    for char in sequence.chars().cycle() {
        if current == end {
            break;
        }

        let (child0, child1) = graph[current as usize];
        if char == 'L' {
            current = child0;
        } else {
            current = child1;
        }
        part1 += 1;
    }

    let mut part2 = 1;
    let ends = map
        .iter()
        .filter(|(str, _)| &str[2..] == "Z")
        .map(|(_, i)| *i)
        .collect::<Vec<_>>();

    for mut current in map
        .iter()
        .filter(|(str, _)| &str[2..] == "A")
        .map(|(_, i)| *i)
    {
        let mut steps = 0;
        for char in sequence.chars().cycle() {
            if ends.contains(&current) {
                break;
            }

            let (child0, child1) = graph[current as usize];
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
