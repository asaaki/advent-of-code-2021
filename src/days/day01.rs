aoc_macros::day_impl_common_and_compute!();
fn part_1_(input: StrInputRef) -> usize {
    let depths: Vec<u16> = s2t(input);

    depths.windows(2).filter(|&t| t[0] < t[1]).count()
}

fn part_2_(input: StrInputRef) -> usize {
    let depths: Vec<u16> = s2t(input);

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
    depths.windows(4).filter(|&w| w[0] < w[3]).count()
}
