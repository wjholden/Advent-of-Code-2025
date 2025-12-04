use std::time::Instant;

pub trait Solver {
    fn new(input: &str) -> Self;
    fn solve(self) -> Self;
    fn time(input: &str) -> std::time::Duration
    where
        Self: Sized,
    {
        let start = Instant::now();
        Self::new(input).solve();
        start.elapsed()
    }
}
