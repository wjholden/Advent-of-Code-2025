#[cfg(feature = "faster")]
use std::cell::RefCell;
use std::collections::HashMap;

use advent_of_code_2025::*;
#[cfg(not(feature = "faster"))]
use nalgebra::DMatrix;

pub const PUZZLE: &str = include_str!("../../puzzles/day11.txt");

/// Can't believe this works. The idea for part 2 is you need to find the
/// the product of paths from `svr` to `fft`, `fft` to `dac`, and `dac` to `out`.
/// I don't know why not, but my part1 solver is under-counting these
/// paths when the graph is not a DAG. Apparently others
/// (https://www.reddit.com/r/adventofcode/comments/1pjrojm/2025_day_11_part_2_how_many_times_will_these/)
/// are using Depth-First Search. Mine is a Breadth-First Search. Need to
/// read what others have tried.
///
/// Update: it's been a few days and now I recognize how elegant the recurrence
/// relation really is. Takes us from 2.5s to about 450µs. I'm still impressed
/// with the matrix-based solution, but all that dense multiplication is slow.
/// See https://www.reddit.com/r/adventofcode/comments/1pnq2hu/2025_day_11_an_alternate_approach/.
/// Also, first time using RefCell!
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

    #[cfg(feature = "faster")]
    fn solve(mut self) -> Self {
        let memo = RefCell::new(HashMap::new());
        let p1 = self.paths("you", "out", &memo);
        let a = self.paths("svr", "fft", &memo);
        let b = self.paths("fft", "dac", &memo);
        let c = self.paths("dac", "out", &memo);
        // let d = self.paths("svr", "dac", &memo);
        // let e = self.paths("dac", "fft", &memo);
        // let f = self.paths("fft", "out", &memo);

        self.part1 = p1;
        self.part2 = a * b * c /* + d * e * f */;

        self
    }

    #[cfg(not(feature = "faster"))]
    fn solve(mut self) -> Self {
        if let Some(path_count) = self.paths("you", "out") {
            self.part1 = path_count;
        }

        self.part2 = self.part2();

        self
    }
}

impl Puzzle {
    #[cfg(feature = "faster")]
    fn paths<'a>(
        &'a self,
        src: &'a str,
        dst: &'a str,
        memo: &RefCell<HashMap<(&'a str, &'a str), usize>>,
    ) -> usize {
        if let Some(path_count) = memo.borrow().get(&(src, dst)) {
            return *path_count;
        }
        if src == dst {
            return 0;
        }
        if let Some(edges) = self.adj.get(src) {
            let mut path_count = 0;
            for edge in edges {
                if edge == dst {
                    path_count += 1;
                } else {
                    path_count += self.paths(edge, dst, memo);
                }
            }
            memo.borrow_mut().insert((src, dst), path_count);
            path_count
        } else {
            0
        }
    }

    #[cfg(not(feature = "faster"))]
    fn paths(&self, start: &str, end: &str) -> Option<usize> {
        use std::collections::{HashSet, VecDeque};

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

    #[cfg(not(feature = "faster"))]
    fn sorted_vertices(&self) -> Vec<String> {
        let mut v: Vec<String> = self.adj.keys().into_iter().cloned().collect();
        v.push("out".to_owned());
        v.sort();
        v
    }

    #[cfg(not(feature = "faster"))]
    fn as_matrix(&self) -> DMatrix<usize> {
        let v = self.sorted_vertices();
        DMatrix::from_fn(v.len(), v.len(), |i, j| {
            let src = &v[i];
            let dst = &v[j];
            if let Some(source) = self.adj.get(src)
                && source.contains(dst)
            {
                1
            } else {
                0
            }
        })
    }

    #[cfg(not(feature = "faster"))]
    /// OMG I can't believe this actually works!
    ///
    /// https://stackoverflow.com/a/6208818/5459668
    fn part2(&self) -> usize {
        let v = self.sorted_vertices();
        let mut m = self.as_matrix();
        let m_original = m.clone();
        let svr = v.iter().position(|e| *e == "svr").unwrap();
        let fft = v.iter().position(|e| *e == "fft").unwrap();
        let dac = v.iter().position(|e| *e == "dac").unwrap();
        let out = v.iter().position(|e| *e == "out").unwrap();
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        for _i in 1..v.len() {
            if m.iter().sum::<usize>() == 0 {
                break; // We can stop early once the entire matrix is empty.
            }
            // println!(
            //     "paths from svr to fft with {i} hops: {}",
            //     m.index((svr, fft))
            // );
            // println!(
            //     "paths from fft to dac with {i} hops: {}",
            //     m.index((fft, dac))
            // );
            // println!(
            //     "paths from dac to out with {i} hops: {}",
            //     m.index((dac, out))
            // );
            a += m.index((svr, fft));
            b += m.index((fft, dac));
            c += m.index((dac, out));
            m = m * &m_original;
        }
        // print!("{a} * {b} * {c} = ");
        // println!("{}", a * b * c);
        a * b * c
    }
}

#[cfg(test)]
mod reactor {
    use super::*;

    const SAMPLE1: &str = include_str!("../../samples/day11-part1.txt");
    const SAMPLE2: &str = include_str!("../../samples/day11-part2.txt");

    #[test]
    #[cfg(not(feature = "faster"))]
    fn test1() {
        let puzzle = Puzzle::new(SAMPLE1);
        assert_eq!(puzzle.paths("you", "out"), Some(5));
    }

    #[test]
    #[cfg(feature = "faster")]
    fn test1_dfs() {
        let puzzle = Puzzle::new(SAMPLE1);
        let memo = RefCell::new(HashMap::new());
        assert_eq!(puzzle.paths("you", "out", &memo), 5);
    }

    #[test]
    #[cfg(feature = "faster")]
    fn test2_dfs() {
        let puzzle = Puzzle::new(SAMPLE2);
        let memo = RefCell::new(HashMap::new());
        assert_eq!(puzzle.paths("svr", "fft", &memo), 1);
        assert_eq!(puzzle.paths("fft", "dac", &memo), 1);
        assert_eq!(puzzle.paths("dac", "out", &memo), 2);
    }

    #[test]
    #[cfg(not(feature = "faster"))]
    fn test2_matrix_multiplication() {
        let puzzle = Puzzle::new(SAMPLE2);

        let mut paths = 0;
        if let (Some(a), Some(b), Some(c)) = (
            puzzle.paths("svr", "fft"),
            puzzle.paths("fft", "dac"),
            puzzle.paths("dac", "out"),
        ) {
            println!("svr -> fft -> dac -> out: {a} * {b} * {c} = {}", a * b * c);
            paths += a * b * c;
        } else {
            eprint!("no path svr -> fft -> dac -> out");
        }

        if let (Some(a), Some(b), Some(c)) = (
            puzzle.paths("svr", "dac"),
            puzzle.paths("dac", "fft"),
            puzzle.paths("fft", "out"),
        ) {
            println!("svr -> dac -> fft -> out: {}", a * b * c);
            paths += a * b * c;
        } else {
            eprintln!("no path svr -> dac -> fft -> out");
        }

        assert_eq!(paths, 2);
        assert_eq!(puzzle.part2(), 2);
    }
}
