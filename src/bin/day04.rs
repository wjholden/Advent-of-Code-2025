#[cfg(feature = "faster")]
use std::collections::HashMap;
use std::collections::HashSet;

use advent_of_code_2025::*;

const PUZZLE: &str = include_str!("../../puzzles/day04.txt");
const DIFF: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

/// I really didn't think this select-and-prune technique was going to be fast
/// enough for part 2, but it works.
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("Part 2: {}", d.part2);
    println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Debug)]
pub struct Puzzle {
    pub part1: usize,
    pub part2: usize,
    rolls: HashSet<(isize, isize)>,
}

impl Puzzle {
    #[cfg(not(feature = "faster"))]
    fn removable(&self) -> HashSet<(isize, isize)> {
        let mut candidates = HashSet::new();
        for (i, j) in self.rolls.iter() {
            let mut adj = 0;
            for (di, dj) in DIFF {
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

    #[cfg(feature = "faster")]
    fn adj(&self) -> HashMap<(isize, isize), usize> {
        self.rolls
            .iter()
            .map(|&(i, j)| {
                let mut adj = 0;
                for (di, dj) in DIFF {
                    if self.rolls.contains(&(i + di, j + dj)) {
                        adj += 1;
                    }
                }
                ((i, j), adj)
            })
            .collect()
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

    #[cfg(not(feature = "faster"))]
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

    #[cfg(feature = "faster")]
    fn solve(mut self) -> Self {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut adj = self.adj();
        self.part1 = adj.iter().filter(|&(_, &adj)| adj < 4).count();

        let mut heap = BinaryHeap::new();
        // Why clone? Well, we're going to need to mutate these values later,
        // but we can't do that if the heap is keeping references to these.
        for ((i, j), neighbors) in adj.clone().into_iter() {
            heap.push(Reverse((neighbors, (i, j))));
        }
        // I don't love this external data structure but we need it. Rust's
        // priority queue (`BinaryHeap`) doesn't allow us to update the keys
        // (I think Java could do this). So, we need to know which positions
        // we've already cleared.
        let mut removed = HashSet::new();

        loop {
            if let Some(Reverse((neighbors, (i, j)))) = heap.pop()
                && neighbors < 4
            {
                if removed.contains(&(i, j)) {
                    continue;
                }

                for (di, dj) in DIFF {
                    // Don't revisit anything we've already removed.
                    if removed.contains(&(i + di, j + dj)) {
                        continue;
                    }
                    if let Some(neighbors) = adj.get_mut(&(i + di, j + dj)) {
                        *neighbors -= 1;
                        heap.push(Reverse((*neighbors, (i + di, j + dj))));
                    }
                }
                removed.insert((i, j));
            } else {
                break;
            }
        }
        self.part2 = removed.len();

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
