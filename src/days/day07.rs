type Signed = i32;

aoc_macros::day_impl_common!();

fn compute(input: StrInputRef, not_triangular: bool) -> Signed {
    let positions: Vec<Signed> =
        input[0].split(',').map(|s| s.parse().unwrap()).collect();
    let (&min, &max) = (
        positions.iter().min().unwrap(),
        positions.iter().max().unwrap(),
    );
    let mut fuel = Signed::MAX;
    for pos in min..=max {
        let pos_fuel = positions.iter().fold(0, |fuel, crab| {
            if !not_triangular {
                // the sequence goes 1,3,6,10,… and when you search for it you'll find
                // the triangular number sequence, and its formula is f(x) = 1/2 * x * (x + 1)
                fuel + ((crab - pos).abs() * ((crab - pos).abs() + 1) / 2)
            } else {
                fuel + (crab - pos).abs()
            }
        });
        fuel = fuel.min(pos_fuel);
    }
    fuel
}
