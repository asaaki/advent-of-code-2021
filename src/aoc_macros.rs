#[macro_export]
macro_rules! day_impl {
    ($step1:item, $step2:item) => {
        use super::{helpers::{s2t, StrInput}, AnyDay};
        use crate::utils::*;

        pub(super) struct Day<'a> {
            pub(super) input: StrInput<'a>,
        }

        impl AnyDay for Day<'_> {
            $step1

            $step2
        }
    };
}
