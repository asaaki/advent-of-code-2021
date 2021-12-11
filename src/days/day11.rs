use std::{collections::HashSet, ops::AddAssign};
use unstd::math::usize::divrem;

const DIM: usize = 10;
const IDIM: isize = DIM as isize;
const SIZE: usize = DIM * DIM;

type Digit = u8;
type Map<'a> = MatrixA<'a, Digit, SIZE>;
type FlashSet = HashSet<(usize, usize)>;
type Queue = Vec<(usize, usize)>;

#[rustfmt::skip]
const NEIGHBOURS: [(isize, isize); 8] = [
    (-1, -1), ( 0, -1), ( 1, -1),
    (-1,  0), /* 🐙 */  ( 1,  0),
    (-1,  1), ( 0,  1), ( 1,  1),
];

aoc_macros::day_impl_common!();

fn compute(input: StrInputRef, first_part: bool) -> usize {
    let mut map = init_map(input);
    // quite a lot of temporaries and states ¯\_(ツ)_/¯
    let mut queue = Queue::new();
    let mut flashes = FlashSet::new();
    let mut total = 0;

    if first_part {
        for _day in 1..=SIZE {
            iterate(&mut map, &mut queue, &mut flashes);
            total += flashes.len();
            flashes.clear();
        }
        total
    } else {
        for d in 1.. {
            iterate(&mut map, &mut queue, &mut flashes);
            if flashes.len() == SIZE {
                return d;
            }
            flashes.clear();
        }
        0
    }
}

fn init_map(input: StrInputRef) -> Map {
    let mut map = Map::new(DIM);
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.char_indices() {
            map.insert(x, y, c.to_digit(10).unwrap() as Digit);
        }
    }
    map
}

fn iterate(map: &mut Map, queue: &mut Queue, flashes: &mut FlashSet) {
    queue.clear();
    map.iter_mut().for_each(|o| o.add_assign(1));

    for idx in 0..SIZE {
        let (y, x) = divrem(&idx, &DIM);
        let v = *map.get(x, y).unwrap();
        if v > 9 {
            queue.push((x, y));
        }
    }

    while !queue.is_empty() {
        let (x, y) = queue.pop().unwrap();
        if flashes.contains(&(x, y)) {
            continue;
        }
        let v = *map.get(x, y).unwrap() + 1;
        map.insert(x, y, v);
        if v <= 9 {
            continue;
        }
        flashes.insert((x, y));

        for (dx, dy) in NEIGHBOURS {
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if (0..IDIM).contains(&nx) && (0..IDIM).contains(&ny) {
                // let nv = { *map.get(nx as usize, ny as usize).unwrap() };
                queue.push((nx as usize, ny as usize));
            }
        }
    }

    for (x, y) in flashes.iter() {
        map.insert(*x, *y, 0);
    }
}
