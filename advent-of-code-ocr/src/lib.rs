use std::collections::HashMap;

use itertools::Itertools;
use phf::phf_map;

static CHARACTER_MAP: phf::Map<&'static str, char> = phf_map! {
    ".##.\n#..#\n#..#\n####\n#..#\n#..#" => 'A',
    "###.\n#..#\n###.\n#..#\n#..#\n###." => 'B',
    ".##.\n#..#\n#...\n#...\n#..#\n.##." => 'C',
    "####\n#...\n###.\n#...\n#...\n####" => 'E',
    "####\n#...\n###.\n#...\n#...\n#..." => 'F',
    ".##.\n#..#\n#...\n#.##\n#..#\n.###" => 'G',
    "#..#\n#..#\n####\n#..#\n#..#\n#..#" => 'H',
    ".###\n..#.\n..#.\n..#.\n..#.\n.###" => 'I',
    "..##\n...#\n...#\n...#\n#..#\n.##." => 'J',
    "#..#\n#.#.\n##..\n#.#.\n#.#.\n#..#" => 'K',
    "#...\n#...\n#...\n#...\n#...\n####" => 'L',
    ".##.\n#..#\n#..#\n#..#\n#..#\n.##." => 'O',
    "###.\n#..#\n#..#\n###.\n#...\n#..." => 'P',
    "###.\n#..#\n#..#\n###.\n#.#.\n#..#" => 'R',
    ".###\n#...\n#...\n.##.\n...#\n###." => 'S',
    "#..#\n#..#\n#..#\n#..#\n#..#\n.##." => 'U',
    "#...\n#...\n.#.#\n..#.\n..#.\n..#." => 'Y',
    "####\n...#\n..#.\n.#..\n#...\n####" => 'Z',
};

pub fn parse_letter(letter: &str) -> Option<char> {
    CHARACTER_MAP.get(letter).copied()
}

pub fn parse_string_to_letters(s: &str) -> String {
    let mut letters: HashMap<usize, Vec<char>> = HashMap::new();
    for l in s.lines() {
        for (n, chunk) in l.chars().chunks(5).into_iter().enumerate() {
            if letters.contains_key(&n) {
                letters.get_mut(&n).unwrap().push('\n');
                letters.get_mut(&n).unwrap().extend(chunk.take(4));
            } else {
                letters.entry(n).or_insert(chunk.take(4).collect());
            }
        }
    }

    letters
        .iter()
        .sorted_by_key(|(n, _)| **n)
        .map(|(_, v)| v)
        .map(|x| x.iter().collect::<String>())
        .filter_map(|x| parse_letter(x.as_str()))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn map_letter() {
        let a = ".##.
#..#
#..#
####
#..#
#..#";

        assert_eq!(parse_letter(a), Some('A'));
    }
}
