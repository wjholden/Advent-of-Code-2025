# Advent-of-Code-2025

Rust again, this time with an object-oriented design.

# Daily Themes and Stars

1. `**` modular arithmetic, signed vs unsigned numbers
2. `**` group matching
3. `**` greedy algorithm design, dynamic programming?
4. `**` set difference, grids
5. `**` ranges, off-by-one errors
6. `**` vertical text layout
7. `  `
8. `  `
9. `  `
10. `  `
11. `  `
12. `  `

# Lessons Learned
- [`Iterator::max`](https://www.reddit.com/r/adventofcode/comments/1pcwgjl/2025_day_3_rust_iteratormax_go_brrrrr/) returns the *last* element if multiples.
- Features are pretty neat. Feels a little like preprocessing macros from C. You can use these to conditionally include code.
- You can mutate the entries in a BinaryHeap, but Rust's implementation does not "heapify" the data structure to maintain the heap invariant.
- HashSet/HashMap often outperforms BTreeSet/BTreeMap in these small puzzles.
- Range has some unstable features that might have been useful in day 5.
- There is no `windows_mut` function.
- Zed (currently) removes trailing whitespace from lines, which mattered on day 6.
