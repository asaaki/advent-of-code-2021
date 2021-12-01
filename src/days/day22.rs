use super::AnyDay;
use crate::utils::*;

pub(crate) struct Day {
    pub(super) input: Vec<String>,
}

impl AnyDay for Day {
    fn step1(&self) -> StringResult {
        dbg!(&self.input);
        Ok("[step1] not implemented yet".into())
    }

    fn step2(&self) -> StringResult {
        dbg!(&self.input);
        Ok("[step2] not implemented yet".into())
    }
}
