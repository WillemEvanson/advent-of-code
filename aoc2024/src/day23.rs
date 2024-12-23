use std::collections::{HashMap, HashSet};

use util::bit_set::BitSet;
use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut map = HashMap::new();
    let mut names = Vec::new();
    let mut graph = Vec::new();

    for line in input.lines() {
        let (node0, node1) = line.split_once('-').unwrap();

        let node0_key = if let Some(i) = map.get(node0) {
            *i
        } else {
            let i = map.len() as u32;
            graph.push(Vec::new());
            names.push(node0);
            map.insert(node0, i);
            i
        };

        let node1_key = if let Some(i) = map.get(node1) {
            *i
        } else {
            let i = map.len() as u32;
            graph.push(Vec::new());
            names.push(node1);
            map.insert(node1, i);
            i
        };

        graph[node0_key as usize].push(node1_key);
        graph[node1_key as usize].push(node0_key);
    }

    let mut found = HashSet::new();
    for (i, neighbors) in graph.iter().enumerate() {
        for j in 0..neighbors.len() {
            for k in 0..neighbors.len() {
                if i == j {
                    continue;
                }

                let key1 = neighbors[j];
                let key2 = neighbors[k];

                if graph[key1 as usize].contains(&key2) {
                    let mut array = [i as u32, key1, key2];
                    array.sort_unstable();

                    found.insert(array);
                }
            }
        }
    }

    let mut part1 = 0;
    for &[k0, k1, k2] in found.iter() {
        if names[k0 as usize].starts_with('t')
            || names[k1 as usize].starts_with('t')
            || names[k2 as usize].starts_with('t')
        {
            part1 += 1;
        }
    }

    let mut p = BitSet::new(graph.len() as u32);
    for i in 0..graph.len() {
        p.set(i as u32);
    }

    let maximal_clique = bron_kerbosch(
        &graph,
        BitSet::new(graph.len() as u32),
        p,
        BitSet::new(graph.len() as u32),
    );

    let mut valid = Vec::new();
    for (i, _) in maximal_clique
        .iter()
        .enumerate()
        .filter(|&(_, within)| within)
    {
        valid.push(names[i]);
    }
    valid.sort_unstable();

    let mut part2 = String::new();
    for n in valid.iter().take(valid.len() - 1) {
        part2.push_str(format!("{n},").as_str());
    }
    part2.pop();

    Solution::from((part1, part2))
}

fn bron_kerbosch(neighbors: &[Vec<u32>], r: BitSet, mut p: BitSet, mut x: BitSet) -> BitSet {
    if p.count() == 0 && x.count() == 0 {
        return r;
    }

    let mut i = 0;
    let mut maximal_clique_size = 0;
    let mut maximal_clique = BitSet::new(neighbors.len() as u32);
    while i < neighbors.len() as u32 {
        let v = i;
        if !p.get(i) {
            i += 1;
            continue;
        }

        let mut new_r = r.clone();
        new_r.set(v);

        let mut new_p = BitSet::new(neighbors.len() as u32);
        let mut new_x = BitSet::new(neighbors.len() as u32);
        for &neighbor in neighbors[v as usize].iter() {
            if p.get(neighbor) {
                new_p.set(neighbor);
            }
            if x.get(neighbor) {
                new_x.set(neighbor);
            }
        }

        let clique = bron_kerbosch(neighbors, new_r, new_p, new_x);
        p.unset(v);
        x.set(v);

        let clique_size = clique.count();
        if maximal_clique_size < clique_size {
            maximal_clique_size = clique_size;
            maximal_clique = clique;
        }

        i += 1;
    }

    maximal_clique
}
