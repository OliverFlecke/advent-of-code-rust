pub mod y2017;

pub trait Solution {
    fn solve_a<S>(input: S) -> String
    where
        S: AsRef<str>;

    fn solve_b<S>(input: S) -> String
    where
        S: AsRef<str>;
}
