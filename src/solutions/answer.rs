use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Answer {
    Int(i64),
    UInt(u64),
    String(String),
}

impl From<u64> for Answer {
    fn from(value: u64) -> Self {
        Answer::UInt(value)
    }
}
impl From<u32> for Answer {
    fn from(value: u32) -> Self {
        Answer::UInt(value as u64)
    }
}
impl From<usize> for Answer {
    fn from(value: usize) -> Self {
        Answer::UInt(value as u64)
    }
}
impl From<i64> for Answer {
    fn from(value: i64) -> Self {
        Answer::Int(value)
    }
}
impl From<i32> for Answer {
    fn from(value: i32) -> Self {
        Answer::Int(value as i64)
    }
}
impl From<&str> for Answer {
    fn from(input: &str) -> Self {
        Answer::String(input.to_string())
    }
}
impl From<String> for Answer {
    fn from(input: String) -> Self {
        Answer::String(input)
    }
}

impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Answer::Int(x) => x.to_string(),
                Answer::UInt(x) => x.to_string(),
                Answer::String(x) => x.to_string(),
            }
        )
    }
}
