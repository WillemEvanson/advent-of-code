use util::Solution;

const MIN: f64 = 200000000000000.0;
const MAX: f64 = 400000000000000.0;

pub fn solve(input: &str) -> Solution {
    let hail = input
        .lines()
        .map(|line| {
            let (position, velocity) = line.split_once(" @ ").unwrap();

            let velocity = velocity
                .split(',')
                .map(|str| str.trim().parse::<f64>().unwrap())
                .collect::<Vec<_>>();
            let velocity = Vec3::from([velocity[0], velocity[1], velocity[2]]);

            let position = position
                .split(',')
                .map(|str| str.trim().parse::<f64>().unwrap())
                .collect::<Vec<_>>();

            let position = Vec3::from([position[0], position[1], position[2]]);
            (position, velocity)
        })
        .collect::<Vec<_>>();

    let mut part1 = 0;
    for i in 0..hail.len() {
        let (a, a_vel) = hail[i];
        let b = a + a_vel;

        let vertical = a_vel.x == 0.0;

        for &(c, b_vel) in hail.iter().skip(i + 1) {
            let d = c + b_vel;

            let dx1 = b.x - a.x;
            let dy1 = b.y - a.y;
            let dx2 = d.x - c.x;
            let dy2 = d.y - c.y;
            let dx3 = c.x - a.x;
            let dy3 = c.y - a.y;

            let s = (dx1 * dy3 - dy1 * dx3) / (dy1 * dx2 - dx1 * dy2);
            let t = if vertical {
                (c.y - a.y + (d.y - c.y) * s) / (b.y - a.y)
            } else {
                (c.x - a.x + (d.x - c.x) * s) / (b.x - a.x)
            };

            let x = c.x + (d.x - c.x) * s;
            let y = c.y + (d.y - c.y) * s;

            if t < 0.0 || s < 0.0 {
            } else if (MIN..=MAX).contains(&x) && (MIN..=MAX).contains(&y) {
                part1 += 1;
            }
        }
    }

    let rock_vel = {
        let moved_hail = hail
            .iter()
            .copied()
            .map(|(pos, vel)| (pos - hail[0].0, vel - hail[0].1))
            .collect::<Vec<_>>();

        let point0 = moved_hail[0].0;
        let point1 = moved_hail[1].0;
        let point2 = moved_hail[1].0 + moved_hail[1].1;

        let p1p0 = point1 - point0;
        let p2p0 = point2 - point0;
        let normal = p1p0.cross(p2p0);

        let plane = Plane {
            point: point0,
            normal,
        };

        let t2 = plane.intersect(moved_hail[2].0, moved_hail[2].1).unwrap();
        let p2 = moved_hail[2].0 + moved_hail[2].1 * t2;

        let t3 = plane.intersect(moved_hail[3].0, moved_hail[3].1).unwrap();
        let p3 = moved_hail[3].0 + moved_hail[3].1 * t3;

        (p3 - p2) / (t3 - t2) + hail[0].1
    };

    let (a, a_vel) = hail[0];
    let a_adj_vel = a_vel - rock_vel;
    let b = a + a_adj_vel;

    let (c, c_vel) = hail[1];
    let c_adj_vel = c_vel - rock_vel;
    let d = c + c_adj_vel;

    let dx1 = b.x - a.x;
    let dy1 = b.y - a.y;
    let dx2 = d.x - c.x;
    let dy2 = d.y - c.y;
    let dx3 = c.x - a.x;
    let dy3 = c.y - a.y;

    let s = (dx1 * dy3 - dy1 * dx3) / (dy1 * dx2 - dx1 * dy2);

    let x = c.x + (d.x - c.x) * s;
    let y = c.y + (d.y - c.y) * s;
    let z = c.z + (d.z - c.z) * s;
    let part2 = (x + y + z) as u64;

    Solution::from((part1, part2))
}

struct Plane {
    normal: Vec3,
    point: Vec3,
}

impl Plane {
    fn intersect(&self, origin: Vec3, dir: Vec3) -> Option<f64> {
        let denominator = self.normal.dot(dir);
        let p1p0 = self.point - origin;
        let t = p1p0.dot(self.normal) / denominator;
        if t >= 0.0 {
            Some(t)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(self, other: Self) -> Self {
        Self {
            x: diff_of_products(self.y, other.z, self.z, other.y),
            y: diff_of_products(self.z, other.x, self.x, other.z),
            z: diff_of_products(self.x, other.y, self.y, other.x),
        }
    }
}

impl From<[f64; 3]> for Vec3 {
    fn from(value: [f64; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

fn diff_of_products(a: f64, b: f64, c: f64, d: f64) -> f64 {
    let cd = c * d;
    let err = (-c).mul_add(d, cd);
    let dop = a.mul_add(b, -cd);
    dop + err
}
