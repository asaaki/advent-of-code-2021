use self::helpers::{Str, StrInput};
use crate::utils::*;

mod day00;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
pub(crate) mod helpers;

trait AnyDay {
    fn step(&self, i: u8) -> StringResult {
        match i {
            1 => self.step1(),
            2 => self.step2(),
            _ => Err(CustomError("invalid step".into())),
        }
    }

    fn step1(&self) -> StringResult;
    fn step2(&self) -> StringResult;
}

pub(crate) fn call_a_day<'a>(
    day: u8,
    step: u8,
    input: StrInput,
    test: Option<Str>,
) -> StringResult {
    let maybe_day: Option<Box<dyn AnyDay>> = match day {
        0 => Some(Box::new(day00::Day { input })),
        1 => Some(Box::new(day01::Day { input })),
        2 => Some(Box::new(day02::Day { input })),
        3 => Some(Box::new(day03::Day { input })),
        4 => Some(Box::new(day04::Day { input })),
        5 => Some(Box::new(day05::Day { input })),
        6 => Some(Box::new(day06::Day { input })),
        7 => Some(Box::new(day07::Day { input })),
        8 => Some(Box::new(day08::Day { input })),
        9 => Some(Box::new(day09::Day { input })),
        10 => Some(Box::new(day10::Day { input })),
        11 => Some(Box::new(day11::Day { input })),
        12 => Some(Box::new(day12::Day { input })),
        13 => Some(Box::new(day13::Day { input })),
        14 => Some(Box::new(day14::Day { input })),
        15 => Some(Box::new(day15::Day { input })),
        16 => Some(Box::new(day16::Day { input })),
        17 => Some(Box::new(day17::Day { input })),
        18 => Some(Box::new(day18::Day { input })),
        19 => Some(Box::new(day19::Day { input })),
        20 => Some(Box::new(day20::Day { input })),
        21 => Some(Box::new(day21::Day { input })),
        22 => Some(Box::new(day22::Day { input })),
        23 => Some(Box::new(day23::Day { input })),
        24 => Some(Box::new(day24::Day { input })),
        25 => Some(Box::new(day25::Day { input })),
        _ => None,
    };

    match maybe_day {
        Some(d) => {
            let result = d.step(step)?;
            test.map(|expected| assert_eq!(result, expected));
            Ok(result)
        }
        None => Err(CustomError("invalid day".into())),
    }
}
