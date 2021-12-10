// note: rustfmt (cargo fmt) does not format the macro use;
// https://github.com/rust-lang/rustfmt#limitations
// - also rust analyzer has problems with macro block usage
aoc_macros::day_impl! {
    fn part1(&self) -> StringResult {
        ok_string("<expected result part 1>")
    },

    fn part2(&self) -> StringResult {
        ok_string("<expected result part 2>")
    }
}
