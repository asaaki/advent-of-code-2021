aoc_macros::day_impl! {
    fn step1(&self) -> StringResult {
        let depths: Vec<usize> = s2t(self.input);

        let result: usize = depths.windows(2).filter(|&t| t[0] < t[1]).count();

        Ok(format!("{}", result))
    },

    fn step2(&self) -> StringResult {
        let depths: Vec<usize> = s2t(self.input);
        // due to .windows not available on iterators, we have to take this midstate first
        let sums: Vec<usize> = depths.windows(3).map(|w| w.iter().sum()).collect();
        // … to then .windows and .filter again
        let result: usize = sums.windows(2).filter(|&t| t[0] < t[1]).count();

        Ok(format!("{}", result))
    }
}
