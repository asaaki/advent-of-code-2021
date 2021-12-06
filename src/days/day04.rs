aoc_macros::day_impl! {
    fn part1(&self) -> StringResult {
        let (draws, boards) = parse_input(self.input);
        let result = run_draws(draws, boards);
        ok_string(result)
    },

    fn part2(&self) -> StringResult {
        let (draws, boards) = parse_input(self.input);
        let result = run_draws_until_last_win(draws, boards);
        ok_string(result)
    }
}

fn parse_input(input: StrInputRef) -> (Vec<usize>, Vec<Board>) {
    let draws: Vec<usize> = input[0]
        .split(',')
        .map(|s| s.parse::<usize>().expect("not a number"))
        .collect();

    let mut boards: Vec<Board> = vec![];
    let mut slices: Vec<Vec<usize>> = vec![];
    let mut current = Some(Board::new());

    for line in input.iter().skip(2) {
        if line.is_empty() {
            let finished = current.replace(Board::new());
            if let Some(mut board) = finished {
                board.fill(&slices);
                boards.push(board);
            }
            slices.truncate(0);
        } else {
            let nums: Vec<usize> = line
                .split_whitespace()
                .map(|s| s.parse::<usize>().expect("not a number"))
                .collect();
            slices.push(nums);
        }
    }
    // closing the last board, as there is no empty line to trigger
    if let Some(mut board) = current {
        board.fill(&slices);
        boards.push(board);
    }

    (draws, boards)
}

fn run_draws(draws: Vec<usize>, mut boards: Vec<Board>) -> usize {
    let mut value = 0;
    'draws: for draw in draws {
        for board in boards.iter_mut() {
            board.mark(&draw);
        }
        for board in boards.iter() {
            if board.check_win() {
                let usum = board.unmark_sum();
                value = usum * draw;
                break 'draws;
            }
        }
    }
    value
}

fn run_draws_until_last_win(
    draws: Vec<usize>,
    mut boards: Vec<Board>,
) -> usize {
    let mut last_winner: Option<(usize, usize)> = None;

    let skipmarks = vec![false; boards.len()];
    let mut state: Vec<(&mut Board, bool)> =
        boards.iter_mut().zip(skipmarks.into_iter()).collect();
    for draw in draws {
        for (board, skip) in state.iter_mut() {
            if skip == &false {
                board.mark(&draw);
            }
        }

        for (i, (board, skip)) in state.iter_mut().enumerate() {
            if skip == &false && board.check_win() {
                *skip = true;
                last_winner.replace((draw, i));
            }
        }
    }

    last_winner
        .map(|(draw, idx)| boards.get(idx).map(|b| b.unmark_sum() * draw))
        .flatten()
        .expect("value to be a number")
}

const fn board_size(dim: usize) -> usize {
    dim * dim
}

const DIM: usize = 5;
const SIZE: usize = board_size(DIM);

#[derive(Debug)]
struct Board {
    numbers: [usize; SIZE],
    marks: [usize; SIZE],
}

impl Board {
    fn new() -> Self {
        // NOTE: `0` is a valid number, but we initialize with all marks as unmarked(1),
        //       so no zero has been drawn yet;
        //       marks: 1=unmarked, 0=marked -> makes summarizing easier
        Board {
            numbers: [0; SIZE],
            marks: [1; SIZE],
        }
    }

    fn fill(&mut self, input: &[Vec<usize>]) {
        input.iter().flatten().enumerate().for_each(|(i, &v)| {
            self.numbers[i] = v;
        });
    }

    fn mark(&mut self, number: &usize) {
        for (i, n) in self.numbers.iter().enumerate() {
            if n == number {
                self.marks[i] = 0;
            }
        }
    }

    fn check_win(&self) -> bool {
        let mut win = false;
        // rows
        for row in self.marks.chunks_exact(DIM) {
            win = win || row.iter().all(|b| b == &0);
        }
        for offset in 0..DIM {
            win = win
                || self.marks.iter().skip(offset).step_by(DIM).all(|m| m == &0)
        }

        win
    }

    // to sum all unmarked, we only need to multiply the number with the mark,
    // since 1=unmarked and 0=marked
    fn unmark_sum(&self) -> usize {
        self.numbers
            .iter()
            .zip(self.marks.iter())
            .fold(0usize, |s, (n, m)| s + (n * m))
    }
}
