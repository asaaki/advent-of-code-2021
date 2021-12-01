use super::AnyDay;
use crate::utils::*;

pub(crate) struct Day {
    pub(super) input: Vec<String>,
}

impl AnyDay for Day {
    fn step1(&self) -> StringResult {
        let depths: &Vec<usize> = &self
            .input
            .iter()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let result: usize = depths
            .windows(2)
            .fold(0, |a, t| if t[1] > t[0] { a + 1 } else { a });

        Ok(format!("{}", result))
    }

    fn step2(&self) -> StringResult {
        let depths: &Vec<usize> = &self
            .input
            .iter()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let sums: Vec<usize> = depths.windows(3).map(|w| w.iter().sum()).collect();

        let result: usize = sums
            .windows(2)
            .fold(0, |a, t| if t[1] > t[0] { a + 1 } else { a });

        Ok(format!("{}", result))
    }
}
