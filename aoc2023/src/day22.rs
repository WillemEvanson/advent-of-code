use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut bricks = Vec::new();
    for line in input.lines() {
        let (start, end) = line.split_once('~').unwrap();
        let start: [u64; 3] = start
            .split(',')
            .map(|str| str.parse::<u64>().unwrap())
            .array_chunks()
            .next()
            .unwrap();
        let end: [u64; 3] = end
            .split(',')
            .map(|str| str.parse::<u64>().unwrap())
            .array_chunks()
            .next()
            .unwrap();

        bricks.push(Bounds::new(Vec3::from(start), Vec3::from(end)));
    }
    bricks.sort_by_key(|bounds| bounds.min.z);
    fall(&mut bricks);

    let mut part1 = 0;
    let mut part2 = 0;
    for i in 0..bricks.len() {
        let mut bricks_ = bricks.clone();
        bricks_.remove(i);

        let count = fall(&mut bricks_);
        if count == 0 {
            part1 += 1;
        }
        part2 += count;
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
    x: u64,
    y: u64,
    z: u64,
}

impl From<[u64; 3]> for Vec3 {
    fn from(value: [u64; 3]) -> Self {
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
            x: u64::min(self.x, other.x),
            y: u64::min(self.y, other.y),
            z: u64::min(self.z, other.z),
        }
    }

    pub fn max(self, other: Vec3) -> Vec3 {
        Self {
            x: u64::max(self.x, other.x),
            y: u64::max(self.y, other.y),
            z: u64::max(self.z, other.z),
        }
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

fn fall(bricks: &mut [Bounds]) -> u64 {
    let mut fallen = 0;
    for i in 0..bricks.len() {
        let mut bounds = bricks[i];

        let mut count = 0;
        'fall: while bounds.min.z != 1 {
            bounds.max.z -= 1;
            bounds.min.z -= 1;

            for j in (0..i).rev() {
                if bricks[j].intersects(bounds) {
                    break 'fall;
                }
            }
            bricks[i] = bounds;
            count += 1;
        }

        if count != 0 {
            fallen += 1;
        }
    }
    fallen
}
