aoc_macros::day_impl! {
    fn part1(&self) -> StringResult {
        let bits: Vec<Vec<u32>> = bits(self.input);
        let rotated = rotate(&bits);

        let gamma: Vec<u32> = rotated.iter()
        .map(|b| {
            if b.iter().sum::<u32>() >= (b.len()/2) as u32 { 1 } else { 0 }
        }).collect();

        let epsilon: Vec<u32> = gamma.iter().map(|b| if b == &1 { 0 } else { 1 }).collect();

        let (gamma, epsilon) = (bv2num(&gamma),bv2num(&epsilon));
        let pc = gamma *epsilon;
        ok_string(pc)
    },

    fn part2(&self) -> StringResult {
        let bits: Vec<Vec<u32>> = bits(self.input);
        let oxy = most_least_final(&bits, true);
        let co2 = most_least_final(&bits, false);

        let (oxy, co2) = (bv2num(&oxy),bv2num(&co2));
        let rating = oxy * co2;

        ok_string(rating)
    }
}

fn bits(input: StrInputRef) -> Vec<Vec<u32>> {
    input
        .iter()
        .map(|line| {
            let b: Vec<u32> = line
                .chars()
                .map(|c| c.to_digit(2).expect("bin digit") as u32)
                .collect();
            b
        })
        .collect()
}

fn rotate(bits: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    bits.iter().enumerate().fold(
        vec![vec![0; bits.len()]; bits[0].len()],
        |mut a, (i, num)| {
            for (ni, b) in num.iter().enumerate() {
                a[ni][i] = *b;
            }
            a
        },
    )
}

fn bv2num(bv: &Vec<u32>) -> u32 {
    bv.iter()
        .enumerate()
        .fold(0, |a, (i, b)| a + b * 2u32.pow((bv.len() - 1 - i) as u32))
}

fn most_least_final(bits: &Vec<Vec<u32>>, most: bool) -> Vec<u32> {
    let mut oxys = bits.clone();
    for b in 0..oxys[0].len() {
        let r = rotate(&oxys);
        let r = &r[b];

        let (o, z): (Vec<u32>, Vec<u32>) = r.iter().partition(|&i| i == &1);
        let (oc, zc) = (o.iter().count(), z.iter().count());
        let (m, l) = if oc >= zc { (1, 0) } else { (0, 1) };

        let new = oxys
            .iter()
            .filter(|o| {
                let check = if most { m } else { l };
                o[b] == check
            })
            .map(Clone::clone)
            .collect();
        oxys = new;
        if oxys.len() == 1 {
            break;
        }
    }
    let result = &oxys[0];
    result.clone()
}
