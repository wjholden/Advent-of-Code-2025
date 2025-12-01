pub trait Puzzle<T, R>
where
    T: std::fmt::Display,
    R: std::fmt::Display,
{
    fn new(input: &str) -> Self;
    fn part1(&self) -> T;
    fn part2(&self) -> R;
    fn solve(&mut self);
}
