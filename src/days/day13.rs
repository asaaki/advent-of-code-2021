// Note: for part 2 the sum of dots is not important, but the printed result!

aoc_macros::day_impl_common!();

#[derive(Debug)]
enum Fold {
    Up(usize),
    Left(usize),
}

type Dot = (usize, usize);
type Dots = Vec<Dot>;

struct Transparent {
    width: usize,
    height: usize,
    dots: Dots,
}

impl Transparent {
    fn new(width: usize, height: usize, dots: Dots) -> Self {
        Self {
            width,
            height,
            dots,
        }
    }
}

fn compute(input: StrInputRef, first_part: bool) -> usize {
    let (dots, folds) = input.split_at(find_separator(input));
    let folds = make_folds(folds);
    let (width, height) = calculate_initial_size(&folds);

    let mut paper = Transparent::new(width, height, make_dots(dots));
    fold_it(first_part, &mut paper, &folds);

    if !first_part {
        for y in 0..=paper.height {
            for x in 0..=paper.width {
                let c = if paper.dots.contains(&(x, y)) {
                    '█'
                } else {
                    ' '
                };
                eprint!("{}", c);
            }
            eprintln!();
        }
    }

    paper.dots.sort_unstable();
    paper.dots.dedup();
    paper.dots.len()
}

#[inline]
fn find_separator(input: StrInputRef) -> usize {
    let separator =
        input.iter().enumerate().fold(
            0,
            |idx, (li, l)| {
                if l.is_empty() {
                    li
                } else {
                    idx
                }
            },
        );
    separator
}

#[inline]
fn fold_it(first_part: bool, paper: &mut Transparent, folds: &[Fold]) {
    let folds = if first_part { &folds[..1] } else { folds };
    for fold in folds {
        let dots_iter = paper.dots.iter_mut();
        match fold {
            Fold::Up(f) => {
                dots_iter.filter(|(_, y)| y > f).for_each(
                    |(_, y)| {
                        *y = paper.height - *y - 1;
                    },
                );
                paper.height = *f;
            }
            Fold::Left(f) => {
                dots_iter.filter(|(x, _)| x > f).for_each(
                    |(x, _)| {
                        *x = paper.width - *x - 1;
                    },
                );
                paper.width = *f;
            }
        }
    }
}

#[inline]
fn calculate_initial_size(folds: &[Fold]) -> (usize, usize) {
    folds[..2]
        .iter()
        .map(|f| match f {
            Fold::Up(y) => (0, y * 2 + 1),
            Fold::Left(x) => (x * 2 + 1, 0),
        })
        .fold((0, 0), |(w, h), (fw, fh)| (w + fw, h + fh))
}

#[inline]
fn make_folds(folds: StrInputRef) -> Vec<Fold> {
    let folds = &folds[1..];
    let folds: Vec<Fold> = folds
        .iter()
        .map(|line| {
            let (axis, value) = line[11..].split_once('=').unwrap();
            let value = value.parse().unwrap();
            match axis {
                "y" => Fold::Up(value),
                "x" => Fold::Left(value),
                _ => panic!("we can only fold in 2 directions!"),
            }
        })
        .collect();
    folds
}

#[inline]
fn make_dots(dots: StrInputRef) -> Vec<(usize, usize)> {
    let dots: Vec<(usize, usize)> = dots
        .iter()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    dots
}
