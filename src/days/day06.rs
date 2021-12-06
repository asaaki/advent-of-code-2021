const DAYS_PART1: usize = 80;
const DAYS_PART2: usize = 256;

aoc_macros::day_impl! {
    fn part1(&self) -> StringResult {
        let result = part_1_(self.input, DAYS_PART1);
        ok_string(result)
    },

    fn part2(&self) -> StringResult {
        let result = part_2_(self.input, DAYS_PART2);
        ok_string(result)
    }
}

fn part_1_(input: StrInputRef, days: usize) -> usize {
    let input_iter = input[0].split(',').map(|s| s.parse::<u8>().unwrap());
    let mut population = Vec::with_capacity(1_000_000_000);
    population.extend(input_iter);

    for _ in 0..days {
        let mut new_fishes = 0;
        for fish in population.iter_mut() {
            match fish.checked_sub(1) {
                Some(f) => *fish = f,
                None => {
                    *fish = 6;
                    new_fishes += 1;
                }
            }
        }
        for _ in 0..new_fishes { population.push(8); }
    }

    population.len()
}

fn part_2_(input: StrInputRef, days: usize) -> usize {
    let input_iter = input[0].split(',').map(|s| s.parse::<u8>().unwrap());
    let mut population = Vec::with_capacity(1_000_000_000);
    population.extend(input_iter);

    let mut ages = [0usize; 9];
    // for more efficient memory usage always reuse the same array for the loop:
    let mut next_ages = [0usize; 9];

    population.iter().for_each(|&f| ages[f as usize] += 1);

    for _day in 0..days {
        reset(&mut next_ages);
        for (slot, count) in ages.iter().enumerate() {
            if slot == 0 {
                next_ages[6] += count;
                next_ages[8] += count;
            } else {
                next_ages[slot - 1] += count;
            }
        }
        ages = next_ages;
    }
    ages.iter().sum()
}

fn reset(arr: &mut [usize]) {
    arr.iter_mut().for_each(|x| *x = 0)
}
