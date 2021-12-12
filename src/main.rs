use inputs::Inputs;
use std::convert::TryFrom;
use utils::*;

mod data;
mod days;
mod inputs;
mod utils;

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
    let (input, tests) = Inputs::get_for_day_with_tests(day, test);

    let result = days::call_a_day(day, Part::try_from(part)?, &input, &tests)?;
    println!("\nResult is: {}\n\n", result);
    Ok(())
}
