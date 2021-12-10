use proc_macro::TokenStream;
use std::str::FromStr;

const DAYS: std::ops::RangeInclusive<usize> = 0..=25;

#[proc_macro]
pub fn day_str_values(_: TokenStream) -> TokenStream {
    let days = DAYS
        .map(|d| format!("\"{}\"", d))
        .collect::<Vec<String>>()
        .join(", ");
    let code = format!("&[{}]", days);
    TokenStream::from_str(&code).unwrap()
}

#[proc_macro]
pub fn add_day_mods(_: TokenStream) -> TokenStream {
    let days = DAYS
        .map(|d| format!("mod day{:02};", d))
        .collect::<Vec<String>>()
        .join("");
    TokenStream::from_str(&days).unwrap()
}

#[proc_macro]
pub fn add_day_matches(_: TokenStream) -> TokenStream {
    let lines = DAYS
        .map(|d| {
            format!("{} => Some(Box::new(day{:02}::Day {{ input }})),", d, d)
        })
        .collect::<Vec<String>>()
        .join("");
    let code = format!(
        "match day {{
        {}
        _ => None,
    }}",
        lines
    );
    TokenStream::from_str(&code).unwrap()
}
