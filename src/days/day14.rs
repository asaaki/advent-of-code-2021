use std::{
    collections::HashMap,
    ops::{AddAssign, SubAssign},
};

const A_IDX: usize = 'A' as usize;

aoc_macros::day_impl_common!();

fn compute(input: StrInputRef, first_part: bool) -> usize {
    let init_squence = &input[0];
    let raw_mappings = &input[2..];
    let scratchpad_size = raw_mappings.len() * 2; // roughly twice the size needed

    let rules: Vec<(usize, usize, usize)> = raw_mappings
        .iter()
        .map(|line| {
            let (seq, m) = line.split_once(" -> ").unwrap();
            let l = seq[..1].chars().next().unwrap() as usize - A_IDX;
            let r = seq[1..].chars().next().unwrap() as usize - A_IDX;
            let m = m.chars().next().unwrap() as usize - A_IDX;
            (l, r, m)
        })
        .collect();

    let mut seq_counts: HashMap<(usize, usize), usize> = raw_mappings
        .iter()
        .map(|line| {
            let (seq, _c) = line.split_once(" -> ").unwrap();
            let a = seq[..1].chars().next().unwrap() as usize - A_IDX;
            let b = seq[1..].chars().next().unwrap() as usize - A_IDX;
            ((a, b), 0)
        })
        .collect();

    let mut char_counts_a: [usize; 26] = [0; 26];

    for c in init_squence.chars() {
        char_counts_a[c as u32 as usize - A_IDX] += 1;
    }

    for w in init_squence.as_bytes().windows(2) {
        let key = (w[0] as usize - A_IDX, w[1] as usize - A_IDX);
        seq_counts.entry(key).or_insert(0).add_assign(1);
    }

    let mut to_increment = Vec::with_capacity(scratchpad_size);
    let mut to_decrement = Vec::with_capacity(scratchpad_size);
    let mut biggest = 0usize;

    let max_steps = if first_part { 10 } else { 40 };
    for _step in 0..max_steps {
        to_decrement.clear();
        to_increment.clear();

        for (l, r, m) in rules.iter() {
            if let Some(c) = seq_counts.get_mut(&(*l, *r)) {
                if c > &mut 0 {
                    char_counts_a[*m] += *c;
                    to_decrement.push((*l, *r, *c));
                    to_increment.push((*l, *m, *c));
                    to_increment.push((*m, *r, *c));
                }
            }
        }
        for (l, r, c) in to_decrement.iter() {
            seq_counts.entry((*l, *r)).or_insert(0).sub_assign(c);
        }
        for (l, r, c) in to_increment.iter() {
            seq_counts.entry((*l, *r)).or_insert(0).add_assign(c);
        }

        biggest = biggest.max(to_increment.len());
    }

    let max = char_counts_a.iter().max().unwrap();
    let min = char_counts_a.iter().filter(|&&v| v > 0).min().unwrap();
    max - min
}
