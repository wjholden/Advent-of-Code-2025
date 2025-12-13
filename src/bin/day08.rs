use advent_of_code_2025::*;
use nalgebra::DMatrix;

pub const PUZZLE: &str = include_str!("../../puzzles/day08.txt");

/// I can't believe this actually runs as fast as it does. <100ms in release.
///
/// Saw some inspiring ideas on Reddit for ways to speed this up. An obvious
/// one I wish I had thought of is that you don't need to actually take the
/// square root of the distance. Another is that you don't need to fill out
/// an entire distance matrix, a priority queue (min-heap) will suffice.
/// I think there was something else that I can't remember. Anyways, I can't
/// quite get it to work for now, but maybe I can come back to this later.
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("Part 2: {}", d.part2);
    println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Default, Debug)]
struct Point(f64, f64, f64);

impl Point {
    /// Classical Euclidean distance.
    fn distance(&self, other: &Self) -> f64 {
        ((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2) + (self.2 - other.2).powi(2))
            .sqrt()
    }
}

#[derive(Debug)]
struct DisjointSet(Vec<usize>);

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self((0..n).collect())
    }

    fn find(&self, e: usize) -> usize {
        let parent = *self.0.get(e).expect("parent index");
        if parent != e {
            self.find(parent)
        } else {
            parent
        }
    }

    fn union(&mut self, i: usize, j: usize) {
        let pi = self.find(i);
        let pj = self.find(j);
        let new = pi.min(pj);
        let old = pi.max(pj);
        for value in self.0.iter_mut() {
            if *value == old {
                *value = new;
            }
        }
    }

    fn tally(&self) -> Vec<usize> {
        let mut totals = vec![0; self.0.len()];
        for i in 0..self.0.len() {
            totals[self.0[i]] += 1;
        }
        totals
    }

    fn part1(&self) -> usize {
        let mut totals: Vec<usize> = self.tally();
        totals.sort();
        totals.into_iter().rev().take(3).product()
    }

    fn all_equal(&self) -> bool {
        let first = self.0[0];
        for &element in self.0[1..].iter() {
            if first != element {
                return false;
            }
        }
        true
    }
}

#[derive(Default, Debug)]
pub struct Puzzle {
    pub part1: usize,
    pub part2: usize,
    points: Vec<Point>,
    distances: DMatrix<f64>,
}

impl Solver for Puzzle {
    fn new(input: &str) -> Self {
        let mut instance = Self::default();
        let input = input.replace("\r\n", "\n");
        let mut it = input.split([',', '\n']);
        // Split doesn't provide a windows() function.
        while let (Some(x), Some(y), Some(z)) = (it.next(), it.next(), it.next()) {
            instance.points.push(Point(
                x.parse::<f64>().expect("x"),
                y.parse::<f64>().expect("y"),
                z.parse::<f64>().expect("z"),
            ));
        }

        let n = instance.points.len();
        // We have the points. Now we need their distances.
        instance.distances = DMatrix::from_fn(n, n, |i, j| {
            if i < j {
                instance.points[i].distance(&instance.points[j])
            } else {
                // By filling the diagonal and lower triangle with infinities,
                // we won't have to deal with duplicated lowest distances.
                f64::INFINITY
            }
        });

        instance
    }

    fn solve(mut self) -> Self {
        let n;
        #[cfg(test)]
        {
            n = 10;
        }
        #[cfg(not(test))]
        {
            n = 1000;
        }

        let mut disjoint_set = DisjointSet::new(self.points.len());

        // Look, I know this is stupid. We'll grab the smallest element
        // from our matrix n times, setting its value once we take it.
        let mut di: Vec<(usize, &f64)> = self.distances.into_iter().enumerate().collect();
        di.sort_by(|&(_, &d1), &(_, &d2)| d1.partial_cmp(&d2).unwrap());
        for (i, _) in di[0..n].iter() {
            let a = i / self.points.len();
            let b = i % self.points.len();
            disjoint_set.union(a, b);
        }

        self.part1 = disjoint_set.part1();

        for (i, _) in di[n..].iter() {
            let a = i / self.points.len();
            let b = i % self.points.len();
            disjoint_set.union(a, b);
            if disjoint_set.all_equal() {
                let x1 = self.points[a].0 as usize;
                let x2 = self.points[b].0 as usize;
                self.part2 = x1 * x2;
                break;
            }
        }

        self
    }
}

#[cfg(test)]
mod playground {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day08.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part1, 40);
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE).solve().part2, 25272);
    }
}
