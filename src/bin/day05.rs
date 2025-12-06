use std::ops::RangeInclusive;

use advent_of_code_2025::*;

const PUZZLE: &str = include_str!("../../puzzles/day05.txt");

/// Tricky puzzle! My bug was not noticing that a range might have a length of
/// zero but not at 0..=0.
///
/// Constructing `Range` objects turned out to be less great than I expected.
/// My approach was to construct the ranges and then sort them by start and end.
/// Now having a monotonically increasing list of ranges, we pass through them
/// quickly, merging if the start of the right range begins before the end of
/// the left range. The merge operation sets the left range to 0..=0 and the
/// right to the left start and the greater of the ends.
///
/// You know all that safety you get in Rust from not using sentinel values?
/// Yeah...0..=0 is a sentinel value now, which means if the input contained
/// a real range 0..=0 (it doesn't), then we don't handle that case right.
///
/// Using 0..=0 as a sentinel value was supposed to make it easy to know if we
/// should include or exclude a range in our part2 sum. Easy, except what if
/// the range is x..=x? So, we check if x=0 and, it not, push 1 to part2.
///
/// So in the updated version you're reading here, we use Optional instead of
/// the sentinel value. Our program now handles ranges that include zero.
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("Part 2: {}", d.part2);
    println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Debug)]
pub struct Puzzle {
    pub part1: usize,
    pub part2: usize,
    ranges: Vec<RangeInclusive<usize>>,
    ingredients: Vec<usize>,
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let mut ranges: Vec<RangeInclusive<usize>> = input
            .lines()
            .into_iter()
            .take_while(|line| !line.is_empty())
            .filter_map(|line| {
                if let Some((l, r)) = line.split_once("-") {
                    let l = l.parse::<usize>().expect("left side of range");
                    let r = r.parse::<usize>().expect("right side of range");
                    assert!(l <= r);
                    Some(l..=r)
                } else {
                    None
                }
            })
            .collect();
        ranges.sort_by(|r1, r2| {
            r1.start()
                .cmp(&r2.start())
                .then_with(|| r1.end().cmp(&r2.end()))
        });
        let ingredients = input
            .lines()
            .skip(ranges.len() + 1)
            .into_iter()
            .map(|line| line.parse::<usize>().expect("numbered ingredient"))
            .collect();
        Self {
            part1: 0,
            part2: 0,
            ranges,
            ingredients,
        }
    }

    fn solve(mut self) -> Self {
        'outer: for ingredient in &self.ingredients {
            for range in &self.ranges {
                if range.contains(&ingredient) {
                    self.part1 += 1;
                    continue 'outer;
                }
            }
        }

        let n = self.ranges.len();
        let mut ranges: Vec<Option<RangeInclusive<usize>>> =
            self.ranges.clone().into_iter().map(|r| Some(r)).collect();
        for i in 0..n - 1 {
            if let (Some(r1), Some(r2)) = (ranges[i].clone(), ranges[i + 1].clone())
                && r2.start() <= r1.end()
            {
                let start1 = *r1.start();
                let end1 = *r1.end();
                let end2 = *r2.end();
                ranges[i] = None;
                ranges[i + 1] = Some(start1..=(end1.max(end2)));
            }
        }

        self.part2 = ranges.into_iter().fold(0, |acc, r| match r {
            Some(r) => acc + 1 + r.end() - r.start(),
            None => acc,
        });
        self
    }
}

#[cfg(test)]
mod day01 {
    use super::*;

    const SAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    const MORE: &str = "11-50
31-70
41-45
111-130
130-140
121-180
201-230
211-220
215-225
215-225
216-216
33-33
201-202
1000-1000
0-0

162";

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part2, 14);
    }

    #[test]
    fn more() {
        assert_eq!(Puzzle::new(MORE).solve().part2, 162);
    }
}
