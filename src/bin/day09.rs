use std::{collections::BinaryHeap, f64};

use advent_of_code_2025::*;
use rand::Rng;

pub const PUZZLE: &str = include_str!("../../puzzles/day09.txt");

fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("Part 2: {}", d.part2);
    //println!("{:?}", Puzzle::time(PUZZLE));
}

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

impl Turn {
    /// I don't completely understand this but here goes...
    /// https://stackoverflow.com/a/3461533/5459668
    ///
    /// Note that this looks reversed because positive `y` is down.
    fn direction(a: (isize, isize), b: (isize, isize), c: (isize, isize)) -> Self {
        // Using cross product this time. I've forgotten so much math...
        let (ax, ay) = a;
        let (bx, by) = b;
        let (cx, cy) = c;

        if (bx - ax) * (cy - ay) - (by - ay) * (cx - ax) > 0 {
            Turn::Right
        } else {
            Turn::Left
        }
    }
}

#[derive(Default, Debug)]
pub struct Puzzle {
    pub part1: usize,
    pub part2: usize,
    pairs: Vec<(i64, i64)>,
    turns: Vec<Turn>,
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

        // let n = instance.pairs.len();
        // for j in 0..n {
        //     let i = (j + n - 1) % n;
        //     let k = (j + 1) % n;

        //     instance.turns.push(Turn::direction(
        //         instance.pairs[i],
        //         instance.pairs[j],
        //         instance.pairs[k],
        //     ));
        // }

        // for ((i, j), turn) in instance.pairs.iter().zip(&instance.turns) {
        //     println!("{:?}: {:?}", (i, j), turn);
        // }

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
                //println!("{:?}", (area, (x1, y1), (x2, y2)));
                queue.push((area, i, j));
            }
        }

        self.part1 = match queue.peek() {
            Some((area, _, _)) => *area as usize,
            None => panic!("the max area should be at the head of the queue"),
        };

        // for &(x1, y1) in self.pairs.iter() {
        //     for &(x2, y2) in self.pairs.iter() {
        //         let dx = 1 + x1.abs_diff(x2);
        //         let dy = 1 + y1.abs_diff(y2);
        //         let area = dx * dy;
        //         println!("({x1},{y1}), ({x2},{y2}), [{dx} x {dy}], distance: {area}");
        //         self.part1 = self.part1.max(area);
        //     }
        // }

        // Now for part 2. We've kept the areas, but we need to check if there
        // is a corner inside them. So, we just loop through the queue and take
        // the first one that has nothing inside the area.
        //'outer: for (area, i, j) in queue {
        // An algorithm based on turns. The input quietly contains an
        // invariant that we might not have expected: the points are given
        // in order! Starting from i, we can walk to j and see if we have
        // to take any left turns.
        // for k in 0..(j - i - 2) {
        //     let (x1, y1) = self.pairs[i + k];
        //     let (x2, y2) = self.pairs[i + k + 1];
        //     let (x3, y3) = self.pairs[i + k + 2];
        //     let (x1, y1) = (x1 as isize, y1 as isize);
        //     let x2 = x2 as isize;
        //     let (x3, y3) = (x3 as isize, y3 as isize);
        //     let (opposite, adjacent) = if x1 == x2 {
        //         (x3 - x1, y3 - y1)
        //     } else {
        //         (y3 - y1, x3 - x1)
        //     };
        //     assert_ne!(adjacent, 0);
        //     let tangent = opposite as f64 / adjacent as f64;
        //     println!(
        //         "({x1},{y1}), ({x2},{y2}), ({x3},{y3}): ({opposite}/{adjacent})={tangent}"
        //     );

        //     match tangent.partial_cmp(&0.0).unwrap() {
        //         std::cmp::Ordering::Less => {
        //             // right turn: ok
        //         }
        //         std::cmp::Ordering::Equal => unreachable!("we should not be going straight"),
        //         std::cmp::Ordering::Greater => {
        //             // we've turned left before reaching the other corner,
        //             // which means there is an empty space in the region.
        //             continue 'outer;
        //         }
        //     }

        //}

        // 'inner: for &(x3, y3) in self.pairs.iter() {
        //     if (x1, y1) == (x3, y3) || (x2, y2) == (x3, y3) {
        //         continue 'inner;
        //     }
        //     if ((x1 < x3 && x3 < x2) || (x1 > x3 && x3 > x2))
        //         && ((y1 < y3 && y3 < y2) || (y1 > y3 && y3 > y2))
        //     {
        //         println!("({x3},{y3}) is inside area ({x1},{y1}) and ({x2},{y2})");
        //         continue 'outer;
        //     }
        // }
        // If we made it here, then we found the solution and can stop.
        //println!("Solution at ({x1},{y1}) and ({x2},{y2})");
        // println!("Solution at {:?} and {:?}", self.pairs[i], self.pairs[j]);
        // self.part2 = area;
        // break;
        //}

        // Third time's the charm.
        //
        // We iterate over the queue. For each candidate area, we're going to
        // check each point in the region. The area is a solution if there is
        // no left turn inside of it.
        // 'outer: for (area, i, j) in queue {
        //     let (x1, y1) = self.pairs[i];
        //     let (x2, y2) = self.pairs[j];
        //     'inner: for k in 0..n {
        //         let (x3, y3) = self.pairs[k];
        //         match (x1.cmp(&x3), y1.cmp(&x3), x2.cmp(&x3), y2.cmp(&y3)) {
        //             (Ordering::Equal, Ordering::Equal, _, _)
        //             | (_, _, Ordering::Equal, Ordering::Equal) => {
        //                 // The current point is one of our two corners.
        //                 continue 'inner;
        //             }
        //             (Ordering::Less, _, Ordering::Less, _)
        //             | (Ordering::Greater, _, Ordering::Greater, _)
        //             | (_, Ordering::Less, _, Ordering::Less)
        //             | (_, Ordering::Greater, _, Ordering::Greater) => {
        //                 // The point is fully outside of our area.
        //                 continue 'inner;
        //             }
        //             (Ordering::Greater, Ordering::Greater, Ordering::Less, Ordering::Less)
        //             | (Ordering::Greater, Ordering::Less, Ordering::Less, Ordering::Greater)
        //             | (Ordering::Less, Ordering::Greater, Ordering::Greater, Ordering::Less)
        //             | (Ordering::Less, Ordering::Less, Ordering::Greater, Ordering::Greater) => {
        //                 // The point is fully inside of our area.
        //                 continue 'outer;
        //             }
        //             (Ordering::Equal, _, _, Ordering::Equal)
        //             | (_, Ordering::Equal, Ordering::Equal, _) => {
        //                 // The point is one of the other two corners of the area.
        //                 // This might be a solution. If it's a right turn, then yes.
        //                 match self.turns[k] {
        //                     Turn::Left => continue 'outer,
        //                     Turn::Right => continue 'inner,
        //                 }
        //             }
        //             (Ordering::Equal, _, Ordering::Equal, _)
        //             | (_, Ordering::Equal, _, Ordering::Equal) => {
        //                 // I don't know that this is completely safe, but I'm
        //                 // pretty confident we can discard skinny solutions where
        //                 // x1=x2 or y1=y2.
        //                 continue 'outer;
        //             }
        //             (Ordering::Less, Ordering::Less, Ordering::Equal, Ordering::Greater) => todo!(),
        //             (Ordering::Less, Ordering::Less, Ordering::Greater, Ordering::Equal) => todo!(),
        //             (Ordering::Less, Ordering::Equal, Ordering::Greater, Ordering::Less) => todo!(),
        //             (Ordering::Less, Ordering::Equal, Ordering::Greater, Ordering::Greater) => todo!(),
        //             (Ordering::Less, Ordering::Greater, Ordering::Equal, Ordering::Less) => todo!(),
        //             (Ordering::Less, Ordering::Greater, Ordering::Greater, Ordering::Equal) => todo!(),
        //             (Ordering::Equal, Ordering::Less, Ordering::Less, Ordering::Greater) => todo!(),
        //             (Ordering::Equal, Ordering::Less, Ordering::Greater, Ordering::Greater) => todo!(),
        //             (Ordering::Equal, Ordering::Greater, Ordering::Less, Ordering::Less) => todo!(),
        //             (Ordering::Equal, Ordering::Greater, Ordering::Greater, Ordering::Less) => todo!(),
        //             (Ordering::Greater, Ordering::Less, Ordering::Less, Ordering::Equal) => todo!(),
        //             (Ordering::Greater, Ordering::Less, Ordering::Equal, Ordering::Greater) => todo!(),
        //             (Ordering::Greater, Ordering::Equal, Ordering::Less, Ordering::Less) => todo!(),
        //             (Ordering::Greater, Ordering::Equal, Ordering::Less, Ordering::Greater) => todo!(),
        //             (Ordering::Greater, Ordering::Greater, Ordering::Less, Ordering::Equal) => todo!(),
        //             (Ordering::Greater, Ordering::Greater, Ordering::Equal, Ordering::Less) => todo!(),
        //         };
        //     }
        //     // If we made it here then we've found the solution.
        //     println!("Solution at ({x1},{y1}) and ({x2},{y2}) with area {area}.");
        //     self.part2 = area;
        //     break;
        // }

        // 'outer: for (area, i, j) in queue {
        //     'inner: for k in 0..n {
        //         let (x1, y1) = self.pairs[i];
        //         let (x2, y2) = self.pairs[j];
        //         let (x3, y3) = self.pairs[k];
        //         if i == k || j == k {
        //             // same point as one of our area corners.
        //             continue 'inner;
        //         } else if (x1 == x3 && y2 == x3) || (x2 == x3 && y1 == x3) {
        //             // corner of same area.
        //             continue 'inner;
        //         } else if (x1 < x3 && x2 < x3)
        //             || (x1 > x3 && x2 > x3)
        //             || (y1 < y3 && y2 < y3)
        //             || (y1 > y3 && y2 > y3)
        //         {
        //             // fully outside.
        //             continue 'inner;
        //         } else if ((x1 < x3 && x3 < x2) || (x1 > x3 && x3 > x2))
        //             && ((y1 < y3 && y3 < y2) || (y1 > y3 && y3 > y2))
        //         {
        //             // fully inside.
        //             continue 'outer;
        //         } else {
        //             // we must be on a corner
        //         }
        //     }
        // }

        // Attempt #5. Walk from i to j. If any point is to the right, then
        // some line must pass through the diagonal of our rectangle.
        // 'outer: for (area, i, j) in queue {
        //     let a = self.pairs[i];
        //     let b = self.pairs[j];
        //     // Maybe a two-step approach? We first walk from i to j.
        //     // If we realize a point is "right" of our midline,
        //     // stop.
        //     for k in i + 1..j - 1 {
        //         let c = self.pairs[k];
        //         match Turn::direction(a, b, c) {
        //             Turn::Left => {}
        //             Turn::Right => {
        //                 println!("{c:?} falls right of {a:?} and {b:?}");
        //                 continue 'outer;
        //             }
        //         }
        //     }
        // Ok, that side was convex, but how about the other side?
        //}

        // Starting to get discouraged, but here's another idea. Build our
        // candidate solutions one thing at a time. Add them to the queue so
        // long as the angle from the starting vertex to the other is decreasing.
        // let mut queue = BinaryHeap::with_capacity(n * (n - 1) / 2);
        // 'outer: for i in 0..n - 1 {
        //     let mut angle = f64::INFINITY;
        //     for j in i + 1..n {
        //         let (x1, y1) = self.pairs[i];
        //         let (x2, y2) = self.pairs[j];
        //         let dx = x1 - x2;
        //         let dy = y1 - y2;
        //         let new_angle = (dy as f64).abs().atan2((dx as f64).abs());
        //         if new_angle < angle {
        //             let area = (1 + dx.abs()) * (1 + dy.abs());
        //             queue.push(area);
        //             angle = new_angle;
        //             println!(
        //                 "ok {:?} and {:?} (area = {area}, angle = {angle})",
        //                 (x1, y1),
        //                 (x2, y2)
        //             );
        //         } else {
        //             println!("stopped comparing {:?} at {:?}", (x1, y1), (x2, y2));
        //             continue 'outer;
        //         }
        //     }
        // }

        let mut rng = rand::rng();
        // Starting to get discouraged. Let's try a stochastic approach.
        // We'll take rectangles from our queue. We'll randomly generate
        // points inside each rectangle. If all of them fit inside the larger
        // region, then we'll accept it as a solution.
        'outer: for (area, i, j) in queue {
            let (x1, y1) = self.pairs[i];
            let (x2, y2) = self.pairs[j];
            println!("*** ({x1},{y1}) and ({x2},{y2}) ***");
            let xmin = x1.min(x2);
            let xmax = x1.max(x2);
            let ymin = y1.min(y2);
            let ymax = y1.max(y2);
            for _ in 1..=100 {
                let x = rng.random_range(xmin..=xmax);
                let y = rng.random_range(ymin..=ymax);

                if self.pairs.contains(&(x, y)) {
                    println!("skip ({x},{y})");
                    continue;
                }

                // https://math.stackexchange.com/a/59820/474318
                let mut angle = 0.0;
                for k in 0..n {
                    let l = (k + 1) % n;
                    let a = self.pairs[k];
                    let b = self.pairs[l];
                    let angle1 = inner_angle1((x, y), a, b);
                    let angle2 = inner_angle2((x, y), a, b);
                    //assert_eq!(angle1, angle2);
                    angle += angle2;
                }
                if (angle - 2.0 * f64::consts::PI).abs() < 0.001 {
                    println!("({x},{y}) is inside the region (angle = {angle}).",);
                } else {
                    println!("({x},{y}) is OUTSIDE of the region (angle = {angle}).",);
                    continue 'outer;
                }
            }
            // If we made it to here then we found our solution.
            println!(
                "Accepting {:?} and {:?} with area {area}.",
                self.pairs[i], self.pairs[j]
            );
            self.part2 = area as usize;
            break;
        }

        self
    }
}

fn inner_angle1(from: (i64, i64), a: (i64, i64), b: (i64, i64)) -> f64 {
    // https://stackoverflow.com/a/16544330/5459668
    // https://stackoverflow.com/questions/14066933/direct-way-of-computing-the-clockwise-angle-between-two-vectors
    let x1 = a.0 - from.0;
    let y1 = a.1 - from.1;
    let x2 = b.0 - from.0;
    let y2 = b.1 - from.1;
    let dot = x1 * x2 + y1 * y2;
    let det = x1 * y2 - y1 * x2;
    (dot as f64).atan2(det as f64)
}

fn inner_angle2(from: (i64, i64), a: (i64, i64), b: (i64, i64)) -> f64 {
    let x1 = a.0 - from.0;
    let y1 = a.1 - from.1;
    let x2 = b.0 - from.0;
    let y2 = b.1 - from.1;
    let dot = x1 * x2 + y1 * y2;
    let len1 = ((x1.pow(2) + y1.pow(2)) as f64).sqrt();
    let len2 = ((x2.pow(2) + y2.pow(2)) as f64).sqrt();
    ((dot as f64) / (len1 * len2)).acos()
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
