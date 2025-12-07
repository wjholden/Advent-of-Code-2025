use std::panic;

use advent_of_code_2025::*;
use nalgebra::DMatrix;

const PUZZLE: &str = include_str!("../../puzzles/day06.txt");

/// Ugh. That was tricky. Who knew that vertical text would be so difficult to
/// work with?
///
/// Sometimes you're proud of your fast and elegant AoC solution. Other times
/// you try not to think about the index-array horrors of your ugly code.
/// This is one of those latter times.
///
/// Later comment: so I implemented a very clever idea to use matrix
/// multiplication (thank you Reddit), but by my benchmarks it isn't reliably
/// faster. Oh well.
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("Part 2: {}", d.part2); // 5129287 too low
    #[cfg(not(feature = "faster"))]
    {
        println!("{:?}", Puzzle::time(PUZZLE));
    }
    #[cfg(feature = "faster")]
    {
        println!("{:?} (not-actually-faster approach)", Puzzle::time(PUZZLE));
    }
}

#[derive(Debug)]
pub struct Puzzle {
    pub part1: usize,
    pub part2: usize,
    matrix: DMatrix<usize>,
    #[cfg(not(feature = "faster"))]
    vertical_numbers: Vec<Vec<usize>>,
    #[cfg(feature = "faster")]
    z: DMatrix<usize>,
    operators: Vec<String>,
}

#[cfg(not(feature = "faster"))]
fn parse_part2(input: &str, rows: usize, cols: usize) -> Vec<Vec<usize>> {
    // Vertical parsing. I suddenly sympathize for those working with eastern Asian languages.
    let mut start_markers: Vec<usize> = input
        .lines()
        .last()
        .unwrap()
        .char_indices()
        .filter_map(|(i, c)| match c {
            '+' | '*' => Some(i),
            ' ' => None,
            _ => unreachable!(),
        })
        .collect();
    start_markers.push(input.lines().next().unwrap().len() + 1); // for the last token.
    let widths: Vec<(usize, usize)> = start_markers
        .windows(2)
        .filter_map(|w| {
            if let &[start, end] = w {
                Some((start, end - 1))
            } else {
                None
            }
        })
        .collect();
    assert_eq!(widths.len(), cols);
    let lines: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

    let mut vertical_numbers = Vec::new();
    for (start, end) in widths {
        let group_width = end - start;
        let mut vertical_group = vec![0; group_width];
        for c in start..end {
            for r in 0..rows {
                // don't use .chars().nth(), it is really slow.
                match lines[r][c] as char {
                    ' ' => {}
                    d @ '1'..='9' => {
                        vertical_group[c - start] =
                            10 * vertical_group[c - start] + d as usize - '0' as usize;
                    }
                    other => {
                        eprintln!("unexpected {other}");
                        panic!();
                    }
                }
            }
        }
        vertical_numbers.push(vertical_group);
    }
    vertical_numbers
}

#[cfg(feature = "faster")]
fn parse_part2(input: &str, rows: usize) -> DMatrix<usize> {
    let cols_wide = input.lines().next().unwrap().len();
    let mut y = DMatrix::<usize>::zeros(rows, cols_wide);
    for (row, line) in input.lines().take(rows).enumerate() {
        for (col, c) in line.char_indices() {
            y[(row, col)] = match c {
                ' ' => 0,
                d @ '1'..='9' => d as usize - '0' as usize,
                x => {
                    eprintln!("unexpected symbol {x}");
                    panic!();
                }
            }
        }
    }
    // Yeah, I know. It's a bubble sort variant. Whatever, nrows<10.
    for col in 0..y.ncols() {
        for r1 in (0..y.nrows() - 1).rev() {
            for r2 in (1..y.nrows()).rev() {
                if y[(r2, col)] == 0 {
                    y.swap((r1, col), (r2, col));
                }
            }
        }
    }
    let x = DMatrix::from_iterator(
        1,
        rows,
        (0..rows).map(|i| 10usize.pow(rows as u32 - i as u32 - 1)),
    );
    let z = x * y;
    z
}

impl Puzzle {
    #[cfg(not(feature = "faster"))]
    fn part2(mut self) -> Self {
        for (operator, list) in self.operators.iter().zip(self.vertical_numbers.iter()) {
            let x = match operator.as_str() {
                "+" => list.iter().sum::<usize>(),
                "*" => list.iter().product::<usize>(),
                _ => panic!("unexpected operator"),
            };
            self.part2 += x;
        }
        self
    }

    #[cfg(feature = "faster")]
    fn part2(mut self) -> Self {
        let mut it = self.z.iter();
        for operator in self.operators.iter() {
            let x = match operator.as_str() {
                "+" => it.by_ref().take_while(|x| **x > 0).sum::<usize>(),
                "*" => it.by_ref().take_while(|x| **x > 0).product::<usize>(),
                _ => panic!("unexpected operator"),
            };
            self.part2 += x;
        }
        self
    }
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let rows = input.lines().count() - 1;
        let cols = input
            .lines()
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .count();
        let mut operators = Vec::new();

        let mut m1 = DMatrix::zeros(rows, cols);

        for (row, line) in input.lines().enumerate() {
            for (col, word) in line.split_whitespace().enumerate() {
                match word.parse::<usize>() {
                    Ok(x) => m1[(row, col)] = x,
                    Err(_) => operators.push(word.to_owned()),
                }
            }
        }

        Self {
            part1: 0,
            part2: 0,
            matrix: m1,
            #[cfg(not(feature = "faster"))]
            vertical_numbers: parse_part2(input, rows, cols),
            #[cfg(feature = "faster")]
            z: parse_part2(input, rows),
            operators,
        }
    }

    fn solve(mut self) -> Self {
        for (operator, list) in self.operators.iter().zip(self.matrix.column_iter()) {
            let x = match operator.as_str() {
                "+" => list.iter().sum::<usize>(),
                "*" => list.iter().product::<usize>(),
                _ => panic!("unexpected operator"),
            };
            self.part1 += x;
        }

        self = self.part2();

        self
    }
}

#[cfg(test)]
mod day01 {
    use super::*;

    // Zed is trimming trailing whitespaces in my string literal. Never noticed
    // until today, where trailing whitespaces actually matter.
    const SAMPLE: &str = include_str!("../../samples/day06.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, 4277556);
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part2, 3263827);
    }
}
