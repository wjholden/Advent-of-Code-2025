use advent_of_code_2025::*;

const PUZZLE: &str = include_str!("../../puzzles/day01.txt");

/// Trickier than it looks. I resorted to a naive algorithm that
/// simply counts each individual click. Had initially tried
/// to be clever with modulo arithmetic but it didn't work out.
///
/// nb: `rem_euclid`, `isize`
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("Part 2: {}", d.part2);
    println!("{:?}", Puzzle::new(PUZZLE).time());
}

#[derive(Debug)]
pub struct Puzzle {
    position: isize,
    turns: Vec<isize>,
    pub part1: usize,
    pub part2: usize,
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let turns = input
            .lines()
            .map(|line| {
                let direction = line.chars().next().unwrap();
                let count = line[1..].parse::<isize>().unwrap();
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

    fn solve(mut self) -> Self {
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
            debug_assert!(0 <= self.position && self.position <= 99);
        }
        self
    }
}

impl Puzzle {
    // todo: passes tests but gets wrong answer for part 2.
    #[allow(dead_code)]
    fn solve_faster(mut self) -> Self {
        for turn in self.turns.iter() {
            self.position = self.position + turn;
            if self.position < 0 || self.position > 99 {
                self.part2 += 1 + (turn.abs() / 100) as usize;
            }
            self.position = self.position.rem_euclid(100);
            if self.position == 0 {
                self.part1 += 1;
            }
        }
        self
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
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part2, 6);
    }

    #[test]
    fn test3() {
        assert_eq!(Puzzle::new("R1000").solve().part2, 10);
    }

    #[test]
    fn faster1() {
        assert_eq!(Puzzle::new(SAMPLE).solve_faster().part1, 3);
    }

    #[test]
    fn faster2() {
        assert_eq!(Puzzle::new(SAMPLE).solve_faster().part2, 6);
    }
}
