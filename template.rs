use advent_of_code_2025::*;

const PUZZLE: &str = include_str!("../../puzzles/day02.txt");

fn main() {
    let d = Day01::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    //println!("Part 2: {}", d.part2);
}

#[derive(Debug)]
pub struct Day01 {
    pub part1: usize,
    pub part2: usize,
}

impl Puzzle for Day01 {
    fn new(input: &str) -> Self {
        Self {}
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
        assert_eq!(Day01::new(SAMPLE).solve().part1, todo!());
    }
}
