use crate::days::helpers::{Cow, StrInput};
use rust_embed::RustEmbed;
use std::io::{BufRead, Cursor};

#[derive(RustEmbed)]
#[folder = "inputs"]
pub(crate) struct Inputs;

type Test<'a> = Option<Cow<'a, str>>;
type Tests<'a> = (Test<'a>, Test<'a>);

impl Inputs {
    pub(crate) fn get_for_day<'a>(day: u8, test: bool) -> StrInput<'a> {
        let input = {
            let test_or_challenge = if test { "test" } else { "challenge" };
            Inputs::get(&format!("{}.{}.txt", day, test_or_challenge))
                .expect("input file to be stored in binary")
                .data
        };

        Cursor::new(&input)
            .lines()
            .filter_map(Result::ok)
            .map(Cow::from)
            .collect()
    }

    pub(crate) fn get_for_day_with_tests<'a>(
        day: u8,
        test: bool,
    ) -> (StrInput<'a>, Tests<'a>) {
        let mut input = Self::get_for_day(day, test);

        let tests = if test {
            let test2 = input.pop();
            let test1 = input.pop();
            (test1, test2)
        } else {
            (None, None)
        };

        (input, tests)
    }
}
