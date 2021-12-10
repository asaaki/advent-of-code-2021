#[macro_export]
macro_rules! day_impl {
    ($step1:item, $step2:item) => {
        use super::{helpers::{ok_string, s2t, StrInputRef}, AnyDay};
        use crate::{data::*, utils::*};

        pub(super) struct Day<'a> {
            pub(super) input: StrInputRef<'a>,
        }

        impl AnyDay for Day<'_> {
            $step1

            $step2
        }
    };
}

#[macro_export]
macro_rules! day_impl_common {
    () => {
        $crate::day_impl! {
            fn part1(&self) -> StringResult {
                let result = compute(self.input, true);
                ok_string(result)
            },

            fn part2(&self) -> StringResult {
                let result = compute(self.input, false);
                ok_string(result)
            }
        }
    };
}

#[macro_export]
macro_rules! day_impl_compute {
    () => {
        fn compute(input: StrInputRef, first_part: bool) -> usize {
            if first_part {
                part_1_(input)
            } else {
                part_2_(input)
            }
        }
    };
}

#[macro_export]
macro_rules! day_impl_common_and_compute {
    () => {
        $crate::day_impl_common!();
        $crate::day_impl_compute!();
    };
}
