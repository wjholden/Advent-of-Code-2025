use advent_of_code_2025::*;

const PUZZLE: &str = include_str!("../../puzzles/dayXX.txt");

fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    //println!("Part 2: {}", d.part2);
    println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Debug)]
pub struct Puzzle {
    pub part1: usize,
    pub part2: usize,
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        todo!()
    }

    fn solve(mut self) -> Self {
        self
    }
}

#[cfg(test)]
mod day01 {
    use super::*;

    const SAMPLE: &str = "";

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, todo!());
    }
}
