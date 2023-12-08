use std::fmt::Display;

use duplicate::duplicate_item;
use serde::{Deserialize, Serialize};

/// An answer that can be submitted to a AoC problem.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Answer {
    Int(i64),
    UInt(u64),
    String(String),
}

#[duplicate_item(
    int_type;
    [ usize ];
    [ u64 ];
    [ u32 ];
    [ u16 ];
    [ u8 ];
)]
impl From<int_type> for Answer {
    fn from(value: int_type) -> Self {
        #[allow(clippy::unnecessary_cast)]
        Answer::UInt(value as u64)
    }
}

#[duplicate_item(
    int_type;
    [ isize ];
    [ i64 ];
    [ i32 ];
    [ i16 ];
    [ i8 ];

)]
impl From<int_type> for Answer {
    fn from(value: int_type) -> Self {
        #[allow(clippy::unnecessary_cast)]
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
