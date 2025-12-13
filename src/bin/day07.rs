use std::collections::{HashMap, HashSet};

use advent_of_code_2025::*;

pub const PUZZLE: &str = include_str!("../../puzzles/day07.txt");

fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("Part 2: {}", d.part2);
    println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Default, Debug)]
pub struct Puzzle {
    pub part1: usize,
    pub part2: usize,
    start: (usize, usize),
    splitters: HashSet<(usize, usize)>,
    rows: usize,
    cols: usize,
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let mut instance = Self::default();
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.char_indices() {
                match c {
                    '.' => {}
                    '^' => {
                        instance.splitters.insert((row, col));
                    }
                    'S' => {
                        instance.start = (row, col);
                    }
                    _ => panic!("unexpected symbol in input"),
                };
                instance.cols = col.max(instance.cols);
            }
            instance.rows = row.max(instance.rows);
        }
        instance
    }

    fn solve(mut self) -> Self {
        let mut beams = HashMap::from([(self.start, 1usize)]);
        for row in 0..=self.rows {
            for col in 0..=self.cols {
                // If you don't remove the object, then you have to clone it.
                // This small optimization makes a sizable difference.
                if let Some(paths) = beams.remove(&(row, col)) {
                    if self.splitters.remove(&(row, col)) {
                        *beams.entry((row + 1, col - 1)).or_default() += paths;
                        *beams.entry((row + 1, col + 1)).or_default() += paths;
                        self.part1 += 1;
                    } else {
                        *beams.entry((row + 1, col)).or_default() += paths;
                    }
                    if row == self.rows {
                        self.part2 += paths;
                    }
                }
            }
        }
        self
    }
}

#[cfg(test)]
mod laboratories {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day07.txt");
    const MINE: &str = "..S..
..^..
.^.^.
..^..
.^.^.
..^..
.^.^.
..^..
.^.^.
.....";

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, 21);
    }

    #[test]
    fn test2() {
        // OMG part2, not part1. I spend like 30 minutes searching for a bug
        // that turned out to be this line checking against the wrong thing.
        assert_eq!(Puzzle::new(SAMPLE).solve().part2, 40);
    }

    #[test]
    fn small() {
        assert_eq!(Puzzle::new(MINE).solve().part2, 15 + 16 + 15);
    }
}
