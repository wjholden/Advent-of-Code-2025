use advent_of_code_2025::*;

const PUZZLE: &str = include_str!("../../puzzles/day02.txt");

fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("Part 2: {}", d.part2);
}

/// ...you can find the invalid IDs by looking for any ID which is made only of
/// some sequence of digits repeated twice. So, 55 (5 twice), 6464 (64 twice),
/// and 123123 (123 twice) would all be invalid IDs.
fn is_valid(i: usize) -> bool {
    let n = i.ilog10() + 1;
    // odd numbers can never be invalid since you can't break them into two sequences.
    if n % 2 == 1 {
        return true;
    }
    let d = 10usize.pow(n / 2);
    let half = i % d;
    i != half + (half * d) // invalid if doubled sequence.
}

/// Now, an ID is invalid if it is made only of some sequence of digits
/// repeated at least twice
fn is_valid2(i: usize) -> bool {
    let n = i.ilog10() + 1;
    for s in 1..=n / 2 {
        // skip anything where we can't make groups of sequences.
        if !n.is_multiple_of(s) {
            continue;
        }
        // ok, we can make m groups of s digits in an n-digit number.
        let m = n / s;
        // each group has d digits
        let d = 10usize.pow(s);
        // the last group is digits "seq"
        let seq = i % d;
        let mut total = 0;
        for _ in 1..=m {
            total = d * total + seq;
        }
        if total == i {
            return false;
        }
    }
    true
}

#[derive(Debug)]
pub struct Puzzle {
    pub part1: usize,
    pub part2: usize,
    pairs: Vec<(usize, usize)>,
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let input = input.replace('\n', "");
        let pairs = input
            .split(',')
            .map(|range| match range.split_once('-') {
                Some((l, r)) => (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()),
                None => panic!(),
            })
            .collect();
        Self {
            part1: 0,
            part2: 0,
            pairs,
        }
    }

    fn solve(mut self) -> Self {
        // part 1
        for (l, r) in self.pairs.iter() {
            for i in *l..=*r {
                if !is_valid(i) {
                    self.part1 += i;
                }
                if !is_valid2(i) {
                    self.part2 += i;
                }
            }
        }

        self
    }
}

#[cfg(test)]
mod day01 {
    use super::*;

    const SAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, 1227775554);
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part2, 4174379265);
    }

    #[test]
    fn invalid() {
        for i in [55, 6464, 123123] {
            assert!(!is_valid(i))
        }
    }

    #[test]
    fn invalid2() {
        for i in [12341234, 123123123, 1212121212, 1111111] {
            assert!(!is_valid2(i))
        }
    }
}
