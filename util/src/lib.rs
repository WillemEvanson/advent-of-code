mod bit_set;
mod lexer;
mod random;
mod solution;
mod union_find;

pub use bit_set::BitSet;
pub use lexer::Lexer;
pub use random::Rng;
pub use solution::{Answer, Solution};
pub use union_find::UnionFind;

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
