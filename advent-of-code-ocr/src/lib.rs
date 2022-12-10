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

/// Parse a letter represented in the AoC format.
///
/// Example:
/// ```
/// # use advent_of_code_ocr::parse_letter;
/// let letter = "#..#\n#..#\n####\n#..#\n#..#\n#..#";
///
/// assert_eq!(parse_letter(letter), Some('H'));
/// ```
pub fn parse_letter(letter: &str) -> Option<char> {
    CHARACTER_MAP.get(letter).copied()
}

/// Parse a string representing a full screen from a AoC puzzle.
///
/// Note: The current version will return all the characters it can parse,
/// and ignore everyone that it cannot. To check individual characters, use the
/// `parse_letter` function to parse them and get an `Option<char>`.
///
/// Example:
/// ```
/// # use advent_of_code_ocr::parse_string_to_letters;
///
/// // Input is:
/// // ####.###....##.###..###..#..#..##..#..#.
/// // #....#..#....#.#..#.#..#.#.#..#..#.#..#.
/// // ###..#..#....#.###..#..#.##...#..#.####.
/// // #....###.....#.#..#.###..#.#..####.#..#.
/// // #....#....#..#.#..#.#.#..#.#..#..#.#..#.
/// // ####.#.....##..###..#..#.#..#.#..#.#..#.
/// let input = "####.###....##.###..###..#..#..##..#..#.\n#....#..#....#.#..#.#..#.#.#..#..#.#..#.\n###..#..#....#.###..#..#.##...#..#.####.\n#....###.....#.#..#.###..#.#..####.#..#.\n#....#....#..#.#..#.#.#..#.#..#..#.#..#.\n####.#.....##..###..#..#.#..#.#..#.#..#.";
///
/// assert_eq!(parse_string_to_letters(input), "EPJBRKAH");
/// ```
pub fn parse_string_to_letters(s: &str) -> String {
    split_screen(s)
        .iter()
        .filter_map(|x| parse_letter(x.as_str()))
        .collect()
}

/// Split a string representing a AoC screen into the section strings for the
/// individual characters. This assumes every character is 4 characters wide and a
/// single column is used to split the individual letters.
///
/// This will split the string:
/// ```text
/// ####.###....##.###..###..#..#..##..#..#.
/// #....#..#....#.#..#.#..#.#.#..#..#.#..#.
/// ###..#..#....#.###..#..#.##...#..#.####.
/// #....###.....#.#..#.###..#.#..####.#..#.
/// #....#....#..#.#..#.#.#..#.#..#..#.#..#.
/// ####.#.....##..###..#..#.#..#.#..#.#..#.
/// ```
///
/// into the vector of
///
/// ```text
/// ####    ###.    ..##    ###.    ###.    #..#    .##.    #..#
/// #...    #..#    ...#    #..#    #..#    #.#.    #..#    #..#
/// ###.    #..#    ...#    ###.    #..#    ##..    #..#    ####
/// #...    ###.    ...#    #..#    ###.    #.#.    ####    #..#
/// #...    #...    #..#    #..#    #.#.    #.#.    #..#    #..#
/// ####    #...    .##.    ###.    #..#    #..#    #..#    #..#
/// ```
pub fn split_screen(s: &str) -> Vec<String> {
    let mut letters: HashMap<usize, Vec<char>> = HashMap::new();
    for l in s.lines() {
        for (n, chunk) in l.chars().chunks(5).into_iter().enumerate() {
            if letters.contains_key(&n) {
                letters.get_mut(&n).unwrap().push('\n');
                letters.get_mut(&n).unwrap().extend(chunk.take(4));
            } else {
                letters.entry(n).or_insert_with(|| chunk.take(4).collect());
            }
        }
    }

    letters
        .iter()
        .sorted_by_key(|(n, _)| **n)
        .map(|(_, v)| v)
        .map(|x| x.iter().collect::<String>())
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

    #[test]
    fn split_screen_to_sections() {
        let screen = "####.###....##.###..###..#..#..##..#..#.\n#....#..#....#.#..#.#..#.#.#..#..#.#..#.\n###..#..#....#.###..#..#.##...#..#.####.\n#....###.....#.#..#.###..#.#..####.#..#.\n#....#....#..#.#..#.#.#..#.#..#..#.#..#.\n####.#.....##..###..#..#.#..#.#..#.#..#.";

        let binding = split_screen(screen);
        let mut it = binding.iter();
        assert_eq!(
            it.next(),
            Some(&"####\n#...\n###.\n#...\n#...\n####".to_string())
        );
        assert_eq!(
            it.next(),
            Some(&"###.\n#..#\n#..#\n###.\n#...\n#...".to_string())
        );
        assert_eq!(
            it.next(),
            Some(&"..##\n...#\n...#\n...#\n#..#\n.##.".to_string())
        );
        assert_eq!(
            it.next(),
            Some(&"###.\n#..#\n###.\n#..#\n#..#\n###.".to_string())
        );
        assert_eq!(
            it.next(),
            Some(&"###.\n#..#\n#..#\n###.\n#.#.\n#..#".to_string())
        );
        assert_eq!(
            it.next(),
            Some(&"#..#\n#.#.\n##..\n#.#.\n#.#.\n#..#".to_string())
        );
        assert_eq!(
            it.next(),
            Some(&".##.\n#..#\n#..#\n####\n#..#\n#..#".to_string())
        );
        assert_eq!(
            it.next(),
            Some(&"#..#\n#..#\n####\n#..#\n#..#\n#..#".to_string())
        );
        assert_eq!(it.next(), None);
    }
}
