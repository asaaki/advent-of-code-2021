const TAGS: [(char, char); 4] =
    [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];

const TAG_VALUES: [usize; 4] = [3, 57, 1197, 25137];

aoc_macros::day_impl! {
    fn part1(&self) -> StringResult {
        let result = compute(self.input, true);
        ok_string(result)
    },

    fn part2(&self) -> StringResult {
        let result = compute(self.input, false);
        ok_string(result)
    }
}

fn compute(input: StrInputRef, summarize_corrupted: bool) -> usize {
    let len = maxlen(input);
    let mut stack = Vec::with_capacity(len);
    if summarize_corrupted {
        input
            .iter()
            .fold(0, |acc, line| acc + validate(line, &mut stack))
    } else {
        let mut scores: Vec<usize> = input
            .iter()
            .filter_map(|line| {
                (validate(line, &mut stack) == 0).then(|| {
                    stack.iter().rev().fold(0, |acc, c| {
                        (acc * 5)
                            + TAGS.iter().position(|(_, t)| t == c).unwrap()
                            + 1
                    })
                })
            })
            .collect();
        scores.sort_unstable();
        scores[scores.len() / 2]
    }
}

// to reserve the minimum max capacity necessary
// worst case scenario: a full list of opening tags
#[inline]
fn maxlen(input: StrInputRef) -> usize {
    input
        .iter()
        .map(|l| l.len())
        .max_by(|l1, l2| l1.cmp(l2))
        .unwrap_or(128)
}

// #[inline]
fn validate(line: &str, stack: &mut Vec<char>) -> usize {
    stack.clear();
    for c in line.chars() {
        for (idx, (open, close)) in TAGS.iter().enumerate() {
            if c == *open {
                stack.push(*close);
            }
            if c == *close {
                if stack.last().unwrap() == &c {
                    stack.pop();
                } else {
                    return TAG_VALUES[idx];
                }
            }
        }
    }
    0
}
