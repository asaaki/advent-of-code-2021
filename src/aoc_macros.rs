#[macro_export]
macro_rules! day_impl {
    ($step1:item, $step2:item) => {
        use super::{helpers::{s2t, StrInputRef}, AnyDay};
        use crate::utils::*;

        pub(super) struct Day<'a> {
            pub(super) input: StrInputRef<'a>,
        }

        impl AnyDay for Day<'_> {
            $step1

            $step2
        }
    };
}
