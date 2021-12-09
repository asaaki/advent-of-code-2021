type Sequence = Vec<String>;

const UNIQUE_DIGITS: [usize; 4] = [2, 3, 4, 7];

// idx = digit, value = segments
const SEQMENTS_PER_DIGIT: [usize; 10] = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];

aoc_macros::day_impl! {
    fn part1(&self) -> StringResult {
        let result = part_1_(self.input);
        ok_string(result)
    },

    fn part2(&self) -> StringResult {
        let result = part_2_(self.input);
        ok_string(result)
    }
}

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
    unprocessed_lines_iter(input)
        .map(|(i_iter, o_iter)| {
            // need to allocate, because nested iterations
            let outputs: Sequence = o_iter.map(ToOwned::to_owned).collect();

            if outputs.iter().all(|s| UNIQUE_DIGITS.contains(&s.len())) {
                // simple shortcut: all outputs are unique segment digits
                outputs.iter().rev().enumerate().fold(0, |acc, (idx, seq)| {
                    acc + (SEQMENTS_PER_DIGIT
                        .iter()
                        .position(|&v| v == seq.len())
                        .unwrap()
                        * 10usize.pow(idx as u32))
                })
            } else {
                // not so simple, we need to find and match segments

                // slots for number 0 ... 9
                let mut matches: [Option<String>; 10] = [
                    None, None, None, None, None, None, None, None, None, None,
                ];

                // first round, find unique values (1,4,7,8) + match them
                // and also remove them from the input list for the next step
                let inputs: Sequence = i_iter
                    .filter(|&t| {
                        if UNIQUE_DIGITS.contains(&t.len()) {
                            matches[SEQMENTS_PER_DIGIT
                                .iter()
                                .position(|&v| v == t.len())
                                .unwrap()] = Some(t.to_owned());
                            false
                        } else {
                            true
                        }
                    })
                    .map(ToOwned::to_owned)
                    .collect();

                loop {
                    for token in &inputs {
                        // remaining tokens have length 5 or 6
                        match token.len() {
                            5 => {
                                // find 2, 3, 5
                                if let Some(one) = matches[1].as_ref() {
                                    if intersect_count(token, one) == 2 {
                                        matches[3] = Some(token.to_owned());
                                    }
                                }
                                // very verbose, sorry …
                                if matches[1].is_some()
                                    && matches[3].is_some()
                                    && matches[4].is_some()
                                    && matches[7].is_some()
                                {
                                    let (&one, &three, &four, &seven) = (
                                        &matches[1].as_ref().unwrap(),
                                        &matches[3].as_ref().unwrap(),
                                        &matches[4].as_ref().unwrap(),
                                        &matches[7].as_ref().unwrap(),
                                    );
                                    if intersect_count(token, one) == 1
                                        && intersect_count(token, three) == 4
                                        && intersect_count(token, four) == 2
                                        && intersect_count(token, seven) == 2
                                    {
                                        matches[2] = Some(token.to_owned());
                                    }
                                }
                                if matches[2].is_some() && matches[4].is_some()
                                {
                                    let (&two, &four) = (
                                        &matches[2].as_ref().unwrap(),
                                        &matches[4].as_ref().unwrap(),
                                    );
                                    if intersect_count(token, two) == 3
                                        && intersect_count(token, four) == 3
                                    {
                                        matches[5] = Some(token.to_owned());
                                    }
                                }
                            }
                            6 => {
                                // find 0, 6, 9
                                if matches[1].is_some()
                                    && matches[2].is_some()
                                    && matches[3].is_some()
                                {
                                    let (&one, &two, &three) = (
                                        &matches[1].as_ref().unwrap(),
                                        &matches[2].as_ref().unwrap(),
                                        &matches[3].as_ref().unwrap(),
                                    );
                                    if intersect_count(token, one) == 2
                                        && intersect_count(token, two) == 4
                                        && intersect_count(token, three) == 4
                                    {
                                        matches[0] = Some(token.to_owned());
                                    }
                                }
                                if matches[7].is_some() && matches[0].is_some()
                                {
                                    let (&seven, &zero) = (
                                        &matches[7].as_ref().unwrap(),
                                        &matches[0].as_ref().unwrap(),
                                    );
                                    if intersect_count(token, seven) == 3
                                        && intersect_count(token, zero) == 5
                                    {
                                        matches[9] = Some(token.to_owned());
                                    }
                                }
                                if matches[1].is_some() && matches[2].is_some()
                                {
                                    let (&one, &two) = (
                                        &matches[1].as_ref().unwrap(),
                                        &matches[2].as_ref().unwrap(),
                                    );
                                    if intersect_count(token, one) == 1
                                        && intersect_count(token, two) == 4
                                    {
                                        matches[6] = Some(token.to_owned());
                                    }
                                }
                            }
                            _ => (), // skip (we have to catch all, otherwise non-exhaustive)
                        }
                    }

                    // once all slots are filled, we can leave
                    if matches.iter().all(Option::is_some) {
                        break;
                    }
                }

                // calculate displayed segment number
                outputs.iter().rev().enumerate().fold(0, |acc, (idx, seq)| {
                    acc + (matches
                        .iter()
                        .position(|v| {
                            sorted(v.as_ref().unwrap()) == sorted(seq)
                        })
                        .unwrap()
                        * 10usize.pow(idx as u32))
                })
            }
        })
        .sum()
}

fn sorted(t: &str) -> String {
    let mut chars: Vec<char> = t.chars().collect();
    chars.sort_unstable();
    String::from_iter(chars)
}

fn intersect_count(left: &str, right: &str) -> usize {
    left.chars()
        .fold(0, |acc, c| if right.contains(c) { acc + 1 } else { acc })
}

fn unprocessed_lines_iter(
    input: StrInputRef,
) -> impl Iterator<
    Item = (
        impl DoubleEndedIterator<Item = &str> + '_,
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
