use std::collections::HashSet;

use crate::solutions::{answer::Answer, Solution};

const RIGHT: i64 = 1;
const LEFT: i64 = -1;

pub struct Day25;

impl Solution for Day25 {
    fn solve_a(&self, _input: &str) -> Option<Answer> {
        const ITERATIONS: usize = 12_172_063;
        // const ITERATIONS: usize = 6;
        let mut tape: HashSet<i64> = HashSet::new();
        let mut state = State::A;
        let mut index: i64 = 0;

        for _ in 0..=ITERATIONS {
            match state {
                State::A => {
                    if !tape.contains(&index) {
                        tape.insert(index);
                        index += RIGHT;
                        state = State::B;
                    } else {
                        tape.remove(&index);
                        index += LEFT;
                        state = State::C;
                    }
                }
                State::B => {
                    if !tape.contains(&index) {
                        tape.insert(index);
                        index += LEFT;
                        state = State::A;
                    } else {
                        tape.insert(index);
                        index += LEFT;
                        state = State::D;
                    }
                }
                State::C => {
                    if !tape.contains(&index) {
                        tape.insert(index);
                        index += RIGHT;
                        state = State::D;
                    } else {
                        tape.remove(&index);
                        index += RIGHT;
                        state = State::C;
                    }
                }
                State::D => {
                    if !tape.contains(&index) {
                        tape.remove(&index);
                        index += LEFT;
                        state = State::B;
                    } else {
                        tape.remove(&index);
                        index += RIGHT;
                        state = State::E;
                    }
                }
                State::E => {
                    if !tape.contains(&index) {
                        tape.insert(index);
                        index += RIGHT;
                        state = State::C;
                    } else {
                        tape.insert(index);
                        index += LEFT;
                        state = State::F;
                    }
                }
                State::F => {
                    if !tape.contains(&index) {
                        tape.insert(index);
                        index += LEFT;
                        state = State::E;
                    } else {
                        tape.insert(index);
                        index += RIGHT;
                        state = State::A;
                    }
                }
            }
        }

        Some(tape.len().into())
    }

    fn solve_b(&self, _input: &str) -> Option<Answer> {
        None
    }
}

enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}

#[cfg(test)]
mod test {
    // use super::*;

    // This test does not work out of the box, as the example uses different
    // rules. The encoded rules below can be used:
    // State::A => {
    //     if !tape.contains(&index) {
    //         tape.insert(index);
    //         index += RIGHT;
    //         state = State::B;
    //     } else {
    //         tape.remove(&index);
    //         index += LEFT;
    //         state = State::B;
    //     }
    // }
    // State::B => {
    //     if !tape.contains(&index) {
    //         tape.insert(index);
    //         index += LEFT;
    //         state = State::A;
    //     } else {
    //         tape.insert(index);
    //         index += RIGHT;
    //         state = State::A;
    //     }
    // }
    // #[test]
    // fn test_a() {
    //     assert_eq!(Day25{}.solve_a(""), Answer::UInt(3));
    // }
}
