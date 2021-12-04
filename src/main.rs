use days::helpers::StrInput;
use std::{
    borrow::Cow,
    convert::TryFrom,
    env::current_dir,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};
use utils::*;

mod days;
mod utils;

const DAY_VALUES: &[&str; 26] = &[
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
    "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25",
];

const PART_VALUES: &[&str; 2] = &["1", "2"];

#[repr(u8)]
#[derive(Copy, Clone)]
enum Part {
    One = 1,
    Two = 2,
}

impl TryFrom<u8> for Part {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Part::One),
            2 => Ok(Part::Two),
            _ => Err("A day can have only 2 parts"),
        }
    }
}

#[kommand::main]
fn main(
    #[kommand(possible_values = DAY_VALUES)] day: u8,
    #[kommand(possible_values = PART_VALUES)] part: u8,
    #[kommand(short = 't', long)] test: bool,
    #[kommand(short = 'd', long)] debug: bool,
) -> NullResult {
    if debug {
        return print_debug();
    };

    println!("[AoC'21] Day: {}, part: {}, test? {}", day, part, test);

    let input = {
        let test_or_challenge = if test { "test" } else { "challenge" };
        let rel_input: PathBuf =
            format!("inputs/{}.{}.txt", day, test_or_challenge)
                .parse()
                .expect("arguments to form a valid path string");
        let mut input_path = current_dir()?;
        input_path.push(rel_input);
        File::open(input_path)?
    };

    let mut input: StrInput = BufReader::new(input)
        .lines()
        .filter_map(Result::ok)
        .map(Cow::from)
        .collect();

    let tests = if test {
        let test2 = input.pop();
        let test1 = input.pop();
        (test1, test2)
    } else { (None, None) };

    let part = Part::try_from(part)?;
    let result = days::call_a_day(day, part, &input, tests)?;
    println!("Result is: {}", result);

    Ok(())
}
