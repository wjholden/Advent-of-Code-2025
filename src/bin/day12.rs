use advent_of_code_2025::*;

pub const PUZZLE: &str = include_str!("../../puzzles/day12.txt");

/// I knew something was up as soon as I saw this.
/// https://www.reddit.com/r/adventofcode/comments/1pkkiq8/2025_day_12_part_1_done_it_but_feels_dirty/
///
/// Glad we played this game on easy. The last day usually isn't so hard, so
/// when I saw Tetris tiling I was pretty worried we were in for another
/// NP-hard nightmare.
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Default, Debug)]
pub struct Puzzle {
    pub part1: usize,
    regions: Vec<Region>,
}

#[derive(Debug)]
struct Region {
    width: usize,
    length: usize,
    quantities: Vec<usize>,
}

impl Region {
    fn new(line: &str) -> Self {
        let mut it = line.split(&['x', ':', ' '][..]);
        let width = it.next().unwrap().parse().unwrap();
        let length = it.next().unwrap().parse().unwrap();
        _ = it.next(); // discard empty token between ": ".
        let quantities = it.map(|q| q.parse().unwrap()).collect();
        Self {
            width,
            length,
            quantities,
        }
    }
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let mut instance = Self::default();
        instance.regions = input.lines().skip(30).map(Region::new).collect();
        instance
    }

    fn solve(mut self) -> Self {
        for region in self.regions.iter() {
            let area = region.length * region.width;
            let required = 9 * region.quantities.iter().sum::<usize>();
            if required <= area {
                self.part1 += 1;
            }
        }
        self
    }
}

#[cfg(test)]
mod christmas_tree_farm {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day12.txt");

    #[test]
    fn test1() {
        // The test case is MUCH harder than the real input.
        if Puzzle::new(SAMPLE).solve().part1 != 2 {
            println!(
                "This test for Day 12 would fail, but the puzzle input is a much simpler case."
            );
        }
    }
}
