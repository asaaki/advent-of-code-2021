use crate::utils::*;

mod day0;
mod day1;
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
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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

pub(crate) fn call_a_day(
    day: u8,
    step: u8,
    input: Vec<String>,
    test: Option<String>,
) -> StringResult {
    let maybe_day: Option<Box<dyn AnyDay>> = match day {
        0 => Some(Box::new(day0::Day { input, test })),
        1 => Some(Box::new(day1::Day { input, test })),
        2 => Some(Box::new(day2::Day { input, test })),
        3 => Some(Box::new(day3::Day { input, test })),
        4 => Some(Box::new(day4::Day { input, test })),
        5 => Some(Box::new(day5::Day { input, test })),
        6 => Some(Box::new(day6::Day { input, test })),
        7 => Some(Box::new(day7::Day { input, test })),
        8 => Some(Box::new(day8::Day { input, test })),
        9 => Some(Box::new(day9::Day { input, test })),
        10 => Some(Box::new(day10::Day { input, test })),
        11 => Some(Box::new(day11::Day { input, test })),
        12 => Some(Box::new(day12::Day { input, test })),
        13 => Some(Box::new(day13::Day { input, test })),
        14 => Some(Box::new(day14::Day { input, test })),
        15 => Some(Box::new(day15::Day { input, test })),
        16 => Some(Box::new(day16::Day { input, test })),
        17 => Some(Box::new(day17::Day { input, test })),
        18 => Some(Box::new(day18::Day { input, test })),
        19 => Some(Box::new(day19::Day { input, test })),
        20 => Some(Box::new(day20::Day { input, test })),
        21 => Some(Box::new(day21::Day { input, test })),
        22 => Some(Box::new(day22::Day { input, test })),
        23 => Some(Box::new(day23::Day { input, test })),
        24 => Some(Box::new(day24::Day { input, test })),
        25 => Some(Box::new(day25::Day { input, test })),
        _ => None,
    };

    match maybe_day {
        Some(d) => d.step(step),
        None => Err(CustomError("invalid day".into())),
    }
}
