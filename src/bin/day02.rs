use advent_of_code_2025::*;

const PUZZLE: &str = include_str!("../../puzzles/day02.txt");

/// There is a clever solution out there where you test for divisibility
/// with numbers like 1001, but I'm not sure how to make this work for ranges.
/// You might end up with tricky off-by-one errors where patterns like `xyzxyz`
/// overlap `xyxyxy`.
///
/// A four-digit number `xyxy` has repeated groups of digits iff `xyxy = xy * 101`.
/// `x` and `y` may be equal, which handles the case `xxxx`.
///
/// ```
/// 2 digits:  xx -> 11
/// 3 digits:  xxx -> 111
/// 4 digits:  xyxy -> 101
/// 5 digits:  xxxxx -> 11111
/// 6 digits:  xyzxyz -> 1001
///            xyxyxy -> 10101
/// 7 digits:  xxxxxxx -> 1111111
/// 8 digits:  xyxyxyxy -> 1010101
///            xyzwxyzw -> 10001
/// 9 digits:  xyzxyzxyz -> 1001001
/// 10 digits: xyxyxyxyxy -> 101010101
///            xyzwtxyzwt -> 100001
/// ```
///
/// Someone found a closed-form solution:
/// https://www.reddit.com/r/adventofcode/comments/1pcbgai/2025_day_2_day_2_should_be_easy_right_closed/
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("Part 2: {}", d.part2);
    #[cfg(not(feature = "faster"))]
    {
        println!("{:?}", Puzzle::time(PUZZLE));
    }
    #[cfg(feature = "faster")]
    {
        println!("{:?} (faster approach)", Puzzle::time(PUZZLE));
    }
}

/// ...you can find the invalid IDs by looking for any ID which is made only of
/// some sequence of digits repeated twice. So, 55 (5 twice), 6464 (64 twice),
/// and 123123 (123 twice) would all be invalid IDs.
fn is_invalid(i: usize) -> bool {
    let n = i.ilog10() + 1;
    // odd numbers can never be invalid since you can't break them into two sequences.
    if n % 2 == 1 {
        return false;
    }
    let d = 10usize.pow(n / 2);
    let half = i % d;
    i == half + (half * d) // invalid if doubled sequence.
}

/// Now, an ID is invalid if it is made only of some sequence of digits
/// repeated at least twice
#[cfg(not(feature = "faster"))]
fn is_invalid2(i: usize) -> bool {
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
            return true;
        }
    }
    false
}

#[cfg(feature = "faster")]
fn is_invalid2(i: usize) -> bool {
    match 1 + i.ilog10() {
        1 => false,
        2 => i.is_multiple_of(11),
        3 => i.is_multiple_of(111),
        4 => i.is_multiple_of(101),
        5 => i.is_multiple_of(11111),
        6 => i.is_multiple_of(1001) || i.is_multiple_of(10101),
        7 => i.is_multiple_of(1111111),
        8 => i.is_multiple_of(1010101) || i.is_multiple_of(10001),
        9 => i.is_multiple_of(1001001),
        10 => i.is_multiple_of(101010101) || i.is_multiple_of(100001),
        _ => panic!(),
    }
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
        self.part1 = 0;
        self.part2 = 0;
        for (l, r) in self.pairs.iter() {
            for i in *l..=*r {
                if is_invalid(i) {
                    self.part1 += i;
                }
                if is_invalid2(i) {
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
            assert!(is_invalid(i))
        }
    }

    #[test]
    fn invalid2() {
        for i in [12341234, 123123123, 1212121212, 1111111] {
            assert!(is_invalid2(i))
        }
    }
}
