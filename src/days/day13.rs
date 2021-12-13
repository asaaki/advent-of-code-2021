// Note: for part 2 the sum of dots is not important, but the printed result!

aoc_macros::day_impl_common!();

#[derive(Debug)]
enum Fold {
    Up(usize),
    Left(usize),
}

type Paper<'a> = MatrixV<'a, usize>;

fn compute(input: StrInputRef, first_part: bool) -> usize {
    let (dots, folds) = input.split_at(find_separator(input));
    let dots = make_dots(dots);
    let folds = make_folds(folds);
    let (w, h) = calculate_initial_size(&folds);

    let mut paper = Paper::new(w);
    paper.fill_iter(std::iter::repeat(0).take(w * h));
    for (x, y) in dots {
        paper.insert(x, y, 1);
    }

    fold_it(first_part, &mut paper, &folds);

    if !first_part {
        for line in paper.iter_rows() {
            for dot in line {
                let c = if dot > &0 { '█' } else { ' ' };
                eprint!("{}", c);
            }
            eprintln!();
        }
    }

    paper.view().iter().sum()
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
fn fold_it(first_part: bool, paper: &mut Paper, folds: &[Fold]) {
    let folds = if first_part { &folds[..1] } else { folds };
    for fold in folds {
        match fold {
            Fold::Up(f) => {
                fold_up(paper, f);
            }
            Fold::Left(f) => {
                paper.transpose();
                fold_up(paper, f);
                paper.transpose();
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

#[inline]
fn fold_up(paper: &mut MatrixV<usize>, f: &usize) {
    let data = paper.view();
    let cs = paper.chunk_size();
    let (up, down) = (&data[..(f * cs)], &data[((f + 1) * cs)..]);
    let merged: Vec<usize> = up
        .chunks_exact(cs)
        .zip(down.chunks_exact(cs).rev())
        .flat_map(|(l, r)| {
            l.iter().zip(r.iter()).map(|(a, b)| a.max(b).to_owned())
        })
        .collect();
    let mut folded = Paper::new(cs);
    folded.fill(&merged);
    *paper = folded;
}
