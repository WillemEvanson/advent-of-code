use std::collections::HashMap;

use rand::Rng;
use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut edges = Vec::new();
    let mut mapped = HashMap::new();
    let mut back = Vec::new();
    for line in input.lines() {
        let (name, connected) = line.split_once(": ").unwrap();

        let name = match mapped.get(&name) {
            Some(&id) => id,
            None => {
                let id = mapped.len() as u32;
                mapped.insert(name, id);
                back.push(name);
                id
            }
        };

        for edge in connected.split_ascii_whitespace() {
            let edge = match mapped.get(&edge) {
                Some(&id) => id,
                None => {
                    let id = mapped.len() as u32;
                    mapped.insert(edge, id);
                    back.push(edge);
                    id
                }
            };
            edges.push((name, edge));
        }
    }

    let result = loop {
        let (x, merged) = contract(edges.clone(), back.len());
        if x.len() == 3 {
            let mut counts = vec![1; back.len()];
            for (to, from) in merged {
                counts[to as usize] += counts[from as usize];
                counts[from as usize] = 0;
            }
            let mut first = 0;
            let mut second = 0;
            for i in counts {
                if i != 0 {
                    if first == 0 {
                        first = i;
                    } else {
                        second = i;
                        break;
                    }
                }
            }
            break first * second;
        }
    };
    Solution::from(result)
}

fn contract(mut edges: Vec<(u32, u32)>, mut n: usize) -> (Vec<(u32, u32)>, Vec<(u32, u32)>) {
    let mut merged = Vec::new();

    let mut rand = rand::thread_rng();
    while n > 2 {
        let idx = rand.gen_range(0..edges.len());
        let (v0, v1) = edges.remove(idx);
        merged.push((v0, v1));

        for (p0, p1) in edges.iter_mut() {
            if v0 == *p0 {
                *p0 = v0;
            } else if v0 == *p1 {
                *p1 = v0;
            }

            if v1 == *p0 {
                *p0 = v0;
            } else if v1 == *p1 {
                *p1 = v0;
            }
        }

        let mut j = 0;
        while j < edges.len() {
            if edges[j].0 == edges[j].1 {
                edges.swap_remove(j);
            } else {
                j += 1;
            }
        }

        n -= 1;
    }
    (edges, merged)
}
