use super::helpers::ok_string;

aoc_macros::day_impl! {
    fn part1(&self) -> StringResult {
        let depths: Vec<usize> = s2t(self.input);

        let result: usize = depths
            .windows(2)
            .filter(|t| t[0] < t[1])
            .count();

        ok_string(result)
    },

    fn part2(&self) -> StringResult {
        let depths: Vec<usize> = s2t(self.input);

        /*
        we can combine both windowings into a single one
              stage 1    single stage
            A w          W
            B w w        W W
            C w w w      W W W
            D   w w w    W W W
            E     w w      W W
            F       w        W
              s-s
                s-s
                  s-s
              stage 2
        */
        let result: usize = depths
            .windows(4)
            .filter(|w| (w[0] + w[1] + w[2]) < (w[1] + w[2] + w[3]))
            .count();

        ok_string(result)
    }
}
