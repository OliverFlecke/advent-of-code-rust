# OCR for Advent of Code puzzles

Crate to convert [Advent of Code](https://adventofcode.com) from ASCII letters to characters.
This is required for several of the puzzle days through the years, including: [2016/09](https://adventofcode.com/2016/day/9), [2019/08](https://adventofcode.com/2019/day/9), [2019/11](https://adventofcode.com/2019/day/11), [2021/13](https://adventofcode.com/2021/day/13), and [2022/10](https://adventofcode.com/2022/day/10).

This has been made to help automate parsing of those puzzles for my own AoC solutions, but are shared for others interest.

## Installation

```sh
cargo install advent-of-code-ocr
```

## Usage

The main function to parse a screen from AoC is the `parse_string_to_letters`.

```rust
use advent_of_code_ocr::parse_string_to_letters;

// Input is:
// ####.###....##.###..###..#..#..##..#..#.
// #....#..#....#.#..#.#..#.#.#..#..#.#..#.
// ###..#..#....#.###..#..#.##...#..#.####.
// #....###.....#.#..#.###..#.#..####.#..#.
// #....#....#..#.#..#.#.#..#.#..#..#.#..#.
// ####.#.....##..###..#..#.#..#.#..#.#..#.
let input = "####.###....##.###..###..#..#..##..#..#.\n#....#..#....#.#..#.#..#.#.#..#..#.#..#.\n###..#..#....#.###..#..#.##...#..#.####.\n#....###.....#.#..#.###..#.#..####.#..#.\n#....#....#..#.#..#.#.#..#.#..#..#.#..#.\n####.#.....##..###..#..#.#..#.#..#.#..#.";

assert_eq!(parse_string_to_letters(input), "EPJBRKAH");
```

Two other functions are exposed by this crate:

- `parse_letter` which tries to convert a single AoC character to a `Option<char>`
- `split_screen` which splits a full AoC screen to individual AoC characters.
