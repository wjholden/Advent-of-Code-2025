use std::{
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet},
    usize,
};

use advent_of_code_2025::*;

pub const PUZZLE: &str = include_str!("../../puzzles/day09.txt");

/// This was an extremely hard problem for me. Two lessons learned.
/// 1) Coordinate compression is awesome!
/// 2) `BinaryHeap::iter` doesn't do what you expect. If you want the minimum
/// element, you need to use `BinaryHeap::pop`!
///
/// This was a really cool idea and had a chance, but I could never get it to
/// work. https://math.stackexchange.com/a/59820/474318 To determine if a point P
/// is bounded by a polygon with vertices A, B, C, D, E, ..., you'll sum the
/// angles between PA and PB, PB and PC, PC and PD, and so on. The result will
/// be ±2π.
///
/// I came up with another approach based on turns. This one uses an idea from
/// https://stackoverflow.com/a/3461533/5459668 where you determine whether
/// point c is left of line a--b with `(b.x - a.x)*(c.y - a.y) - (b.y - a.y)*(c.x - a.x) > 0`.
/// (Signs might be reversed for a world where +y points downwards.)
///
/// Yet another approach was to guess if points were inside or outside of the
/// polygon based on the "winding number" along the perimeter. I've already
/// forgotten what this means.
///
/// I got close when I decided to flood the space and guess if the rectangle
/// was a subset (https://www.reddit.com/r/adventofcode/comments/1pichj2/comment/nt5guy3/).
/// The problem I ran into is that it was too slow. Coordinate compression to
/// the rescue! You don't need all these huge empty spaces. The distances between
/// points doesn't matter for this problem, only their relative ordering.
///
/// The last thing I had tried, before coordinate compression, was to generate
/// horizontal and vertical lines from the input. I built an intersection
/// function that works, but couldn't reason my way to how you'd know if you
/// were inside or outside of the region. This approach was probably always
/// doomed.
///
/// See also:
/// https://stackoverflow.com/a/16544330/5459668
/// https://stackoverflow.com/questions/14066933/direct-way-of-computing-the-clockwise-angle-between-two-vectors
/// https://math.stackexchange.com/a/59820/474318
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("Part 2: {}", d.part2); // 1069029791 too low
    println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Default, Debug)]
pub struct Puzzle {
    pub part1: usize,
    pub part2: usize,
    pairs: Vec<(usize, usize)>,
    cx: HashMap<usize, usize>,
    cy: HashMap<usize, usize>,
    compressed: Vec<(usize, usize)>,
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let mut instance = Self::default();
        for line in input.lines() {
            match line.split_once(',') {
                Some((x, y)) => instance
                    .pairs
                    .push((x.parse().expect("x"), y.parse().expect("y"))),
                None => {
                    eprint!("bad line {line}");
                }
            }
        }

        // Compress x values.
        let mut all_x: Vec<usize> = instance.pairs.iter().map(|&(x, _)| x).collect();
        all_x.sort();
        all_x.dedup();
        instance.cx = HashMap::from_iter(all_x.into_iter().enumerate().map(|(i, x)| (x, i)));
        // Compress y values.
        let mut all_y: Vec<usize> = instance.pairs.iter().map(|&(_, y)| y).collect();
        all_y.sort();
        all_y.dedup();
        instance.cy = HashMap::from_iter(all_y.into_iter().enumerate().map(|(i, y)| (y, i)));

        instance.compressed = instance
            .pairs
            .iter()
            .map(|(x, y)| {
                (
                    instance.cx.get(&x).unwrap().clone(),
                    instance.cy.get(&y).unwrap().clone(),
                )
            })
            .collect();

        instance
    }

    fn solve(mut self) -> Self {
        let n = self.pairs.len();
        let mut queue = BinaryHeap::with_capacity(n * (n - 1) / 2);
        for i in 0..n - 1 {
            for j in i + 1..n {
                let (x1, y1) = self.pairs[i];
                let (x2, y2) = self.pairs[j];
                let dx = 1 + x1.abs_diff(x2);
                let dy = 1 + y1.abs_diff(y2);
                let area = dx * dy;
                queue.push((area, i, j));
            }
        }

        self.part1 = match queue.peek() {
            Some((area, _, _)) => *area as usize,
            None => panic!("the max area should be at the head of the queue"),
        };

        let mut polygon = Polygon::new(&self.compressed);
        // Flood the whole thing.
        polygon.flood_dfs();
        // We've flooded the entire region, which should be pretty big.
        // Now let's find the solution.
        while let Some((area, i, j)) = queue.pop() {
            let rect = Polygon::new_rectangle(self.compressed[i], self.compressed[j]);
            if rect.is_subset_of_filled(&polygon) {
                self.part2 = area;
                break;
            }
        }
        self
    }
}

#[derive(Debug)]
struct Polygon {
    points: HashSet<(usize, usize)>,
    is_filled: bool,
    min_y: usize,
    max_y: usize,
    min_x: usize,
    max_x: usize,
}

impl Polygon {
    fn new(pairs: &[(usize, usize)]) -> Polygon {
        // Outline of the larger region.
        let mut points = HashSet::new();
        //let mut lines = HashSet::new();
        let n = pairs.len();
        let mut max_y = 0;
        let mut max_x = 0;
        let mut min_y = usize::MAX;
        let mut min_x = usize::MAX;
        for i in 0..n {
            let (x1, y1) = pairs[i];
            let (x2, y2) = pairs[(i + 1) % n];
            max_y = max_y.max(y1);
            max_x = max_x.max(x1);
            min_y = min_y.min(y1);
            min_x = min_x.min(x1);
            for x in x1.min(x2)..=x1.max(x2) {
                for y in y1.min(y2)..=y1.max(y2) {
                    points.insert((x, y));
                }
            }
        }
        Self {
            points,
            is_filled: false,
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    fn new_rectangle(a: (usize, usize), b: (usize, usize)) -> Self {
        let (x1, y1) = a;
        let (x2, y2) = b;
        Polygon::new(&[
            (x1.min(x2), y1.min(y2)),
            (x1.max(x2), y1.min(y2)),
            (x1.max(x2), y1.max(y2)),
            (x1.min(x2), y1.max(y2)),
        ])
    }

    fn find_empty_space(&self) -> Option<(usize, usize)> {
        for y in self.min_y + 1..self.max_y - 1 {
            let mut inside = false;
            for x in self.min_x..self.max_x {
                match (inside, self.points.contains(&(x, y))) {
                    (false, true) => inside = true,
                    (false, false) => (),
                    (true, false) => return Some((x, y)),
                    (true, true) => (),
                }
            }
        }
        None
    }

    fn flood_dfs(&mut self) {
        let start = match self.find_empty_space() {
            Some(start) => start,
            None => return, // Some regions don't have any empty spaces to fill.
        };
        // Great, we can use this as our start point. We know we're inside the
        // polygon, so conduct a depth-first search to fill in the space.
        //
        // Is this really a DFS? We're using a B-Tree because it lets us
        // pop the minimum element easily. We won't insert anything into this
        // data structure again
        let mut stack = BTreeSet::from([start]);
        while let Some(current) = stack.pop_first() {
            if self.points.insert(current) {
                let west = (current.0 - 1, current.1);
                let east = (current.0 + 1, current.1);
                let north = (current.0, current.1 - 1);
                let south = (current.0, current.1 + 1);
                for next in [west, east, north, south] {
                    if !self.points.contains(&next) {
                        stack.insert(next);
                    }
                }
            }
        }
        self.is_filled = true;
    }

    fn is_subset_of_filled(&self, other: &Self) -> bool {
        if !other.is_filled {
            panic!("don't use this against a hollow (not flooded) polygon");
        }
        self.points.is_subset(&other.points)
    }
}

#[cfg(test)]
mod movie_theater {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day09.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, 50);
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part2, 24);
    }
}
