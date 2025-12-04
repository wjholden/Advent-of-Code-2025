use std::collections::HashSet;

use advent_of_code_2025::*;

const PUZZLE: &str = include_str!("../../puzzles/day04.txt");

/// I really didn't think this select-and-prune technique was going to be fast
/// enough for part 2, but it works.
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("Part 2: {}", d.part2);
}

#[derive(Debug)]
pub struct Puzzle {
    pub part1: usize,
    pub part2: usize,
    rolls: HashSet<(isize, isize)>,
}

impl Puzzle {
    fn removable(&self) -> HashSet<(isize, isize)> {
        let mut candidates = HashSet::new();
        let diff = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        for (i, j) in self.rolls.iter() {
            let mut adj = 0;
            for (di, dj) in diff {
                if self.rolls.contains(&(i + di, j + dj)) {
                    adj += 1;
                }
            }
            if adj < 4 {
                candidates.insert((*i, *j));
            }
        }
        candidates
    }
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let rolls = input
            .lines()
            .enumerate()
            .fold(HashSet::new(), |mut acc, (i, line)| {
                for (j, c) in line.char_indices() {
                    match c {
                        '@' => {
                            acc.insert((i as isize, j as isize));
                        }
                        '.' => {}
                        _ => panic!(),
                    };
                }
                acc
            });
        Self {
            part1: 0,
            part2: 0,
            rolls,
        }
    }

    fn solve(mut self) -> Self {
        self.part1 = self.removable().len();
        while let remove = self.removable()
            && !remove.is_empty()
        {
            self.part2 += remove.len();
            self.rolls.retain(|x| !remove.contains(x));
        }
        self
    }
}

#[cfg(test)]
mod day01 {
    use super::*;

    const SAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, 13);
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part2, 43);
    }
}
