# Advent-of-Code-2025

Rust again, this time with an object-oriented design and no coding assistant.

# Daily Themes and Stars

1. `**` modular arithmetic, signed vs unsigned numbers
2. `**` group matching
3. `**` greedy algorithm design, dynamic programming?
4. `**` set difference, grids
5. `**` ranges, off-by-one errors
6. `**` vertical text layout
7. `**` dynamic programming, graph paths
8. `**` union-find/disjoint sets, Euclidean distances
9. `**` coordinate compression, geometry, flood fill
10. `**` linear algebra, constraint solvers
11. `**` directed acyclic graphs, DFS, dynamic programming
12. `**` special cases of NP-complete problems

# Lessons Learned
- [`Iterator::max`](https://www.reddit.com/r/adventofcode/comments/1pcwgjl/2025_day_3_rust_iteratormax_go_brrrrr/) returns the *last* element if multiples.
- Features are pretty neat. Feels a little like preprocessing macros from C. You can use these to conditionally include code.
- You can mutate the entries in a BinaryHeap, but Rust's implementation does not "heapify" the data structure to maintain the heap invariant.
- HashSet/HashMap often outperforms BTreeSet/BTreeMap in these small puzzles.
- Range has some unstable features that might have been useful in day 5.
- There is no `windows_mut` function.
- Zed (currently) removes trailing whitespace from lines, which mattered on day 6.
- If you only need set entries once, then you might be able to significantly speed up an algorithm by removing the data instead of just checking contains.
- In day 8, using a priority queue should be (I don't have it working yet) a substantial performance improvement over populating a full distances matrix and then sorting it.
- Also in day 8, you don't need to calculate the square roots of the distances.
- Tuples are supposed to be ordered in lexical order, from left to right.
- Coordinate compression is so clever!
- [`BinaryHeap::iter` doesn't do what you expect](https://github.com/rust-lang/rust/issues/59278). Use `BinaryHeap::pop` instead.
- I'm excited about the potential in [Zelen](https://github.com/radevgit/zelen) and [Selen](https://github.com/radevgit/selen), but not presently fast enough for [Day 10](https://adventofcode.com/2025/day/10). See also [Zelen issue #7](https://github.com/radevgit/zelen/issues/7).
- [Pumpkin](https://github.com/consol-lab/pumpkin) solves Day 10 in about 30 seconds (in parallel) on my machine, but this is still substantially slower than calling [Gecode](https://www.gecode.dev) through [MiniZinc](https://www.minizinc.org). The Pumpkin developers quickly responded to a question I raised about how to model equality constraints (see [Pumpkin issue #336](https://github.com/ConSol-Lab/Pumpkin/issues/336#issuecomment-3654540256)).
- Ratatui is nice. I'd like to write more TUI apps like this.
- A Min-Max Heap sounds [really interesting](https://probablydance.com/2020/08/31/on-modern-hardware-the-min-max-heap-beats-a-binary-heap/), but there is [no Min-Max Heap in `std::collections`](https://github.com/rust-lang/rust/issues/76250) and the [min-max-heap crate](https://crates.io/crates/min-max-heap) might not be actively maintained (there are other implementations on Crates.io).
- [So true](https://kieranhealy.org/blog/archives/2022/05/20/every-springer-math-text/).
- Algorithm to [determine if a point is inside a polygon](https://math.stackexchange.com/a/59820/474318).
- Function to [determine if a point is to the left or right of a line](https://stackoverflow.com/a/3461533/5459668).
