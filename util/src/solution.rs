#[derive(Debug, Clone)]
pub enum Solution {
    Two(Answer, Answer),
    One(Answer),
}

impl From<(u64, u64)> for Solution {
    fn from((part1, part2): (u64, u64)) -> Self {
        Self::Two(Answer::from(part1), Answer::from(part2))
    }
}

impl From<(u64, String)> for Solution {
    fn from((part1, part2): (u64, String)) -> Self {
        Self::Two(Answer::from(part1), Answer::from(part2))
    }
}

impl From<(String, u64)> for Solution {
    fn from((part1, part2): (String, u64)) -> Self {
        Self::Two(Answer::from(part1), Answer::from(part2))
    }
}

impl From<(String, String)> for Solution {
    fn from((part1, part2): (String, String)) -> Self {
        Self::Two(Answer::from(part1), Answer::from(part2))
    }
}

impl From<u64> for Solution {
    fn from(result: u64) -> Self {
        Self::One(Answer::from(result))
    }
}

impl From<String> for Solution {
    fn from(result: String) -> Self {
        Self::One(Answer::from(result))
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Two(part1, part2) => write!(f, "{part1}{part2}"),
            Self::One(result) => write!(f, "{result}{:<40}", ""),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Answer {
    String(String),
    Integer(u64),
}

impl From<String> for Answer {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<u64> for Answer {
    fn from(value: u64) -> Self {
        Self::Integer(value)
    }
}

impl std::fmt::Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(str) => write!(f, "{str:<40}"),
            Self::Integer(int) => write!(f, "{int:<40}"),
        }
    }
}
