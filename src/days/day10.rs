const TOKENS: [(char, char, usize); 4] = [
    ('(', ')', 3),
    ('[', ']', 57),
    ('{', '}', 1197),
    ('<', '>', 25137),
];

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
    let input_iter = input.iter();
    let mut stack = Vec::with_capacity(len);
    if summarize_corrupted {
        input_iter.fold(0, |acc, line| acc + validate(line, &mut stack))
    } else {
        // allocation needed for sorting
        let mut scores: Vec<usize> = input_iter
            .filter_map(|line| {
                (validate(line, &mut stack) == 0).then(|| {
                    stack.iter().rev().fold(0, |acc, i| (acc * 5) + i + 1)
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
fn validate(line: &str, stack: &mut Vec<usize>) -> usize {
    stack.clear();
    for c in line.chars() {
        for (idx, (open, close, value)) in TOKENS.iter().enumerate() {
            if c == *open {
                stack.push(idx);
            }
            if c == *close {
                if TOKENS[*stack.last().unwrap()].1 == c {
                    stack.pop();
                } else {
                    return *value;
                }
            }
        }
    }
    0
}
