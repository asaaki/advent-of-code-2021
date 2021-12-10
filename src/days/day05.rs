aoc_macros::day_impl_common!();

fn compute(input: StrInputRef, part_one: bool) -> usize {
    let vents = parse_vents(input, !part_one);
    let max = vents.iter().flatten().map(|(x, y)| x.max(y)).max().unwrap();
    let mut map: MatrixV<usize> = MatrixV::new(max + 1); // if v{0-9} -> 10x10 map
    map.fill_square(0);

    for vent in vents {
        for (x, y) in vent {
            let v = map.get(x, y).expect("why you no value?") + 1;
            map.insert(x, y, v);
        }
    }
    map.view().iter().filter(|&v| v > &1).count()
}

type Coord = (usize, usize);

fn parse_vents(
    input: StrInputRef,
    diagonals: bool,
) -> Vec<Vec<(usize, usize)>> {
    input
        .iter()
        .map(|vent| {
            let parts: Vec<&str> = vent.split_whitespace().collect();
            let (start, end) = (parse_coord(parts[0]), parse_coord(parts[2]));
            endpoints2lines(start, end, diagonals)
        })
        .collect()
}

fn parse_coord(s: &str) -> Coord {
    let parts: Vec<usize> =
        s.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
    (parts[0], parts[1])
}

fn endpoints2lines(start: Coord, end: Coord, diagonals: bool) -> Vec<Coord> {
    if start.0 == end.0 {
        let ((y1, y2), _rev) = order((start.1, end.1));
        (y1..=y2).map(|y| (start.0, y)).collect()
    } else if start.1 == end.1 {
        let ((x1, x2), _rev) = order((start.0, end.0));
        (x1..=x2).map(|x| (x, start.1)).collect()
    } else if diagonals {
        /*
            simple growing: 0,0->1,1 | x=(0..1), y=(0..1)
                decreasing: 0,1->1,0 | x=(0..1), Y=(1..0)
                            1,0->0,1 | X=(1..0), y=(0..1)
                            1,1->0,0 | X=(1..0), Y=(1..0)

            x+,y- -> rx.zip(ry.rev())
            x-,y+ -> rx.rev().zip(ry)
            x-,y- -> rx.rev().zip(ry.rev())
        */
        let ((x1, x2), xrev) = order((start.0, end.0));
        let ((y1, y2), yrev) = order((start.1, end.1));

        let rx = x1..=x2;
        let ry = y1..=y2;

        match (xrev, yrev) {
            (false, false) => rx.zip(ry).collect(),
            (false, true) => rx.zip(ry.rev()).collect(),
            (true, false) => rx.rev().zip(ry).collect(),
            (true, true) => rx.rev().zip(ry.rev()).collect(),
        }
    } else {
        vec![]
    }
}

fn order(input: Coord) -> (Coord, bool) {
    if input.0 < input.1 {
        (input, false)
    } else {
        ((input.1, input.0), true)
    }
}
