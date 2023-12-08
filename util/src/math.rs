#[inline]
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[inline]
pub fn lcm(a: u64, b: u64) -> u64 {
    if a > b {
        (a / gcd(a, b)) * b
    } else {
        (b / gcd(a, b)) * a
    }
}
