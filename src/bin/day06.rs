use advent_of_code_2025::*;
use nalgebra::DMatrix;

const PUZZLE: &str = include_str!("../../puzzles/day06.txt");

/// Ugh. That was tricky. Who knew that vertical text would be so difficult to
/// work with?
///
/// Sometimes you're proud of your fast and elegant AoC solution. Other times
/// you try not to think about the index-array horrors of your ugly code.
/// This is one of those latter times.
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("Part 2: {}", d.part2); // 5129287 too low
    println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Debug)]
pub struct Puzzle {
    pub part1: usize,
    pub part2: usize,
    matrix: DMatrix<usize>,
    vertical_numbers: Vec<Vec<usize>>,
    operators: Vec<String>,
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

        let mut matrix = DMatrix::zeros(rows, cols);

        for (row, line) in input.lines().enumerate() {
            for (col, word) in line.split_whitespace().enumerate() {
                match word.parse::<usize>() {
                    Ok(x) => matrix[(row, col)] = x,
                    Err(_) => operators.push(word.to_owned()),
                }
            }
        }

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

        Self {
            part1: 0,
            part2: 0,
            matrix,
            vertical_numbers,
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
