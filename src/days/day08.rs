const UNIQUE_DIGITS: [usize; 4] = [2, 3, 4, 7];

// idx = digit, value = segments
const SEQMENTS_PER_DIGIT: [usize; 10] = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];

aoc_macros::day_impl_common_and_compute!();

fn part_1_(input: StrInputRef) -> usize {
    unprocessed_lines_iter(input)
        .map(|(_input, output)| output)
        .fold(0, |acc, line| {
            acc + line
                .filter(|&seq| UNIQUE_DIGITS.contains(&seq.len()))
                .count()
        })
}

fn part_2_(input: StrInputRef) -> usize {
    // slots for number 0 ... 9
    // How to init an array of Options: https://github.com/rust-lang/rust/issues/44796
    let mut matches: [Option<&str>; 10] = Default::default();
    // all inputs remove 4 uniques, so we need to store only 6 more items
    let mut inputs: Vec<&str> = Vec::with_capacity(6);

    unprocessed_lines_iter(input)
        .map(|(i_iter, o_iter)| {
            // reset for each line
            matches = Default::default();
            inputs.clear();

            // first round, find unique values (1,4,7,8) + match them
            // and also remove them from the input list for the next step
            inputs = i_iter
                .filter(|&token| {
                    if UNIQUE_DIGITS.contains(&token.len()) {
                        matches[SEQMENTS_PER_DIGIT
                            .iter()
                            .position(|&v| v == token.len())
                            .unwrap()] = Some(token);
                        false
                    } else {
                        true
                    }
                })
                .collect();

            loop {
                for &token in &inputs {
                    // remaining tokens have length 5 or 6
                    match token.len() {
                        5 => {
                            // find 2, 3, 5
                            if let Some(one) = matches[1] {
                                if intersect_count(token, one) == 2 {
                                    matches[3] = Some(token);
                                }
                            }
                            if let (Some(three), Some(four), Some(seven)) =
                                (matches[3], matches[4], matches[7])
                            {
                                if intersect_count(token, three) == 4
                                    && intersect_count(token, four) == 2
                                    && intersect_count(token, seven) == 2
                                {
                                    matches[2] = Some(token);
                                }
                            }
                            if let (Some(two), Some(four)) =
                                (matches[2], matches[4])
                            {
                                if intersect_count(token, two) == 3
                                    && intersect_count(token, four) == 3
                                {
                                    matches[5] = Some(token);
                                }
                            }
                        }
                        6 => {
                            // find 0, 6, 9
                            if let (Some(one), Some(two)) =
                                (matches[1], matches[2])
                            {
                                if intersect_count(token, one) == 1
                                    && intersect_count(token, two) == 4
                                {
                                    matches[6] = Some(token);
                                }
                            }
                            if let (Some(one), Some(two), Some(three)) =
                                (matches[1], matches[2], matches[3])
                            {
                                if intersect_count(token, one) == 2
                                    && intersect_count(token, two) == 4
                                    && intersect_count(token, three) == 4
                                {
                                    matches[0] = Some(token);
                                }
                            }
                            if let (Some(zero), Some(seven)) =
                                (matches[0], matches[7])
                            {
                                if intersect_count(token, seven) == 3
                                    && intersect_count(token, zero) == 5
                                {
                                    matches[9] = Some(token);
                                }
                            }
                        }
                        _ => (), // skip (we have to catch all, otherwise non-exhaustive match error)
                    }
                }

                // once all slots are filled, we can leave
                if matches.iter().all(Option::is_some) {
                    break;
                }
            }

            // calculate displayed segment number
            // .zip(0u32..) as alternative to .enumerate(), to return a different num type;
            // be careful, the position is swapped (item, idx) instead of (idx, item);
            // why? avoids casting
            o_iter.rev().zip(0u32..).fold(0, |acc, (seq, idx)| {
                acc + (matches
                    .iter()
                    .position(|v| same_chars(v.unwrap(), seq))
                    .unwrap()
                    * 10usize.pow(idx))
            })
        })
        .sum()
}

fn same_chars(left: &str, right: &str) -> bool {
    left.len() == right.len() && left.chars().all(|l| right.contains(l))
}

fn intersect_count(left: &str, right: &str) -> usize {
    left.chars()
        .fold(0, |acc, c| if right.contains(c) { acc + 1 } else { acc })
}

fn unprocessed_lines_iter(
    input: StrInputRef,
) -> impl Iterator<
    Item = (
        impl Iterator<Item = &str> + '_,
        impl DoubleEndedIterator<Item = &str> + '_,
    ),
> + '_ {
    input.iter().map(move |line| {
        line.split_once(" | ")
            .map(|(left, right)| {
                (left.split_whitespace(), right.split_whitespace())
            })
            .expect("malformed input data line; no ` | ` delimiter found")
    })
}
