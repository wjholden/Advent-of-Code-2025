use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code_2025::*;

pub const PUZZLE: &str = include_str!("../../puzzles/day11.txt");

fn main() {
    let d = Puzzle::new(PUZZLE);
    //println!("{}", d.paths("svr", "out").unwrap());
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("Part 2: {}", d.part2); // 4913080290762 too low.
    //println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Default, Debug)]
pub struct Puzzle {
    pub part1: usize,
    pub part2: usize,
    adj: HashMap<String, Vec<String>>,
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let input = input.replace(":", "");
        let mut instance = Self::default();
        for line in input.lines() {
            let mut line: Vec<String> = line.split_ascii_whitespace().map(str::to_owned).collect();
            instance.adj.insert(line.remove(0), line);
        }
        instance
    }

    fn solve(mut self) -> Self {
        if let Some(path_count) = self.paths("you", "out") {
            self.part1 = path_count;
        }

        if let (Some(a), Some(b), Some(c)) = (
            self.paths("svr", "fft"),
            self.paths("fft", "dac"),
            self.paths("dac", "out"),
        ) {
            println!("svr -> fft -> dac -> out: {}", a * b * c);
            self.part2 += a * b * c;
        } else {
            eprint!("no path svr -> fft -> dac -> out");
        }

        if let (Some(a), Some(b), Some(c)) = (
            self.paths("svr", "dac"),
            self.paths("dac", "fft"),
            self.paths("fft", "out"),
        ) {
            println!("svr -> dac -> fft -> out: {}", a * b * c);
            self.part2 += a * b * c;
        } else {
            eprintln!("no path svr -> dac -> fft -> out");
        }
        self
    }
}

impl Puzzle {
    fn paths(&self, start: &str, end: &str) -> Option<usize> {
        let mut queue = VecDeque::<String>::new();
        let mut paths: HashMap<String, usize> = HashMap::new();
        let mut explored: HashSet<String> = HashSet::new();
        queue.push_back(String::from(start));
        paths.insert(String::from(start), 1);
        while let Some(u) = queue.pop_front() {
            if explored.contains(&u) {
                continue;
            }
            let paths_to_u = paths.get(&u).unwrap().clone();
            if let Some(v) = self.adj.get(&u) {
                for v in v.iter() {
                    if explored.contains(v) {
                        println!(
                            "Not a DAG? We've found a path to {v}, which is already explored."
                        );
                        continue;
                    }
                    *paths.entry(v.clone()).or_default() += paths_to_u;
                    queue.push_back(v.clone());
                }
            }

            explored.insert(u.clone());
        }
        match paths.get(end) {
            Some(path_count) => Some(path_count.clone()),
            None => None,
        }
    }
}

#[cfg(test)]
mod puzzle_name {
    use super::*;

    const SAMPLE1: &str = include_str!("../../samples/day11-part1.txt");
    const SAMPLE2: &str = include_str!("../../samples/day11-part2.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE1).solve().part1, 5);
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE2).solve().part2, 2);
    }
}
