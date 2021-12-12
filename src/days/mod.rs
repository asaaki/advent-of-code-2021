use self::helpers::{Str, StrInputRef};
use super::Part;
use crate::utils::*;
use std::time::Instant;

aoc_proc_macros::add_day_mods!();
pub(crate) mod helpers;

pub(crate) trait AnyDay {
    fn part(&self, part: Part) -> StringResult {
        match part {
            Part::One => self.part1(),
            Part::Two => self.part2(),
        }
    }

    fn part1(&self) -> StringResult;
    fn part2(&self) -> StringResult;
}

pub(crate) fn call_a_day(
    day: u8,
    part: Part,
    input: StrInputRef,
    tests: &(Option<Str>, Option<Str>),
) -> CustomErrorResult<String> {
    let maybe_day: Option<Box<dyn AnyDay>> =
        aoc_proc_macros::add_day_matches!();
    match maybe_day {
        Some(d) => {
            let instant = Instant::now();
            let result = d.part(part).expect("(infallible)");
            let runtime = instant.elapsed();
            let test = match part {
                Part::One => tests.0.as_ref(),
                Part::Two => tests.1.as_ref(),
            };

            if let Some(expected) = test {
                assert_eq!(&result, expected);
            }

            Ok(format!(
                "{} (and it took {:6.3} ms to calculate)",
                result,
                runtime.as_secs_f64() * 1000.0
            ))
        }
        None => Err(CustomError("invalid day".into())),
    }
}
