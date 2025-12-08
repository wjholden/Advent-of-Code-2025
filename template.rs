use advent_of_code_2025::*;

pub const PUZZLE: &str = include_str!("../../puzzles/dayXX.txt");

fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    //println!("Part 2: {}", d.part2);
    println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Default, Debug)]
pub struct Puzzle {
    pub part1: usize,
    pub part2: usize,
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let mut instance = Self::default();

        instance
    }

    fn solve(mut self) -> Self {
        self
    }
}

#[cfg(test)]
mod puzzle_name {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/dayXX.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, todo!());
    }
}
