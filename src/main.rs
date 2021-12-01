#![allow(dead_code)]

use std::{
    env::current_dir,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};
use utils::*;

mod days;
mod utils;

#[kommand::main]
fn main(
    day: u8,
    step: u8,
    #[kommand(short = 't', long)] test: bool,
    #[kommand(short = 'd', long)] debug: bool,
) -> NullResult {
    println!("[AoC'21] Day: {}, Step: {}, test? {}", day, step, test);

    if debug {
        print_debug()?;
        return Ok(());
    };

    let input = {
        let maybe_test = if test { "test" } else { "" };
        let rel_input: PathBuf = format!("inputs/{}.{}.{}.txt", day, step, maybe_test)
            .parse()
            .expect("arguments to form a valid path string");
        let mut input_path = current_dir()?;
        input_path.push(rel_input);
        File::open(input_path)?
    };

    let mut input: Vec<String> = BufReader::new(input)
        .lines()
        .filter_map(Result::ok)
        .collect();

    let test = test.then(|| input.pop()).flatten();
    // or without flatten; not really better:
    // let test = test.then(|| ()).and_then(|_| input.pop());

    let res = days::call_a_day(day, step, input, test)?;
    println!("Result is: {}", res);

    Ok(())
}
