use util::Solution;

pub fn solve(input: &str) -> Solution {
    let mut sections = input.split("\n\n");
    let mut seeds = sections
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|str| str.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    for section in sections {
        let mut mappings = Vec::new();
        for line in section.lines().skip(1) {
            let (dst, rem) = line.split_once(' ').unwrap();
            let (src, len) = rem.split_once(' ').unwrap();

            let dst = dst.parse::<u64>().unwrap();
            let src = src.parse::<u64>().unwrap();
            let len = len.parse::<u64>().unwrap();

            mappings.push((dst, src, len));
        }

        for seed in seeds.iter_mut() {
            if let Some(&(dst, src, _)) = mappings
                .iter()
                .find(|&&(_, src, range)| (src..src + range).contains(seed))
            {
                *seed = dst + (*seed - src);
            }
        }
    }
    let part1 = *seeds.iter().min().unwrap();

    let mut sections = input.split("\n\n");
    let mut seeds = sections
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|str| str.parse::<u64>().unwrap())
        .array_chunks::<2>()
        .map(|[start, len]| Range::new(start, start + len))
        .collect::<Vec<_>>();

    for section in sections {
        let mut mappings = Vec::new();
        for line in section.lines().skip(1) {
            let (dst, rem) = line.split_once(' ').unwrap();
            let (src, len) = rem.split_once(' ').unwrap();

            let dst = dst.parse::<u64>().unwrap();
            let src = src.parse::<u64>().unwrap();
            let len = len.parse::<u64>().unwrap();

            mappings.push((dst, src, len));
        }

        let mut i = 0;
        'seeds: while let Some(&range) = seeds.get(i) {
            for &(dst, src, len) in mappings.iter() {
                let src_range = Range::new(src, src + len);
                if src_range.contains(range) {
                    seeds[i] = dst + (range - src);

                    i += 1;
                    continue 'seeds;
                }

                let intersection = range.intersect(src_range);
                if !intersection.is_empty() {
                    let (lower, upper) = range.difference(src_range);

                    seeds[i] = intersection;
                    if !lower.is_empty() && range.contains(lower) {
                        seeds.push(lower);
                    }
                    if !upper.is_empty() && range.contains(upper) {
                        seeds.push(upper);
                    }
                    continue 'seeds;
                }
            }
            i += 1;
        }
    }
    let part2 = seeds.iter().min().unwrap().start;

    Solution::from((part1, part2))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    fn is_empty(&self) -> bool {
        self.end.saturating_sub(self.start) == 0
    }

    fn contains(&self, other: Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn intersect(&self, other: Self) -> Self {
        Self {
            start: u64::max(self.start, other.start),
            end: u64::min(self.end, other.end),
        }
    }

    fn difference(&self, other: Range) -> (Range, Range) {
        let intersection = self.intersect(other);

        let lower = Self {
            start: u64::min(self.start, other.start),
            end: intersection.start,
        };
        let upper = Self {
            start: intersection.end,
            end: u64::max(self.end, other.end),
        };
        (lower, upper)
    }
}

impl std::ops::Add<u64> for Range {
    type Output = Range;

    fn add(self, rhs: u64) -> Self::Output {
        Self {
            start: self.start + rhs,
            end: self.end + rhs,
        }
    }
}

impl std::ops::Add<Range> for u64 {
    type Output = Range;

    fn add(self, rhs: Range) -> Self::Output {
        Range {
            start: rhs.start + self,
            end: rhs.end + self,
        }
    }
}

impl std::ops::Sub<u64> for Range {
    type Output = Range;

    fn sub(self, rhs: u64) -> Self::Output {
        Self {
            start: self.start - rhs,
            end: self.end - rhs,
        }
    }
}

impl std::ops::Sub<Range> for u64 {
    type Output = Range;

    fn sub(self, rhs: Range) -> Self::Output {
        Range {
            start: rhs.start - self,
            end: rhs.end - self,
        }
    }
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.start, self.end)
    }
}
