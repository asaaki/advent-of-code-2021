type Item = u8;
type Row = Vec<Item>;
type MI = MatrixV<Item>;

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

fn input2matrix(input: StrInputRef, transposed: bool) -> MI {
    let mut m = MatrixV::new(input[0].len());
    m.fill(&bitstream(input));
    if transposed {
        m.transpose();
    }
    m
}

fn part_1_(input: StrInputRef) -> usize {
    let input = input2matrix(input, true);

    let gamma: Row = input
        .iter_rows()
        .map(|b| {
            // TODOs: Vec<T>: .sum_usize(), .sum_non_zeros{_usize}(), .count_non_zeros{_usize}()
            if b.iter().map(|i| *i as usize).sum::<usize>() >= (b.len() / 2) {
                1
            } else {
                0
            }
        })
        .collect();

    let epsilon: Row = gamma.invert(None);

    let (gamma, epsilon) = (bits2num(&gamma[..]), bits2num(&epsilon[..]));
    gamma * epsilon
}

fn part_2_(input: StrInputRef) -> usize {
    let input = input2matrix(input, false);
    let oxy = most_least_final(&input, true);
    let co2 = most_least_final(&input, false);
    oxy * co2
}

fn bitstream(input: StrInputRef) -> Row {
    input
        .iter()
        .flat_map(|line| {
            line.chars()
                .map(|c| c.to_digit(2).expect("bin digit") as Item)
        })
        .collect()
}

// [ ] init step: use hard-coded data type everywhere
// [*] middle step: make data type configurable
// [ ] final step: make data type generic
fn bits2num(bv: &[Item]) -> usize {
    bv.iter().enumerate().fold(0, |a, (i, b)| {
        a + (*b) as usize * 2usize.pow(bv.len() as u32 - 1 - i as u32)
    })
}

fn most_least_final(bits: &MI, most: bool) -> usize {
    let mut data = bits.clone();
    let steps = 0..data.chunk_size();
    for step in steps {
        let column = data.iter_cols().nth(step).unwrap();
        let (o, z): (Row, Row) = column.into_iter().partition(|&i| i == &1);
        let (oc, zc) = (o.iter().count(), z.iter().count());
        let (m, l) = if oc >= zc { (1, 0) } else { (0, 1) };

        let new: Row = data
            .iter_rows()
            .filter(|o| {
                let check = if most { m } else { l };
                o[step] == check
            })
            .flatten()
            .map(ToOwned::to_owned)
            .collect();
        let mut new_data = MatrixV::new(data.chunk_size());
        new_data.fill(&new);
        data = new_data;

        if data.len() == data.chunk_size() {
            break;
        }
    }
    let result = &data.iter_rows().next().unwrap();
    bits2num(result)
}
