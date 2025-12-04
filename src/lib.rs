use std::time::Instant;

pub trait Solver {
    fn new(input: &str) -> Self;
    fn solve(self) -> Self;
    fn time(self) -> std::time::Duration
    where
        Self: Sized,
    {
        let start = Instant::now();
        self.solve();
        start.elapsed()
    }
}
