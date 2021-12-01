use std::borrow::Cow;
use std::str::FromStr;

pub(crate) type Str<'a> = Cow<'a, str>;
// when creating the collection
pub(crate) type StrInput<'a> = Vec<Str<'a>>;
// … and when passing it around
pub(crate) type StrInputRef<'a> = &'a [Str<'a>];

pub(crate) fn s2t<T: FromStr, S: AsRef<str>>(input: &[S]) -> Vec<T> {
    input
        .iter()
        .map(|s| s.as_ref().parse::<T>())
        .filter_map(Result::ok)
        .collect()
}
