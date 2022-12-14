# Advent of Code in Rust

This repository contains solutions for the yearly [Advent of code](https://adventofcode.com) challenges.
The idea for this repo is to mirror [my solutions in F\#](https://github.com/OliverFlecke/advent-oc-code) – which I have usually been using to solve these problems – and the features to interact with the site programmatically, but build with Rust instead.
This is mostly as a learning experience.

## Executing the problem

The solutions can be run an executed using `cargo`, with some tests for both the library and problems for the individual days.
These are both implemented with Rust's standard test structure, and can therefore be execute with `cargo test`.

To run solutions for a given year and day (note the `y` prefix on the year):

```sh
cargo solve y<year> <day>
```

Use the `-a` or `-b` to submit part A and B for the individual days.

### Toolchain

Currently validated Rust version is `v1.65.0`. No nightly features are used.

Solutions are implemented in Rust, and are normally run with the latest version of the language (but usually only updated each december in this project).
See [here for installation instructions](https://www.rust-lang.org/learn/get-started).

## Authentication

To use the helper functions to retreive input and submit answers, the library will look for a AOC_TOKEN environment variable with a valid cookie session.

- Go to [adventofcode.com](https://adventofcode.com) and login
- Open the developer settings in your browser (F12)
- Go to `application` -> `Cookies`.
- You should see a session variable - this is the token we need.
- Add this to your environment with `export AOC_TOKEN=<your token>`

(I usually put it in a `.token` file to easily load the variable in later sessions)

## Features

- [x] Authentication against the AoC server
- [x] Retreiving problem inputs for each day
  - [x] Local caching of inputs (these are stored under a `.input` directory in the repository's root)
  - Note that a few problems cannot be downloaded automatically, as it is part of the description page.
- [x] Submitting answers to the AoC server
  - [ ] This is currently only available through the CLI, but could be exposed as a crate.
- [x] Benchmarking of solutions

## How to add a new solution

To add a new solution, create a `struct` that implements the `Solution` trait.
This requires you to implement two functions, one for each of the parts.
Both of these takes a string as input and should return an `Answer`.

The `solution.rs` also contains a helper function that can generate the correct solution struct for a given day.
When adding a new solution struct, it should also be added to this method.

## Benchmarking solutions

Each year of solutions can be benchmarked together.
This can be done by running the binary **without** the day parameter, e.g. to benchmark year 2017 use:
```sh
cargo run -r -- y<year>
```

Remember to use the `-r` to build in release mode for proper results.
