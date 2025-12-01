use advent_of_code_2025::*;

fn main() {
    let d = Day01::new(include_str!("../../puzzles/day01.txt"));
    dbg!(d.part1());
    println!("Part 1: {}", d.part1());
}

#[derive(Debug)]
pub struct Day01 {}

impl Puzzle<u64, u64> for Day01 {
    fn new(_input: &str) -> Self {
        Self {}
    }

    fn part1(&self) -> u64 {
        todo!()
    }

    fn part2(&self) -> u64 {
        todo!()
    }
}

#[cfg(test)]
mod day01 {
    use super::*;

    const SAMPLE: &str = "";

    #[test]
    fn test1() {
        assert_eq!(Day01::new(SAMPLE).part1(), 0);
    }
}
