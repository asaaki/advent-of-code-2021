aoc_macros::day_impl! {
    fn part1(&self) -> StringResult {
        let depths: Vec<u16> = s2t(self.input);

        let result = depths
            .windows(2)
            .filter(|&t| t[0] < t[1])
            .count();

        ok_string(result)
    },

    fn part2(&self) -> StringResult {
        let depths: Vec<u16> = s2t(self.input);

        /*
        we can combine both windowings into a single one

              stage 1    single stage     optimization
            A w          W                X--
            B w w        W W              ---
            C w w w      W W W            ---
            D   w w w    W W W            --Y
            E     w w      W W
            F       w        W
              s-s                         X<Y
                s-s
                  s-s
              stage 2
        */
        let result = depths
            .windows(4)
            .filter(|&w| w[0] < w[3])
            .count();

        ok_string(result)
    }
}
