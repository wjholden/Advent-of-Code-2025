use std::{
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet},
    usize,
};

use advent_of_code_2025::*;

pub const PUZZLE: &str = include_str!("../../puzzles/day09.txt");

/// Extremely helpful: https://www.reddit.com/r/adventofcode/comments/1pichj2/comment/nt5guy3/
fn main() {
    let d = Puzzle::new(PUZZLE);
    let d = d.solve();
    println!("Part 1: {}", d.part1);
    println!("Part 2: {}", d.part2); // 1069029791 too low
    //println!("{:?}", Puzzle::time(PUZZLE));
}

// #[derive(Debug)]
// enum _Turn {
//     Left,
//     Right,
// }

// impl _Turn {
//     /// I don't completely understand this but here goes...
//     /// https://stackoverflow.com/a/3461533/5459668
//     ///
//     /// Note that this looks reversed because positive `y` is down.
//     fn _direction(a: (isize, isize), b: (isize, isize), c: (isize, isize)) -> Self {
//         // Using cross product this time. I've forgotten so much math...
//         let (ax, ay) = a;
//         let (bx, by) = b;
//         let (cx, cy) = c;

//         if (bx - ax) * (cy - ay) - (by - ay) * (cx - ax) > 0 {
//             _Turn::Right
//         } else {
//             _Turn::Left
//         }
//     }
// }

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
                let area = (dx as usize) * (dy as usize);
                queue.push((area, i, j));
            }
        }

        self.part1 = match queue.peek() {
            Some((area, _, _)) => *area as usize,
            None => panic!("the max area should be at the head of the queue"),
        };

        // Ok, it's been a few days and I need a new approach. We've tried
        // tracking corners to guess if the current rectangle contained any
        // points and that didn't really work. We also tried randomly sampling
        // points inside the region and trying to guess if those were inside
        // the region based on a "winding number" based on the ordered points
        // along the perimeter. I even attempted to iterate over each point of
        // the region in search of corners, but this also never worked. So,
        // now we're desparate on the 11th of December and I'm going to try
        // something so stupid it should work, though slowly: flooding the
        // region. Here goes.

        let mut polygon = Polygon::new(&self.compressed);
        println!("built the polygon outline");
        println!("{:?}", polygon.points);
        // Flood the whole thing.
        polygon.flood_dfs();
        println!("we finished flooding!");
        println!("{:?}", polygon.points);
        // We've flooded the entire region, which should be pretty big.
        // Now let's find the solution.
        for &(area, i, j) in queue.iter() {
            let rect = Polygon::new_rectangle(self.compressed[i], self.compressed[j]);
            //rect.flood_dfs();
            //println!("Filled rectangle: {:?}", rect.points);
            if rect.is_subset_of_filled(&polygon) {
                //println!("{attempts}: {:?}", rect.points);
                self.part2 = area;
                break;
            }
        }

        // // Flooding the region is taking too much time and memory. How about
        // // we walk the four sides of our rectangle and see if any point is
        // // outside of the region?
        // let polygon = Polygon::new(&self.pairs);
        // for (area, i, j) in queue {
        //     let (x1, y1) = self.pairs[i];
        //     let (x2, y2) = self.pairs[j];
        //     if x1.abs_diff(x2) < 2 || y1.abs_diff(y2) < 2 {
        //         println!("skip skinny rectangle {:?} and {:?}", (x1, y1), (x2, y2));
        //     }
        //     let rect = Polygon::new(&[
        //         (x1.min(x2) + 1, y1.min(y2) + 1),
        //         (x1.min(x2) + 1, y1.max(y2) - 1),
        //         (x1.max(x2) - 1, y1.max(y2) - 1),
        //         (x1.max(x2) - 1, y1.min(y2) + 1),
        //     ]);
        //     if rect.is_inside_of_hollow(&polygon) {
        //         self.part2 = area as usize;
        //         return self;
        //     } else {
        //         println!(
        //             "rectangle {:?} and {:?} with area {area} is not inside the region",
        //             (x1, y1),
        //             (x2, y2)
        //         );
        //     }
        // }

        // Omg so frustrated with this puzzle. This must be like my 7th attempt
        // at this.
        //
        // Now we're building polygons of horizontal or vertical line segments.
        // We construct each rectangle as we go through the queue and test its
        // intersections with the region.
        //
        // ...not sure where to go from there, but it's something.
        // let polygon = Polygon::new(&self.pairs);
        // for (area, i, j) in queue {
        //     let a = self.pairs[i];
        //     let b = self.pairs[j];
        //     if a.0 == b.0 || a.1 == b.1 {
        //         // skip skinny rectangles.
        //         continue;
        //     }
        //     let rect = Polygon::new_rectangle(a, b);
        //     println!(
        //         "intersection at {a:?}x{b:?}: {:?}",
        //         rect.intersections(&polygon)
        //     );
        //     println!(
        //         "intersection at {a:?}x{b:?}: {:?}",
        //         polygon.intersections(&rect)
        //     );
        // }

        self
    }
}

// fn _inner_angle1(from: (isize, isize), a: (isize, isize), b: (isize, isize)) -> f64 {
//     // https://stackoverflow.com/a/16544330/5459668
//     // https://stackoverflow.com/questions/14066933/direct-way-of-computing-the-clockwise-angle-between-two-vectors
//     let x1 = a.0 - from.0;
//     let y1 = a.1 - from.1;
//     let x2 = b.0 - from.0;
//     let y2 = b.1 - from.1;
//     let dot = x1 * x2 + y1 * y2;
//     let det = x1 * y2 - y1 * x2;
//     (dot as f64).atan2(det as f64)
// }

// fn _inner_angle2(from: (isize, isize), a: (isize, isize), b: (isize, isize)) -> f64 {
//     let x1 = a.0 - from.0;
//     let y1 = a.1 - from.1;
//     let x2 = b.0 - from.0;
//     let y2 = b.1 - from.1;
//     let dot = x1 * x2 + y1 * y2;
//     let len1 = ((x1.pow(2) + y1.pow(2)) as f64).sqrt();
//     let len2 = ((x2.pow(2) + y2.pow(2)) as f64).sqrt();
//     ((dot as f64) / (len1 * len2)).acos()
// }

// /// https://math.stackexchange.com/a/59820/474318
// ///
// /// The idea was to sample some points inside the largest rectangle in our
// /// queue. If all sampled points were inside the rectangle, then it must
// /// have been the solution.
// ///
// /// Didn't work out in practice.
// fn _is_inside(from: (isize, isize), pairs: Vec<(isize, isize)>) -> bool {
//     let (x, y) = from;
//     let mut angle = 0.0;
//     let n = pairs.len();
//     for k in 0..n {
//         let l = (k + 1) % n;
//         let a = pairs[k];
//         let b = pairs[l];
//         //let angle1 = _inner_angle1((x, y), a, b);
//         let angle2 = _inner_angle2((x, y), a, b);
//         //assert_eq!(angle1, angle2);
//         angle += angle2;
//     }
//     if (angle - 2.0 * f64::consts::PI).abs() < 0.001 {
//         println!("({x},{y}) is inside the region (angle = {angle}).",);
//         true
//     } else {
//         println!("({x},{y}) is OUTSIDE of the region (angle = {angle}).",);
//         false
//     }
// }

// /// Tricky, maybe wrong. We're at a 90° turn. Take the signs of the directions
// /// towards the destination and midpoint. Add one unit in the direction you're
// /// going and take away one unit in the direction you came.
// fn _corner(mid: (isize, isize), src: (isize, isize), dst: (isize, isize)) -> (isize, isize) {
//     let (dy1, dx1) = ((mid.1 - src.1).signum(), (mid.0 - src.0).signum());
//     let (dy2, dx2) = ((dst.1 - mid.1).signum(), (dst.0 - mid.0).signum());
//     // println!(
//     //     "{mid:?} is a {:?} turn (+y is down)",
//     //     Turn::direction(src, mid, dst)
//     // );
//     match _Turn::_direction(src, mid, dst) {
//         _Turn::Left => (mid.0 - dx2 + dx1, mid.1 - dy2 + dy1),
//         _Turn::Right => (mid.0 + dx2 - dx1, mid.1 + dy2 - dy1),
//     }
// }

#[derive(Debug)]
struct Polygon {
    points: HashSet<(usize, usize)>,
    //lines: HashSet<Line<T>>,
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
            //lines,
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
        //println!("I'm trying to build a rectangle from {a:?} and {b:?}");
        Polygon::new(&[
            (x1.min(x2), y1.min(y2)),
            (x1.max(x2), y1.min(y2)),
            (x1.max(x2), y1.max(y2)),
            (x1.min(x2), y1.max(y2)),
        ])
    }

    fn _flood(&mut self) {
        let mut flood_queue = BTreeSet::new();
        flood_queue.insert(self.find_empty_space().unwrap());
        // let mut flood_queue = BTreeSet::from_iter((0..n).filter_map(|i| {
        //     let src = pairs[i];
        //     let mid = pairs[(i + 1) % n];
        //     let dst = pairs[(i + 2) % n];
        //     //let corner = _corner(mid, src, dst);
        //     if !pairs.contains(&corner) {
        //         Some(corner)
        //     } else {
        //         println!("skip {corner:?}");
        //         None
        //     }
        // }));
        println!("queue: {flood_queue:?}");
        while let Some(point) = flood_queue.pop_first() {
            if point.0 == self.max_y || point.1 == self.max_x {
                eprintln!("We should not have found {point:?} in the queue.");
                panic!();
            }
            if self.points.insert(point) {
                let a = (point.0 + 1, point.1);
                if !self.points.contains(&a) {
                    flood_queue.insert(a);
                }
                let b = (point.0, point.1 + 1);
                if !self.points.contains(&b) {
                    flood_queue.insert(b);
                }
                let c = (point.0 - 1, point.1);
                if !self.points.contains(&c) {
                    flood_queue.insert(c);
                }
                let d = (point.0, point.1 - 1);
                if !self.points.contains(&d) {
                    flood_queue.insert(d);
                }
            }
        }
        self.is_filled = true;
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
        //println!("got nothing for {:?}", self);
        None
    }

    fn flood_dfs(&mut self) {
        let start = match self.find_empty_space() {
            Some(start) => start,
            None => return, // Some regions don't have any empty spaces to fill.
        };
        //println!("{self:?}");
        //println!("starting at {start:?}");
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
                // let ne = (current.0 + 1, current.1 - 1);
                // let se = (current.0 + 1, current.1 + 1);
                // let nw = (current.0 - 1, current.1 - 1);
                // let sw = (current.0 - 1, current.1 + 1);
                for next in [west, east, north, south /*ne, se, nw, sw*/] {
                    if !self.points.contains(&next) {
                        stack.insert(next);
                    }
                }
            }
        }
        self.is_filled = true;
    }

    // fn is_inside_of_hollow(&self, other: &Self) -> bool {
    //     if self.is_filled {
    //         panic!("don't use this on a filled (flooded) polygon");
    //     }
    //     self.points.is_disjoint(&other.points)
    // }

    fn is_subset_of_filled(&self, other: &Self) -> bool {
        if !other.is_filled {
            panic!("don't use this against a hollow (not flooded) polygon");
        }
        self.points.is_subset(&other.points)
    }

    //     fn intersections(&self, other: &Self) -> Vec<(isize, isize)> {
    //         let h = self.lines.iter().filter_map(|e| match e {
    //             Line::Horizontal(y, x1, x2) => Some((*y, *x1, *x2)),
    //             Line::Vertical(_, _, _) => None,
    //         });
    //         let v: Vec<(isize, isize, isize)> = other
    //             .lines
    //             .iter()
    //             .filter_map(|e| match e {
    //                 Line::Vertical(x, y1, y2) => Some((*x, *y1, *y2)),
    //                 Line::Horizontal(_, _, _) => None,
    //             })
    //             .collect();
    //         let mut intersections = Vec::new();
    //         for (y, x1, x2) in h {
    //             for (x, y1, y2) in v.iter().cloned() {
    //                 if x1 <= x && x <= x2 && y1 <= y && y <= y2 {
    //                     intersections.push((x, y));
    //                 }
    //             }
    //         }
    //         intersections
    //     }
}

// #[derive(Eq, PartialEq, Hash)]
// enum Line<T> {
//     Horizontal(T, T, T),
//     Vertical(T, T, T),
// }

// impl<T> Line<T>
// where
//     T: Ord + PartialOrd + Add<T, Output = T> + Sub<T, Output = T> + Copy,
// {
//     fn new(a: &(T, T), b: &(T, T)) -> Self {
//         match (a.0 == b.0, a.1 == b.1) {
//             (true, false) => Line::Vertical(a.0, a.1.min(b.1), a.1.max(b.1)),
//             (false, true) => Line::Horizontal(a.1, a.0.min(b.0), a.0.max(b.0)),
//             (false, false) => panic!("diagonal line"),
//             (true, true) => panic!("zero-length line"),
//         }
//     }
// }

// fn rank<I, T>(items: I) -> Vec<usize>
// where
//     I: Iterator<Item = T>,
//     T: Ord,
// {
//     let items: Vec<T> = items.collect();
//     let n = items.len();
//     let mut v: Vec<usize> = (0..n).collect();
//     v.sort_by(|&a, &b| items[a].cmp(&items[b]));
//     v
// }

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

    // #[test]
    // fn corners() {
    //     assert_eq!(_corner((7, 3), (2, 3), (7, 1)), (8, 4));
    // }

    // #[test]
    // fn inside_corner() {
    //     assert_eq!(_corner((10, 10), (0, 10), (10, 20)), (9, 11));
    // }

    // #[test]
    // fn outside_corner() {
    //     assert_eq!(_corner((0, 10), (0, 0), (10, 10)), (-1, 11));
    // }

    // #[test]
    // fn list_rank() {
    //     let v = [3, 1, 4, 15, 92];
    //     let r = rank(v.iter());
    //     assert_eq!(r, [1, 0, 2, 3, 4]);
    // }
}
