# 🎅🎄 Client for Advent of Code 🎄🎅

![Crates.io](https://img.shields.io/crates/l/advent-of-code-client)
[![crates.io](https://img.shields.io/crates/v/advent-of-code-client.svg)](https://crates.io/crates/advent-of-code-client)
[![Documentation](https://docs.rs/advent-of-code-client/badge.svg)](https://docs.rs/advent-of-code-client)

Crate to interact with the yearly challenges at [Advent of Code](https://adventofcode.com).
It functions as both a Rust library and an standalone CLI client.

## Installation

Install with:

```sh
cargo install advent-of-code-client
```

It should now be accessable as `aoc`.

## Authentication

To authorize against the Advent of Code site, the CLI expects `AOC_TOKEN` to be set in your environment with a valid session token, or manually provided with the `--token` flag.

After authenticating on the website, your session token can be found through the following steps (might vary a bit based on your browser):

- Go to [adventofcode.com](https://adventofcode.com) and login
- Open the developer settings in your browser (F12)
- Go to `application` -> `Cookies`.
- You should see a session variable - this is the token we need.
- Add this to your environment with `export AOC_TOKEN=<your token>`

## CLI usage

To submit a solution for a given year and day:

```sh
aoc 2023 1 -a <your answer>
```

Use `-a` to submit an answer for part A, `-b` for part B.
