use super::{helpers::StrInput, AnyDay};
use crate::utils::*;

pub(crate) struct Day<'a> {
    pub(super) input: StrInput<'a>,
}

impl AnyDay for Day<'_> {
    fn step1(&self) -> StringResult {
        dbg!(&self.input);
        Ok("[step1] not implemented yet".into())
    }

    fn step2(&self) -> StringResult {
        dbg!(&self.input);
        Ok("[step2] not implemented yet".into())
    }
}
