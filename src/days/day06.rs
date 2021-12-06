const DAYS_PART1: u8 = 79;
const DAYS_PART2: u8 = 255;

aoc_macros::day_impl! {
    fn part1(&self) -> StringResult {
        let result = compute(self.input, DAYS_PART1);
        ok_string(result)
    },

    fn part2(&self) -> StringResult {
        let result = compute(self.input, DAYS_PART2);
        ok_string(result)
    }
}

/*
The naive approach:
- create a list with the counters ([3,4,3,1,2])
- iterate each day: decrement the counters
  - process the zeros (-1 -> 6), and add new counters (8s) for each
- count the values

This works for a while, but memory is not unlimited and also constantly adding values to
a list (Vec) causes regular resizing, meaning lots of new memory allocations.

Smarter approach:

- keep a list of "counts of counters";
  the index is the counter and the value counts the fishes with that counter
  ([0,1,1,2,1,0,0,0,0])
  Note: sadly we need to keep 9 items, which probably is inefficient in memory layout/cache;
        we use usizes (u64s) for the values, 9*64 = 576 bits (72 bytes),
        so beyond a usual cache line; u32::MAX is sadly too low for the challenge
        u32::MAX      4294967295
        challenge  1708791884591
- iterate each day:
  - use temp storage for the count of counters; reset values to zeros
  - iterate each slot:
    - if slot is 0: add the count to the 6s ("readd") and to the 8s (create) of temp
    - otherwise: add count to slot-1 (decrement) of temp
  - swap content of counters with temp
- summarize the values of the list (total fishes)
*/

type CounterValue = usize;
type Counter = [CounterValue; 9];

fn compute(input: StrInputRef, days: u8) -> CounterValue {
    let population: Vec<u8> = input[0]
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    let mut counters: Counter = [0; 9];
    // for more efficient memory usage always reuse the same array for the loop:
    let mut next_counters: Counter = [0; 9];

    population.iter().for_each(|&f| counters[f as usize] += 1);

    for _day in 0..=days {
        reset(&mut next_counters);
        for (slot, count) in counters.iter().enumerate() {
            if slot == 0 {
                next_counters[6] += count;
                next_counters[8] += count;
            } else {
                next_counters[slot - 1] += count;
            }
        }
        counters = next_counters;
    }
    counters.iter().sum()
}

// Hope/fingers crossed: Rust/LLVM can optimize this enough
// so that this is better than creating a new array for each day.
// maybe-TODO: benchmark this assumption
fn reset(arr: &mut Counter) {
    arr.iter_mut().for_each(|x| *x = 0)
}
