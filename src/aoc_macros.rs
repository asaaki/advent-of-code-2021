// https://github.com/asaaki/advent-of-code-2020/blob/main/src/aoc_macros.rs
#[macro_export]
macro_rules! day_for_2020 {
    ($mod:ident, $step:ident, $data:ident, $expected:ident) => {{
        if let Some(expected_value) = $expected {
            days::$mod::run_test($step, &$data, expected_value)?;
        } else {
            days::$mod::run($step, &$data)?;
        }
        Ok(())
    }};
}
