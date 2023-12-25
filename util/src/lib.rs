mod bit_set;
mod random;
mod solution;

pub use bit_set::BitSet;
pub use random::Rng;
pub use solution::{Answer, Solution};

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    if a > b {
        (a / gcd(a, b)) * b
    } else {
        (b / gcd(a, b)) * a
    }
}
