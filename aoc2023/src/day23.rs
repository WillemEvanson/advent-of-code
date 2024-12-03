use std::collections::{HashMap, HashSet};

use util::bit_set::BitSet;
use util::direction::Direction;
use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut grid = util::grid::Grid::new(
        input.lines().next().unwrap().len() as u32,
        input.lines().count() as u32,
    );

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '#' => Tile::Forest,
                '.' => Tile::Path,
                '<' => Tile::Slope(Direction::Left),
                '>' => Tile::Slope(Direction::Right),
                '^' => Tile::Slope(Direction::Up),
                'v' => Tile::Slope(Direction::Down),
                _ => panic!(),
            };
            grid.set(x as u32, y as u32, tile);
        }
    }

    let mut start_x = 0;
    let start_y = 0;
    for i in 0..grid.width() {
        if let Some(Tile::Path) = grid.get_at(i, start_y) {
            start_x = i;
            break;
        }
    }

    let mut end_x = 0;
    let end_y = grid.height() - 1;
    for i in 0..grid.width() {
        if let Some(Tile::Path) = grid.get_at(i, end_y) {
            end_x = i;
            break;
        }
    }

    let mut visited = HashSet::new();
    let mut paths: HashMap<_, HashMap<_, _>> = HashMap::new();
    let mut todo = vec![(start_x, start_y, Direction::Down)];
    while let Some((mut x, mut y, mut direction)) = todo.pop() {
        if visited.contains(&(x, y, direction)) {
            continue;
        }
        visited.insert((x, y, direction));

        let start_x = x;
        let start_y = y;

        let (offset_x, offset_y) = direction.offset();
        (x, y) = grid.get_offset(x, y, offset_x, offset_y).unwrap();

        let mut slope_forward = false;
        let mut slope_backward = false;
        let mut path_length = 0;
        loop {
            if let Some(Tile::Slope(slope_direction)) = grid.get_at(x, y) {
                if *slope_direction == direction {
                    slope_forward = true;
                } else {
                    slope_backward = true;
                }
            }

            let mut straight = false;
            let (offset_x, offset_y) = direction.offset();
            if let Some(Tile::Path | Tile::Slope(_)) = grid.get_at_offset(x, y, offset_x, offset_y)
            {
                straight = true;
            }

            let mut left = false;
            let (offset_x, offset_y) = direction.left().offset();
            if let Some(Tile::Path | Tile::Slope(_)) = grid.get_at_offset(x, y, offset_x, offset_y)
            {
                left = true;
            }

            let mut right = false;
            let (offset_x, offset_y) = direction.right().offset();
            if let Some(Tile::Path | Tile::Slope(_)) = grid.get_at_offset(x, y, offset_x, offset_y)
            {
                right = true;
            }

            let count = straight as u8 + left as u8 + right as u8;
            match count.cmp(&1) {
                std::cmp::Ordering::Equal => {
                    visited.insert((x, y, direction));
                    path_length += 1;
                    if straight {
                        let (offset_x, offset_y) = direction.offset();
                        (x, y) = grid.get_offset(x, y, offset_x, offset_y).unwrap();
                    }
                    if left {
                        direction = direction.left();
                        let (offset_x, offset_y) = direction.offset();
                        (x, y) = grid.get_offset(x, y, offset_x, offset_y).unwrap();
                    }
                    if right {
                        direction = direction.right();
                        let (offset_x, offset_y) = direction.offset();
                        (x, y) = grid.get_offset(x, y, offset_x, offset_y).unwrap();
                    }
                }
                std::cmp::Ordering::Greater => {
                    if straight {
                        todo.push((x, y, direction))
                    }
                    if left {
                        todo.push((x, y, direction.left()))
                    }
                    if right {
                        todo.push((x, y, direction.right()))
                    }
                    break;
                }
                std::cmp::Ordering::Less => break,
            }
        }

        if path_length != 0 {
            let start = (start_x, start_y);
            let end = (x, y);

            paths
                .entry(start)
                .or_default()
                .insert(end, (path_length + 1, slope_forward));
            paths
                .entry(end)
                .or_default()
                .insert(start, (path_length + 1, slope_backward));
        }
    }

    // Transform the graph into a vector of vectors for faster lookups.
    let mut graph = Vec::new();
    let mut map = HashMap::new();
    for (&(x, y), edges) in paths.iter() {
        fn generate_id(
            map: &mut HashMap<(u32, u32), u32>,
            edges: &mut Vec<Vec<(u32, u32, bool)>>,
            x: u32,
            y: u32,
        ) -> u32 {
            if let Some(&id) = map.get(&(x, y)) {
                id
            } else {
                let id = edges.len() as u32;
                edges.push(Vec::new());
                map.insert((x, y), id);
                id
            }
        }

        let from = generate_id(&mut map, &mut graph, x, y);
        for (&(end_x, end_y), &(path_length, sloped)) in edges.iter() {
            let to = generate_id(&mut map, &mut graph, end_x, end_y);
            graph[from as usize].push((to, path_length, sloped));
        }
    }

    let start_id = *map.get(&(start_x, start_y)).unwrap();
    let end_id = *map.get(&(end_x, end_y)).unwrap();

    // Retrieve the single edge connected to the end node. Since we cannot revisit a
    // node, encountering this node during the search indicates there are no further
    // possibilities to explore.
    let (one_before_id, cost_to_exit, _) = graph[end_id as usize][0];

    // Prune edges which, if taken, would require touching a node twice to reach the
    // end. These edges are those along the outside of the graph which move toward
    // the start.
    let mut visited = BitSet::new(graph.len() as u32);
    let mut stack = vec![start_id];
    while let Some(id) = stack.pop() {
        graph[id as usize].retain(|&(to_id, _, _)| !visited.get(to_id));
        visited.set(id);

        for &(to_id, _, _) in graph[id as usize].iter() {
            if visited.get(to_id) || to_id == one_before_id {
                continue;
            }

            if graph[to_id as usize].len() == 3 {
                stack.push(to_id);
                continue;
            }
        }
    }

    let mut part1 = 0;
    let mut visited = BitSet::new(graph.len() as u32);
    let mut to_visit = vec![IterationState::Visit(start_id, 0)];
    while let Some(state) = to_visit.pop() {
        match state {
            IterationState::Visit(id, count) => {
                if id == one_before_id {
                    part1 = u64::max(part1, count + cost_to_exit as u64);
                    continue;
                }

                to_visit.push(IterationState::Unset(id));
                visited.set(id);

                for &(to_id, length, sloped) in graph[id as usize].iter() {
                    if !sloped {
                        continue;
                    }

                    if visited.get(to_id) {
                        continue;
                    }

                    to_visit.push(IterationState::Visit(to_id, count + length as u64));
                }
            }
            IterationState::Unset(id) => {
                visited.unset(id);
            }
        }
    }

    let mut part2 = 0;
    let mut visited = BitSet::new(graph.len() as u32);
    let mut to_visit = vec![IterationState::Visit(start_id, 0)];
    while let Some(state) = to_visit.pop() {
        match state {
            IterationState::Visit(id, count) => {
                if id == one_before_id {
                    part2 = u64::max(part2, count + cost_to_exit as u64);
                    continue;
                }

                to_visit.push(IterationState::Unset(id));
                visited.set(id);

                for &(to_id, length, _) in graph[id as usize].iter() {
                    if visited.get(to_id) {
                        continue;
                    }

                    to_visit.push(IterationState::Visit(to_id, count + length as u64));
                }
            }
            IterationState::Unset(id) => {
                visited.unset(id);
            }
        }
    }

    Solution::from((part1, part2))
}

enum IterationState {
    Visit(u32, u64),
    Unset(u32),
}

#[derive(Debug, Default, Hash, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Slope(Direction),
    Forest,
    #[default]
    Path,
}
