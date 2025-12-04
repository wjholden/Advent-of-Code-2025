use advent_of_code_2025::*;

const PUZZLE: &str = include_str!("../../puzzles/day03.txt");

/// It's a partially-greedy algorithm! Within a substring of the input, you
/// know to take the left-most copy of the largest digit, but you don't know
/// where that substring begins until you've run this for the previous term.
///
/// Here's a nice visualization of a clever approach:
/// https://www.reddit.com/r/adventofcode/comments/1pdc396/2025_day_3_part_2_python_terminal_visualization/
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1); // 16874 wrong
    println!("Part 2: {}", d.part2);
}

#[derive(Debug)]
pub struct Puzzle {
    input: String,
    pub part1: usize,
    pub part2: usize,
}

fn part1(line: &str) -> usize {
    // This has to be done in two passes. We first need to figure out
    // what the first letter (c1) is. Then we can find the largest
    // letter in the right partition.
    let n = line.len();

    // Reverse because we need the left-most position. max_by was
    // giving the right-most position when there are copies of
    // the thing.
    let (m, c1) = line[..n - 1]
        .char_indices()
        .rev()
        .max_by(|&(_, c1), &(_, c2)| c1.cmp(&c2))
        .unwrap();

    let c2 = line[m + 1..].chars().max().unwrap();

    10 * (c1 as usize - '0' as usize) + (c2 as usize - '0' as usize)
}

#[cfg(not(feature = "up_the_ante"))]
fn part2(line: &str) -> usize {
    let mut start = 0;
    let mut d = 12;
    let n = line.len();
    let mut s12 = 0;
    while d > 0 {
        let mut v = '0';
        let mut new_start = 0;
        for (p, c) in line[start..n - d + 1].char_indices() {
            if c > v {
                new_start = p;
                v = c;
            }
        }
        d -= 1;
        s12 = s12 * 10 + (v as usize - '0' as usize);
        start += new_start + 1;
    }
    s12
}

#[cfg(feature = "up_the_ante")]
fn part2(batteries: &str) -> usize {
    dp(batteries, 12)
}

#[cfg(feature = "up_the_ante")]
/// Doesn't quite work. See case `234234234234278` -- this solution fails to
/// skip over the second 2. Passes some cases but not all.
fn dp(batteries: &str, count: usize) -> usize {
    use std::collections::HashMap;

    let mut x = HashMap::<(usize, usize), usize>::new();
    for (j, c) in batteries.char_indices() {
        x.insert((0, j + 1), c as usize - '0' as usize);
    }
    'outer: for j in 1..=batteries.len() {
        for i in 1..count {
            if i > j {
                continue 'outer;
            }
            let yes_take = 10 * *x.entry((i - 1, j - 1)).or_default() + x.get(&(0, j)).unwrap();
            let not_take = *x.entry((i, j - 1)).or_default();
            x.insert((i, j), yes_take.max(not_take));
        }
    }
    *x.get(&(count - 1, batteries.len())).unwrap()
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        Self {
            input: input.to_owned(),
            part1: 0,
            part2: 0,
        }
    }

    fn solve(mut self) -> Self {
        for line in self.input.lines() {
            self.part1 += part1(line);
            self.part2 += part2(line);
        }
        self
    }
}

#[cfg(test)]
mod day01 {
    use super::*;

    const SAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, 357);
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part2, 3121910778619);
    }

    #[test]
    fn joltage() {
        assert_eq!(part1("12345"), 45);
        assert_eq!(part1("54321"), 54);
        assert_eq!(part1("55555"), 55);
        assert_eq!(part1("52225"), 55);
        assert_eq!(part1("56222225"), 65);
        assert_eq!(part1("11198"), 98);
        assert_eq!(part1("11189"), 89);
        assert_eq!(part1("111918"), 98);
        assert_eq!(part1("111819"), 89);
        assert_eq!(part1("98111989"), 99);
        assert_eq!(part1("111115411111"), 54);
        assert_eq!(part1("1111145411111"), 54);
        assert_eq!(part1("123123"), 33);
        assert_eq!(part1("321321321"), 33); // this was my bug
    }

    #[test]
    #[cfg(feature = "up_the_ante")]
    fn up_the_ante() {
        assert_eq!(dp("987654321111111", 12), 987654321111);
        assert_eq!(dp("811111111111119", 12), 811111111119);
        assert_eq!(dp("234234234234278", 12), 434234234278);
    }
}
