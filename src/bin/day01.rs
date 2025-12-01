use advent_of_code_2025::*;

const PUZZLE: &str = include_str!("../../puzzles/day01.txt");

/// Trickier than it looks. I resorted to a naive algorithm that
/// actually counts each individual click. Had initially tried
/// to be clever with modulo arithmetic but it didn't work out.
fn main() {
    let mut d = Day01::new(PUZZLE);
    d.solve();
    println!("Part 1: {}", d.part1());
    println!("Part 2: {}", d.part2()); // 6474 too high, 6384 too high, 5958 wrong.
}

#[derive(Debug)]
pub struct Day01 {
    position: i64,
    turns: Vec<i64>,
    part1: usize,
    part2: usize,
}

impl Puzzle<usize, usize> for Day01 {
    fn new(input: &str) -> Self {
        let turns = input
            .lines()
            .map(|line| {
                let direction = line.chars().next().unwrap();
                let count = line[1..].parse::<i64>().unwrap();
                match direction {
                    'L' => -count,
                    'R' => count,
                    _ => panic!(),
                }
            })
            .collect();
        Self {
            position: 50,
            turns,
            part1: 0,
            part2: 0,
        }
    }

    fn part1(&self) -> usize {
        self.part1
    }

    fn part2(&self) -> usize {
        self.part2
    }

    fn solve(&mut self) {
        for turn in self.turns.iter() {
            for _ in 1..=turn.abs() {
                if turn.is_positive() {
                    self.position += 1;
                } else {
                    self.position -= 1;
                }
                if self.position == 100 {
                    self.position = 0;
                }
                if self.position == -1 {
                    self.position = 99;
                }
                if self.position == 0 {
                    self.part2 += 1;
                }
            }
            if self.position == 0 {
                self.part1 += 1;
            }
            assert!(0 <= self.position && self.position <= 99);
            println!("Rotate {turn} to {}", self.position);
        }
    }
}

#[cfg(test)]
mod day01 {
    use super::*;

    const SAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test1() {
        let mut d = Day01::new(SAMPLE);
        d.solve();
        assert_eq!(d.part1(), 3);
    }

    #[test]
    fn test2() {
        let mut d = Day01::new(SAMPLE);
        d.solve();
        assert_eq!(d.part2(), 6);
    }

    #[test]
    fn test3() {
        let mut d = Day01::new("R1000");
        d.solve();
        assert_eq!(d.part2(), 10);
    }
}
