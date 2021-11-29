use crate::utils::*;
use super::AnyDay;

pub(crate) struct Day { pub(super) test: Option<String>, pub(super) input: Vec<String> }

impl AnyDay for Day {
  fn step1(&self) -> StringResult {
    dbg!(&self.input);
    dbg!(&self.test);
    Ok("[step1] not implemented yet".into())
  }

  fn step2(&self) -> StringResult {
    dbg!(&self.input);
    dbg!(&self.test);
    Ok("[step2] not implemented yet".into())
  }
}
