use std::collections::HashMap;

use util::Solution;

pub fn solve(input: &str) -> Solution {
    let (ordering_rules, updates) = input.split_once("\n\n").unwrap();

    let mut graph = Vec::new();
    let mut map = HashMap::new();
    for line in ordering_rules.lines() {
        let (before, after) = line.split_once('|').unwrap();
        let before = before.parse::<u32>().unwrap();
        let after = after.parse::<u32>().unwrap();

        let before_id = add(&mut graph, &mut map, before);
        let after_id = add(&mut graph, &mut map, after);
        graph[before_id as usize].1.push(after_id);
        graph[after_id as usize].2.push(before_id);
    }

    let mut part1 = 0;
    let mut part2 = 0;
    for update in updates.lines() {
        let update = update
            .split(',')
            .map(|i| i.parse::<u32>().unwrap())
            .map(|i| *map.get(&i).unwrap())
            .collect::<Vec<_>>();

        let mut correct = true;
        'check: for i in 0..update.len() {
            let edges = &graph[update[i] as usize].2;
            for node in update.iter().skip(i + 1) {
                if edges.contains(node) {
                    correct = false;
                    break 'check;
                }
            }
        }

        if correct {
            let len = update.len();
            part1 += graph[update[(len - 1) / 2] as usize].0 as u64;
        } else {
            let mut update = update;
            'outer: loop {
                let mut swapped = false;
                for i in 0..update.len() {
                    let edges = &graph[update[i] as usize].2;
                    for j in i + 1..update.len() {
                        if edges.contains(&update[j]) {
                            update.swap(i, j);
                            swapped = true;
                        }
                    }
                }
                if !swapped {
                    break 'outer;
                }
            }

            let len = update.len();
            part2 += graph[update[(len - 1) / 2] as usize].0 as u64;
        }
    }

    Solution::from((part1, part2))
}

fn add(graph: &mut Vec<(u32, Vec<u32>, Vec<u32>)>, map: &mut HashMap<u32, u32>, node: u32) -> u32 {
    if let Some(i) = map.get(&node) {
        *i
    } else {
        let i = map.len() as u32;
        graph.push((node, Vec::new(), Vec::new()));
        map.insert(node, i);
        i
    }
}
