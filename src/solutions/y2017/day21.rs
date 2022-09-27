use std::{collections::HashMap, str::FromStr};

use crate::solutions::{answer::Answer, Solution};

const START: &str = ".#.
..#
###";


type Rules = HashMap<String, String>;

pub struct Day21;

impl Solution for Day21 {
    fn solve_a(&self, input: &str) -> Answer {
        let rules = Self::parse(input);
        let mut image = START.clone();

        Self::count_lights(image).into()
    }

    fn solve_b(&self, _input: &str) -> Answer {
        todo!()
    }
}

impl Day21 {
    fn parse(input: &str) -> Rules {
        input
            .lines()
            .map(|l| l.split(" => "))
            .fold(HashMap::new(), |mut state, mut split| {
                state.insert(
                    split.next().unwrap().replace("/", "\n").to_string(),
                    split.next().unwrap().replace("/", "\n").to_string(),
                );

                state
            })
    }

    /// Count the number of lights turned on in the image.
    /// A light is defined to be on if it is equal to '#'.
    fn count_lights(image: &str) -> usize {
        image.chars().filter(|c| *c == '#').count()
    }

    fn lookup(input: &String, rules: &Rules) -> String {
        let l = input;
        println!("Looking up: {l}");
        rules.get(l.as_str()).unwrap().replace("/", "\n")
    }

    fn change(input: &String, rules: &Rules) -> String {
        Self::lookup(input, rules)
    }

    fn get_size(input: &str) -> usize {
        input.chars().take_while(|c| *c != '\n').count()
    }

    fn divide(input: String) -> Vec<String> {
        let size = Self::get_size(input.as_str());
        let lookup_size = if size % 2 == 0 { 2 } else { 3 };
        let lines: Vec<&str> = input.lines().collect();
        let mut parts: Vec<String> = Vec::new();

        for y_start in (0..size).step_by(lookup_size) {
            for x_start in (0..size).step_by(lookup_size) {
                let mut s = String::with_capacity(lookup_size * lookup_size);

                for y in y_start..(y_start + lookup_size) {
                    s.push_str(&lines[y][x_start..(x_start + lookup_size)]);
                    s.push('\n');
                }
                parts.push(s.trim_end().to_string());
            }
        }

        parts
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";

    const FINAL: &str = "##.##.
#..#..
......
##.##.
#..#..
......";

    #[test]
    fn parse() {
        let rules = Day21::parse(INPUT);

        assert_eq!(*rules.get("..\n.#").unwrap(), "##.\n#..\n...");
        assert_eq!(
            *rules.get(".#.\n..#\n###").unwrap(),
            "#..#\n....\n....\n#..#"
        );
    }

    #[test]
    fn get_size_test() {
        assert_eq!(Day21::get_size(START), 3);

        let image = "#..#
....
....
#..#";
        assert_eq!(Day21::get_size(image), 4);
    }

    #[test]
    fn divide() {
        let image = "#..#
....
....
#..#";
        println!("Finding divides of:\n{image}");

        let divided = Day21::divide(image.to_string());
        let mut it = divided.iter();
        assert_eq!(it.next(), Some(&"#.\n..".to_string()));
        assert_eq!(it.next(), Some(&".#\n..".to_string()));
        assert_eq!(it.next(), Some(&"..\n#.".to_string()));
        assert_eq!(it.next(), Some(&"..\n.#".to_string()));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn lookup() {
        let rules = &Day21::parse(INPUT);
        assert_eq!(Day21::lookup(&"#.\n..".to_string(), rules), "##.\n#..\n...");
        assert_eq!(Day21::lookup(&".#\n..".to_string(), rules), "##.\n#..\n...");
        assert_eq!(Day21::lookup(&"..\n#.".to_string(), rules), "##.\n#..\n...");
        assert_eq!(Day21::lookup(&"..\n.#".to_string(), rules), "##.\n#..\n...");
    }

    #[test]
    fn change_test() {
        let rules = &Day21::parse(INPUT);
        let mut image = START.to_string();

        image = Day21::change(&image, rules);
        assert_eq!(image, "#..#\n....\n....\n#..#");

        image = Day21::change(&image, rules);
        assert_eq!(
            image,
            "##.##.
#..#..
......
##.##.
#..#..
......"
        );
    }

    #[test]
    fn count_lights() {
        assert_eq!(Day21::count_lights(FINAL), 12);
    }

    #[test]
    fn test_a() {
        assert_eq!(Day21 {}.solve_a(INPUT), Answer::UInt(12));
    }
}
