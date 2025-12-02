pub trait Solver {
    fn new(input: &str) -> Self;
    fn solve(self) -> Self;
}
