#[derive(Debug)]
enum Movement {
    Forward(usize),
    Up(usize),
    Down(usize),
}

use Movement::*;

aoc_macros::day_impl_common!();

fn compute(input: StrInputRef, simple_move: bool) -> usize {
    let mut state = State::default();
    for s in input {
        let m = Movement::parse(s);
        if simple_move {
            state.r#move(m)
        } else {
            state.aimed_move(m)
        }
    }
    state.product()
}

impl AsRef<Movement> for Movement {
    #[inline]
    fn as_ref(&self) -> &Movement {
        self
    }
}

impl Movement {
    fn parse<S: AsRef<str>>(input: S) -> Self {
        let parts: Vec<&str> = input.as_ref().split(' ').collect();
        let (m, i) = (
            parts[0],
            parts[1].parse().expect("must be a non-neg number"),
        );

        match m {
            "forward" => Forward(i),
            "up" => Up(i),
            "down" => Down(i),
            _ => panic!("unrecognized movement string"),
        }
    }
}

#[derive(Debug, Default)]
struct State {
    depth: usize,
    horizontal: usize,
    aim: usize,
}

impl State {
    // r#move, because `move` is a keyword
    // (r#… escapes it for non-keyword use)
    fn r#move<M: AsRef<Movement>>(&mut self, movement: M) {
        match movement.as_ref() {
            Forward(i) => self.horizontal += i,
            Up(i) => self.depth -= i,
            Down(i) => self.depth += i,
        };
    }

    fn aimed_move<M: AsRef<Movement>>(&mut self, movement: M) {
        match movement.as_ref() {
            Forward(i) => {
                self.horizontal += i;
                self.depth += self.aim * i;
            }
            Up(i) => self.aim -= i,
            Down(i) => self.aim += i,
        };
    }

    fn product(&self) -> usize {
        self.depth * self.horizontal
    }
}
