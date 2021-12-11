use days::helpers::{Cow, StrInput};
use rust_embed::RustEmbed;
use std::{
    convert::TryFrom,
    io::{BufRead, Cursor},
};
use utils::*;

#[derive(RustEmbed)]
#[folder = "inputs"]
struct Inputs;

mod data;
mod days;
mod utils;

const DAY_VALUES: &[&str; 26] = aoc_proc_macros::day_str_values!();
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
        Inputs::get(&format!("{}.{}.txt", day, test_or_challenge))
            .expect("input file to be stored in binary")
            .data
    };

    let mut input: StrInput = Cursor::new(&input)
        .lines()
        .filter_map(Result::ok)
        .map(Cow::from)
        .collect();

    let tests = if test {
        let test2 = input.pop();
        let test1 = input.pop();
        (test1, test2)
    } else {
        (None, None)
    };

    let part = Part::try_from(part)?;
    let result = days::call_a_day(day, part, &input, &tests)?;
    println!("Result is: {}", result);

    Ok(())
}
