use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use itertools::Either;

use crate::solutions::{answer::Answer, Solution};

pub struct Day07;

impl Solution for Day07 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let cmds = parse_commands(input);
        let root = get_files_and_dirs(cmds);
        let mut sum = 0;

        let mut queue = VecDeque::new();
        queue.push_front(root);

        while let Some(current) = queue.pop_back() {
            let size = current.borrow().get_size();
            if size <= 100_000 {
                sum += size;
            }
            current
                .borrow()
                .directories
                .iter()
                .for_each(|d| queue.push_front(d.clone()));
        }

        Some(sum.into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        const TOTAL_SPACE: usize = 70_000_000;
        const NEEDED_SPACE: usize = 30_000_000;

        let cmds = parse_commands(input);
        let root = get_files_and_dirs(cmds);
        let root_size = root.borrow().get_size();
        let unused = TOTAL_SPACE - root_size;

        let mut smallest = usize::MAX;
        let mut queue = VecDeque::new();
        queue.push_front(root);

        while let Some(current) = queue.pop_back() {
            let size = current.borrow().get_size();
            if size + unused >= NEEDED_SPACE {
                smallest = smallest.min(size);
            }
            current
                .borrow()
                .directories
                .iter()
                .for_each(|d| queue.push_front(d.clone()));
        }

        Some(smallest.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct File {
    name: String,
    size: usize,
}

impl TryFrom<&str> for File {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // TODO: Error handling
        let mut split = value.split(' ');
        Ok(Self {
            size: split.next().unwrap().parse::<usize>().unwrap(),
            name: split.next().unwrap().to_string(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Dir {
    name: String,
    parent: Option<Rc<RefCell<Dir>>>,
    directories: Vec<Rc<RefCell<Dir>>>,
    files: Vec<File>,
}

impl Dir {
    pub fn new(name: String) -> Self {
        Self {
            name,
            parent: None,
            directories: vec![],
            files: vec![],
        }
    }

    fn get_size(&self) -> usize {
        let files = self.files.iter().map(|f| f.size).sum::<usize>();
        let dirs = self
            .directories
            .iter()
            .map(|d| d.borrow().get_size())
            .sum::<usize>();

        files + dirs
    }

    #[allow(dead_code)]
    fn pretty_display(&self, depth: usize) {
        let padding = 4 * depth;
        for x in self.files.iter() {
            println!("{:>padding$}{:?}", "", x);
        }

        for d in self.directories.iter() {
            println!("{:>padding$}dir  {:?}", "", d.borrow().name);
            d.borrow().pretty_display(depth + 1);
        }
    }
}

impl TryFrom<&str> for Dir {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split(' ');
        split.next();
        Ok(Self {
            name: split.next().unwrap().to_string(),
            parent: None,
            directories: Vec::new(),
            files: Vec::new(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CdArg {
    Relative(String),
    Out,
    Root,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Command {
    Cd(CdArg),
    Ls(Vec<String>), // Holds the output from this command
}

impl TryFrom<&str> for Command {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("cd") {
            let mut split = value.split(' ');
            split.next();
            match split.next() {
                Some("/") => Ok(Command::Cd(CdArg::Root)),
                Some("..") => Ok(Command::Cd(CdArg::Out)),
                Some(x) => Ok(Command::Cd(CdArg::Relative(x.to_string()))),
                None => Err(()),
            }
        } else if value.starts_with("ls") {
            Ok(Command::Ls(
                value
                    .lines()
                    .skip(1)
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
            ))
        } else {
            Err(())
        }
    }
}

fn parse_file_or_dir(input: &str) -> Either<File, Dir> {
    if input.starts_with("dir") {
        Either::Right(Dir::try_from(input).unwrap())
    } else {
        Either::Left(File::try_from(input).unwrap())
    }
}

fn parse_commands(input: &str) -> Vec<Command> {
    input
        .split('$')
        .skip(1)
        .map(|x| Command::try_from(x.trim()).unwrap())
        .collect()
}

fn get_files_and_dirs(commands: Vec<Command>) -> Rc<RefCell<Dir>> {
    let root = Rc::new(RefCell::new(Dir::new("/".to_string())));
    let mut current_dir = root.clone();

    // Skip the first one, as that always seems to be `cd /`
    for cmd in commands.iter().skip(1) {
        match cmd {
            Command::Cd(CdArg::Root) => current_dir = root.clone(),
            Command::Cd(CdArg::Out) => {
                current_dir = current_dir
                    .clone()
                    .borrow()
                    .parent
                    .as_ref()
                    .unwrap()
                    .clone();
            }
            Command::Cd(CdArg::Relative(x)) => {
                current_dir = current_dir
                    .clone()
                    .borrow()
                    .directories
                    .iter()
                    .find(|d| d.borrow().name.as_str() == x)
                    .map(|x| x.clone())
                    .unwrap_or_else(|| Rc::new(RefCell::new(Dir::new(x.to_string()))));
            }
            Command::Ls(lines) => {
                lines
                    .iter()
                    .map(|l| parse_file_or_dir(l))
                    .for_each(|x| match x {
                        Either::Left(file) => current_dir.borrow_mut().files.push(file),
                        Either::Right(mut dir) => {
                            dir.parent = Some(current_dir.clone());
                            current_dir
                                .borrow_mut()
                                .directories
                                .push(Rc::new(RefCell::new(dir)));
                        }
                    });
            }
        }
    }

    root
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn parse_commands_test() {
        let commands = parse_commands(SAMPLE_INPUT);
        assert_eq!(
            commands,
            vec![
                Command::Cd(CdArg::Root),
                Command::Ls(vec![
                    "dir a".to_string(),
                    "14848514 b.txt".to_string(),
                    "8504156 c.dat".to_string(),
                    "dir d".to_string(),
                ]),
                Command::Cd(CdArg::Relative("a".to_string())),
                Command::Ls(vec![
                    "dir e".to_string(),
                    "29116 f".to_string(),
                    "2557 g".to_string(),
                    "62596 h.lst".to_string(),
                ]),
                Command::Cd(CdArg::Relative("e".to_string())),
                Command::Ls(vec!["584 i".to_string()]),
                Command::Cd(CdArg::Out),
                Command::Cd(CdArg::Out),
                Command::Cd(CdArg::Relative("d".to_string())),
                Command::Ls(vec![
                    "4060174 j".to_string(),
                    "8033020 d.log".to_string(),
                    "5626152 d.ext".to_string(),
                    "7214296 k".to_string(),
                ]),
            ]
        )
    }

    #[test]
    fn get_files_and_dirs_test() {
        let cmds = parse_commands(SAMPLE_INPUT);
        let root = get_files_and_dirs(cmds);
        root.borrow().pretty_display(0);
    }

    #[test]
    fn get_size_test() {
        let cmds = parse_commands(SAMPLE_INPUT);
        let root = get_files_and_dirs(cmds);
        assert_eq!(root.borrow().get_size(), 48381165);
        assert_eq!(
            root.borrow()
                .directories
                .iter()
                .find(|x| x.borrow().name == "a")
                .unwrap()
                .borrow()
                .get_size(),
            94853
        );
    }

    #[test]
    fn test_a() {
        assert_eq!(Day07.solve_a(SAMPLE_INPUT), Some(Answer::UInt(95437)))
    }

    #[test]
    fn test_b() {
        assert_eq!(Day07.solve_b(SAMPLE_INPUT), Some(Answer::UInt(24933642)))
    }
}
