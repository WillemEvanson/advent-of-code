use util::bit_set::BitSet;
use util::grid::Grid;
use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut bricks = Vec::new();
    for line in input.lines() {
        let (start, end) = line.split_once('~').unwrap();
        let start: [u32; 3] = start
            .split(',')
            .map(|str| str.parse::<u32>().unwrap())
            .array_chunks()
            .next()
            .unwrap();
        let end: [u32; 3] = end
            .split(',')
            .map(|str| str.parse::<u32>().unwrap())
            .array_chunks()
            .next()
            .unwrap();

        bricks.push(Bounds::new(Vec3::from(start), Vec3::from(end)));

        max_x = u32::max(max_x, start[0]);
        max_x = u32::max(max_x, end[0]);
        max_y = u32::max(max_y, start[0]);
        max_y = u32::max(max_y, end[0]);
    }
    bricks.sort_by_key(|bounds| bounds.min.z);

    let mut graph = vec![(Vec::new(), Vec::new()); bricks.len()];
    let mut heightmap = Grid::new(max_x + 1, max_y + 1);
    for i in 0..bricks.len() {
        let mut bounds = bricks[i];

        // Determine where we can drop to
        let mut max_z = 0;
        for x in bounds.min.x..=bounds.max.x {
            for y in bounds.min.y..=bounds.max.y {
                max_z = u32::max(max_z, *heightmap.get_at(x, y).unwrap());
            }
        }

        // Drop the brick to that location
        let offset_z = bounds.min.z - (max_z + 1);
        bounds.min.z -= offset_z;
        bounds.max.z -= offset_z;
        bricks[i] = bounds;

        // Update heightmap
        for x in bounds.min.x..=bounds.max.x {
            for y in bounds.min.y..=bounds.max.y {
                heightmap.set(x, y, bounds.max.z);
            }
        }

        // Calcalate which boxes are supporting this brick.
        bounds.min.z -= 1;
        let mut supported_by = Vec::new();
        for (j, brick) in bricks.iter().enumerate().take(i) {
            if brick.intersects(bounds) {
                supported_by.push(j as u32);
            }
        }

        // Add edges to graph
        for &j in supported_by.iter() {
            graph[j as usize].0.push(i as u32);
        }
        graph[i].1 = supported_by;
    }

    let mut part1 = 0;
    let mut part2 = 0;
    let mut stack = Vec::new();
    let mut falling = BitSet::new(bricks.len() as u32);
    for i in 0..bricks.len() as u32 {
        stack.clear();
        falling.clear();

        stack.push(i);
        falling.set(i);
        while let Some(i) = stack.pop() {
            for &j in graph[i as usize].0.iter() {
                if !falling.get(j) && graph[j as usize].1.iter().all(|k| falling.get(*k)) {
                    falling.set(j);
                    stack.push(j);
                }
            }
        }

        if falling.count() - 1 == 0 {
            part1 += 1;
        } else {
            part2 += falling.count() as u64 - 1;
        }
    }

    Solution::from((part1, part2))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Bounds {
    min: Vec3,
    max: Vec3,
}

impl Bounds {
    pub fn new(v0: Vec3, v1: Vec3) -> Self {
        let min = Vec3::min(v0, v1);
        let max = Vec3::max(v0, v1);
        Self { min, max }
    }

    pub fn intersects(&self, other: Self) -> bool {
        let x = (self.min.x <= other.min.x && other.min.x <= self.max.x)
            || (self.min.x <= other.max.x && other.max.x <= self.max.x)
            || (other.min.x <= self.max.x && self.max.x <= other.max.x);

        let y = (self.min.y <= other.min.y && other.min.y <= self.max.y)
            || (self.min.y <= other.max.y && other.max.y <= self.max.y)
            || (other.min.y <= self.max.y && self.max.y <= other.max.y);

        let z = (self.min.z <= other.min.z && other.min.z <= self.max.z)
            || (self.min.z <= other.max.z && other.max.z <= self.max.z)
            || (other.min.z <= self.max.z && self.max.z <= other.max.z);

        x && y && z
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec3 {
    x: u32,
    y: u32,
    z: u32,
}

impl From<[u32; 3]> for Vec3 {
    fn from(value: [u32; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl Vec3 {
    pub fn min(self, other: Vec3) -> Vec3 {
        Self {
            x: u32::min(self.x, other.x),
            y: u32::min(self.y, other.y),
            z: u32::min(self.z, other.z),
        }
    }

    pub fn max(self, other: Vec3) -> Vec3 {
        Self {
            x: u32::max(self.x, other.x),
            y: u32::max(self.y, other.y),
            z: u32::max(self.z, other.z),
        }
    }
}
