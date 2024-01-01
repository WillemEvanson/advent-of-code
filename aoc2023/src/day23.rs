use fxhash::{FxHashMap, FxHashSet};
use util::{BitSet, Solution};

pub fn solve(input: &str) -> Solution {
    let grid = Grid::from_str(input);

    let start_y = 0;
    let start_x = (0..grid.width as i32)
        .find(|&i| !matches!(grid.get_tile(i, start_y), Some(Tile::Forest)))
        .unwrap();

    let end_y = grid.height as i32 - 1;
    let end_x = (0..grid.width as i32)
        .find(|&i| !matches!(grid.get_tile(i, end_y), Some(Tile::Forest)))
        .unwrap();

    let mut visited = FxHashSet::default();
    let mut todo = vec![(start_x, start_y, Dir::Down)];
    let mut paths: FxHashMap<(i32, i32), FxHashMap<(i32, i32), (u32, bool)>> = FxHashMap::default();
    while let Some((mut x, mut y, mut dir)) = todo.pop() {
        if visited.contains(&(x, y, dir)) {
            continue;
        }
        visited.insert((x, y, dir));

        let path_start_x = x;
        let path_start_y = y;

        let (new_x, new_y) = dir.offset(x, y);
        x = new_x;
        y = new_y;

        let mut slope_forward = false;
        let mut slope_backward = false;

        let mut path_len = 0;
        loop {
            if let Some(Tile::Slope(slope_dir)) = grid.get_tile(x, y) {
                if slope_dir == dir {
                    slope_forward = true;
                } else {
                    slope_backward = true;
                }
            }

            let mut straight = false;
            let (new_x, new_y) = dir.offset(x, y);
            if let Some(Tile::Path | Tile::Slope(_)) = grid.get_tile(new_x, new_y) {
                straight = true;
            }

            let mut left = false;
            let (new_x, new_y) = dir.left().offset(x, y);
            if let Some(Tile::Path | Tile::Slope(_)) = grid.get_tile(new_x, new_y) {
                left = true;
            }

            let mut right = false;
            let (new_x, new_y) = dir.right().offset(x, y);
            if let Some(Tile::Path | Tile::Slope(_)) = grid.get_tile(new_x, new_y) {
                right = true;
            }

            let count = straight as u8 + left as u8 + right as u8;
            if count == 1 {
                visited.insert((x, y, dir));
                path_len += 1;
                if straight {
                    let (new_x, new_y) = dir.offset(x, y);
                    x = new_x;
                    y = new_y;
                } else if left {
                    let new_dir = dir.left();
                    let (new_x, new_y) = new_dir.offset(x, y);
                    dir = new_dir;
                    x = new_x;
                    y = new_y;
                } else if right {
                    let new_dir = dir.right();
                    let (new_x, new_y) = new_dir.offset(x, y);
                    dir = new_dir;
                    x = new_x;
                    y = new_y;
                }
            } else if count > 1 {
                if straight {
                    todo.push((x, y, dir));
                }
                if left {
                    let new_dir = dir.left();
                    todo.push((x, y, new_dir));
                }
                if right {
                    let new_dir = dir.right();
                    todo.push((x, y, new_dir));
                }
                break;
            } else {
                break;
            }
        }

        if path_len != 0 {
            let first = (path_start_x, path_start_y);
            let last = (x, y);

            paths
                .entry(first)
                .or_default()
                .insert(last, (path_len + 1, slope_forward));
            paths
                .entry(last)
                .or_default()
                .insert(first, (path_len + 1, slope_backward));
        }
    }

    // Transform graph into vector of vectors for faster lookups
    let mut edges = Vec::new();
    let mut map = FxHashMap::default();
    for (&(x, y), _edges) in paths.iter() {
        let from = if let Some(&id) = map.get(&(x, y)) {
            id
        } else {
            let id = edges.len() as u32;
            map.insert((x, y), id);
            edges.push(Vec::new());
            id
        };

        for (&(end_x, end_y), &(path_len, sloped)) in _edges.iter() {
            let to = if let Some(&id) = map.get(&(end_x, end_y)) {
                id
            } else {
                let id = edges.len() as u32;
                map.insert((end_x, end_y), id);
                edges.push(Vec::new());
                id
            };

            edges[from as usize].push((to, path_len, sloped));
        }
    }

    let start_id = *map.get(&(start_x, start_y)).unwrap();
    let end_id = *map.get(&(end_x, end_y)).unwrap();
    let (one_before_id, cost_to_exit, _) = edges[end_id as usize][0];

    // Part 1
    let mut part1 = 0;
    let mut to_visit = vec![(start_id, 0, BitSet::new(edges.len()))];
    while let Some((id, count, mut visited)) = to_visit.pop() {
        if id == one_before_id {
            part1 = u64::max(count + cost_to_exit as u64, part1);
            continue;
        }

        if visited.get(id as usize) {
            continue;
        }
        visited.set(id as usize);

        for &(to_id, path_len, slope_traversable) in edges[id as usize].iter() {
            if slope_traversable && !visited.get(to_id as usize) {
                to_visit.push((to_id, count + path_len as u64, visited.clone()));
            }
        }
    }

    // Compute minimum steps from each node to end node
    let mut max_distance = 0;
    let mut bfs = vec![(end_id, 0)];
    let mut distances = vec![u32::MAX; edges.len()];
    while let Some((id, distance)) = bfs.pop() {
        if distances[id as usize] <= distance {
            continue;
        }

        distances[id as usize] = distance;
        max_distance = u32::max(max_distance, distance);
        for &(to, _, _) in edges[id as usize].iter() {
            bfs.push((to, distance + 1));
        }
    }

    let mut level_sets = vec![0u8; max_distance as usize + 1];
    for &dist in distances.iter() {
        level_sets[dist as usize] += 1;
    }

    let mut best_possible = vec![0; edges.len()];
    for (i, edges) in edges.iter().enumerate() {
        for &(_, path_len, _) in edges.iter() {
            best_possible[i] = u32::max(best_possible[i], path_len);
        }
    }

    // Prune edges that, if taken, would result in being unable to reach the end
    // node.
    let mut visited = BitSet::new(edges.len());
    let mut stack = vec![start_id];
    while let Some(id) = stack.pop() {
        edges[id as usize].retain(|&(to_id, _, _)| !visited.get(to_id as usize));
        visited.set(id as usize);

        for &(to_id, _, _) in edges[id as usize].iter() {
            if !visited.get(to_id as usize) && to_id != one_before_id {
                if edges[to_id as usize].len() == 3 {
                    stack.push(to_id);
                    continue;
                }
            }
        }
    }

    // Part 2
    let mut part2 = 0;
    let mut visited = BitSet::new(edges.len());
    let mut level_sets = level_sets;
    let mut best_remaining: u64 = best_possible
        .iter()
        .filter(|i| **i != start_id)
        .map(|i| *i as u64)
        .sum();
    let mut stack = vec![RecursionState::Visit(start_id, 0)];
    while let Some(state) = stack.pop() {
        match state {
            RecursionState::Visit(id, count) => {
                if id == one_before_id {
                    part2 = u64::max(count + cost_to_exit as u64, part2);
                    continue;
                }
                let current_distance = distances[id as usize];

                // This results in all edges after this having the proper state in the visited
                // bitset.
                best_remaining -= best_possible[id as usize] as u64;
                level_sets[current_distance as usize] -= 1;
                visited.set(id as usize);
                stack.push(RecursionState::Unset(id));

                for &(to_id, path_len, _) in edges[id as usize].iter() {
                    if level_sets[current_distance as usize] == 0
                        && distances[to_id as usize] >= current_distance
                    {
                        continue;
                    }

                    let new_best_remaining = best_remaining - best_possible[to_id as usize] as u64;
                    if count + path_len as u64 + new_best_remaining < part2 {
                        continue;
                    }

                    if !visited.get(to_id as usize) {
                        stack.push(RecursionState::Visit(to_id, count + path_len as u64));
                    }
                }
            }
            // This is used to massively reduce the amount of copying and allocations that are
            // necessary by not putting the visited and level sets into the recursion state.
            RecursionState::Unset(id) => {
                best_remaining += best_possible[id as usize] as u64;
                level_sets[distances[id as usize] as usize] += 1;
                visited.reset(id as usize);
            }
        }
    }

    Solution::from((part1, part2))
}

pub enum RecursionState {
    Visit(u32, u64),
    Unset(u32),
}

#[derive(Debug, Clone)]
struct Grid {
    tiles: Box<[Tile]>,
    height: usize,
    width: usize,
}

impl Grid {
    pub fn from_str(input: &str) -> Self {
        let grid = input
            .lines()
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '#' => Tile::Forest,
                    '.' => Tile::Path,
                    '<' => Tile::Slope(Dir::Left),
                    '>' => Tile::Slope(Dir::Right),
                    '^' => Tile::Slope(Dir::Up),
                    'v' => Tile::Slope(Dir::Down),
                    _ => panic!(),
                })
            })
            .collect::<Box<_>>();
        Self {
            tiles: grid,
            height: input.lines().count(),
            width: input.lines().next().unwrap().len(),
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<Tile> {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            Some(self.tiles[y as usize * self.width + x as usize])
        } else {
            None
        }
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Slope(Dir),
    Path,
    Forest,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
}

impl Dir {
    pub fn offset(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Self::Up => (x, y - 1),
            Self::Down => (x, y + 1),
            Self::Right => (x + 1, y),
            Self::Left => (x - 1, y),
        }
    }

    pub fn left(&self) -> Dir {
        match self {
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up,
        }
    }

    pub fn right(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}
