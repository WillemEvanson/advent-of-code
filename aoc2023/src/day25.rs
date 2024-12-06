use std::collections::HashMap;

use util::rng::Rng;
use util::union_find::UnionFind;
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

    let mut rng = Rng::new(0);
    let result = loop {
        // Use Karger's algorithm to try to find the minimum cut for the graph. We know
        // that it must be 3, so we terminate upon finding a min_cut of 3.
        let (min_cut, mut find) = contract(&mut rng, edges.clone(), back.len());
        if min_cut == 3 {
            // Karger's algorithm merges nodes together until there are only two nodes. This
            // means that we only need to check for one of two nodes.
            let mut counts = [0; 2];
            let first = find.find(0);
            for i in 0..back.len() {
                if find.find(i as u32) == first {
                    counts[0] += 1;
                } else {
                    counts[1] += 1;
                }
            }

            break counts[0] * counts[1];
        }
    };

    Solution::from(result)
}

/// Karger's algorithm using Kruskal's algorithm and a disjoint-forest for the
/// minimum spanning results in a much faster execution time than the naive
/// implementation.
fn contract(rng: &mut Rng, mut edges: Vec<(u32, u32)>, n: usize) -> (u32, UnionFind) {
    // Fisher-Yates shuffle
    for i in 0..edges.len() - 2 {
        let j = i + rng.gen_bounded_u32(edges.len() as u32 - i as u32) as usize;
        edges.swap(i, j);
    }

    let mut remaining_components = n;
    let mut union_find = UnionFind::new(n as u32);
    for &(u, v) in edges.iter() {
        if union_find.find(u) != union_find.find(v) {
            union_find.merge(u, v);
            remaining_components -= 1;
        }
        if remaining_components == 2 {
            break;
        }
    }

    let mut min_cut = 0;
    for &(u, v) in edges.iter() {
        if union_find.find(u) != union_find.find(v) {
            min_cut += 1;
        }
    }

    (min_cut, union_find)
}
