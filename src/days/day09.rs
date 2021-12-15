/*
    NOTE: naive and inefficient solution for now

    IDEAS: we can probably combine both maps into one;
    - U -> U(usize) = keep track of height for first round
    - new variant: L(usize) = set low points
    - combine both computations and use a flag to stop early for part 1
    - loop break condition: no U's and L's OR just B' and S's
*/

type Digit = u8;
type MapLine = Vec<Digit>;
type Map = Vec<MapLine>;
type MapRef<'a> = &'a [MapLine];
type MaybeDigit<'a> = Option<&'a Digit>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Cell {
    B, // boundary
    U, // unset ; U(usize) // unset with height value
    // L(usize), // set with low point code
    S(usize), // set with area code
}

type CellLine = Vec<Cell>;
type CellMap = Vec<CellLine>;

aoc_macros::day_impl_common_and_compute!();

fn part_1_(input: StrInputRef) -> usize {
    let map = parse(input);
    let (width, height) = (map[0].len(), map.len());
    let mut low_points = vec![];
    for y in 0..height {
        for x in 0..width {
            let (item, top, bottom, left, right): (
                MaybeDigit,
                MaybeDigit,
                MaybeDigit,
                MaybeDigit,
                MaybeDigit,
            ) = (
                map.get(y).and_then(|yline| yline.get(x)),
                map.get(y.wrapping_sub(1)).and_then(|yline| yline.get(x)),
                map.get(y.wrapping_add(1)).and_then(|yline| yline.get(x)),
                map.get(y).and_then(|yline| yline.get(x.wrapping_sub(1))),
                map.get(y).and_then(|yline| yline.get(x.wrapping_add(1))),
            );
            if let Some(item) = item {
                let checks = [
                    top.map(|n| n > item),
                    bottom.map(|n| n > item),
                    left.map(|n| n > item),
                    right.map(|n| n > item),
                ];
                let is_lowest =
                    checks.iter().filter_map(|o| o.as_ref()).all(|&v| v);
                if is_lowest {
                    low_points.push(*item as usize + 1);
                }
            }
        }
    }
    low_points.iter().sum()
}

fn part_2_(input: StrInputRef) -> usize {
    let basemap = parse(input);
    let mut map = parse_to_bitmap(&basemap);
    let (width, height) = (map[0].len(), map.len());

    // like part 1, but we store coordinates this time
    let mut low_points = vec![];
    for y in 0..height {
        for x in 0..width {
            let (item, top, bottom, left, right): (
                MaybeDigit,
                MaybeDigit,
                MaybeDigit,
                MaybeDigit,
                MaybeDigit,
            ) = (
                basemap.get(y).and_then(|yline| yline.get(x)),
                basemap
                    .get(y.wrapping_sub(1))
                    .and_then(|yline| yline.get(x)),
                basemap
                    .get(y.wrapping_add(1))
                    .and_then(|yline| yline.get(x)),
                basemap
                    .get(y)
                    .and_then(|yline| yline.get(x.wrapping_sub(1))),
                basemap
                    .get(y)
                    .and_then(|yline| yline.get(x.wrapping_add(1))),
            );
            if let Some(item) = item {
                let checks = [
                    top.map(|n| n > item),
                    bottom.map(|n| n > item),
                    left.map(|n| n > item),
                    right.map(|n| n > item),
                ];
                let is_lowest =
                    checks.iter().filter_map(|o| o.as_ref()).all(|&v| v);
                if is_lowest {
                    low_points.push((x, y));
                }
            }
        }
    }

    // set low points in cell map as starting points
    for (i, (x, y)) in low_points.iter().enumerate() {
        map[*y][*x] = Cell::S(i + 1);
    }

    // final stage of cell map must be either B's or S(_)'s
    loop {
        // we're done if no U cell is left
        if map.iter().flatten().filter(|&&c| c == Cell::U).count() == 0 {
            break;
        }

        for y in 0..height {
            for x in 0..width {
                match map[y][x] {
                    Cell::B | Cell::U => continue,
                    Cell::S(v) => {
                        let top = map
                            .get(y.wrapping_sub(1))
                            .and_then(|yline| yline.get(x))
                            .map(|_| (x, y.wrapping_sub(1)));
                        let bottom = map
                            .get(y.wrapping_add(1))
                            .and_then(|yline| yline.get(x))
                            .map(|_| (x, y.wrapping_add(1)));
                        let left = map
                            .get(y)
                            .and_then(|yline| yline.get(x.wrapping_sub(1)))
                            .map(|_| (x.wrapping_sub(1), y));
                        let right = map
                            .get(y)
                            .and_then(|yline| yline.get(x.wrapping_add(1)))
                            .map(|_| (x.wrapping_add(1), y));

                        let coords = [top, left, right, bottom];
                        let coords: Vec<(usize, usize)> =
                            coords.iter().filter_map(|&c| c).collect();
                        for (x, y) in coords {
                            if map[y][x] != Cell::B {
                                map[y][x] = Cell::S(v);
                            }
                        }
                    }
                }
            }
        }
    }

    let areas: Vec<&usize> = map
        .iter()
        .flat_map(|line| {
            line.iter().filter_map(|c| match c {
                Cell::S(v) => Some(v),
                _ => None,
            })
        })
        .collect();
    let &&max = areas.iter().max().unwrap();
    let mut sums = vec![0; max + 1];
    areas.iter().for_each(|&&v| {
        sums[v] += 1;
    });
    sums.sort_unstable();
    sums.reverse();
    sums[..3].iter().product()
}

fn parse(input: StrInputRef) -> Map {
    input
        .iter()
        .map(|line| {
            line.bytes()
                .map(|c| (c - b'0'))
                .collect()
        })
        .collect()
}

fn parse_to_bitmap(input: MapRef) -> CellMap {
    input
        .iter()
        .map(|line| {
            line.iter()
                .map(|&c| if c == 9 { Cell::B } else { Cell::U })
                .collect()
        })
        .collect()
}
