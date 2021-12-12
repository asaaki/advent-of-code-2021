use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    result::Result,
};

use shadow_rs::shadow;
shadow!(build);

// Q&D way of catching all errors without worrying about their types at compile time
// @see https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/boxing_errors.html
pub(crate) type GenericResult<T> = Result<T, Box<dyn Error>>;

// type alias to make it reaaaaalllly concise
pub(crate) type NullResult = GenericResult<()>;

// pub(crate) type IoResult = std::io::Result<()>;

// for functions, which should return a result, but can never fail:
pub(crate) type OkResult<T> = Result<T, core::convert::Infallible>;

pub(crate) type StringResult = OkResult<String>;
#[derive(Debug, Clone)]
pub(crate) struct CustomError(pub(crate) String);

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "custom error raised: {}", self.0)
    }
}

impl Error for CustomError {}

pub(crate) type CustomErrorResult<T> = std::result::Result<T, CustomError>;

pub(crate) const DAY_VALUES: &[&str; 26] = aoc_proc_macros::day_str_values!();
pub(crate) const PART_VALUES: &[&str; 2] = &["1", "2"];

#[repr(u8)]
#[derive(Copy, Clone)]
pub(crate) enum Part {
    One = 1,
    Two = 2,
}

impl TryFrom<u8> for Part {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Part::One),
            2 => Ok(Part::Two),
            _ => Err("A day can have only 2 parts"),
        }
    }
}

// debug

pub(crate) fn print_debug() -> NullResult {
    //shadow-rs built in function

    println!("debug:                    {}", shadow_rs::is_debug()); // check if this is a debug build. e.g 'true/false'
    println!("branch:                   {}", shadow_rs::branch()); // get current project branch. e.g 'master/develop'
    println!("tag:                      {}", shadow_rs::tag()); // get current project tag. e.g 'v1.3.5'
    println!("git_clean:                {}", shadow_rs::git_clean()); // get current project clean. e.g 'true/false'
    println!("git_status_file:          {}", shadow_rs::git_status_file()); // get current project statue file. e.g '  * examples/builtin_fn.rs (dirty)'

    //shadow-rs built in const

    println!("build::version()          {}", build::version()); // the version (description binary detail information)
    println!("build::clap_version()     {}", build::clap_version()); // usually used by clap crates version() (description binary detail information)

    // generated const values

    println!("build::PKG_VERSION        {}", build::PKG_VERSION); // current package version. e.g. '1.3.15-beta2'
    println!("build::PKG_VERSION_MAJOR  {}", build::PKG_VERSION_MAJOR); //current package major version. e.g. '1'
    println!("build::PKG_VERSION_MINOR  {}", build::PKG_VERSION_MINOR); //current package minor version. e.g. '3'
    println!("build::PKG_VERSION_PATCH  {}", build::PKG_VERSION_PATCH); //current package minor version. e.g. '15'
    println!("build::PKG_VERSION_PRE    {}", build::PKG_VERSION_PRE); //current package minor version. e.g. 'beta2'
    println!("build::BRANCH             {}", build::BRANCH); // the branch, e.g. 'master'
    println!("build::TAG                {}", build::TAG); // the tag, e.g. 'v1.0.0'
    println!("build::SHORT_COMMIT       {}", build::SHORT_COMMIT); // short commit hash, e.g. '8405e28e'
    println!("build::COMMIT_HASH        {}", build::COMMIT_HASH); // full commit hash, e.g. '8405e28e64080a09525a6cf1b07c22fcaf71a5c5'
    println!("build::COMMIT_DATE        {}", build::COMMIT_DATE); // commit date, e.g. '2021-08-04 12:34:03 +00:00'
    println!("build::COMMIT_DATE_2822   {}", build::COMMIT_DATE_2822); // commit date, e.g. 'Thu, 24 Jun 2021 21:33:59 +0800'
    println!("build::COMMIT_DATE_3339   {}", build::COMMIT_DATE_3339); // commit date, e.g. '2021-06-24T21:33:59.972494+08:00'
    println!("build::COMMIT_AUTHOR      {}", build::COMMIT_AUTHOR); // commit author, e.g. 'baoyachi'
    println!("build::COMMIT_EMAIL       {}", build::COMMIT_EMAIL); // commit email, e.g. 'example@gmail.com'

    println!("build::BUILD_OS           {}", build::BUILD_OS); // the OS that built the binary, e.g. 'macos-x86_64'
    println!("build::BUILD_TARGET       {}", build::BUILD_TARGET); // the OS target that built the binary, e.g. 'x86_64-apple-darwin'
    println!("build::BUILD_TARGET_ARCH  {}", build::BUILD_TARGET_ARCH); // the OS target arch that built the binary, e.g. 'x86_64'
    println!("build::RUST_VERSION       {}", build::RUST_VERSION); // rustc version e.g. 'rustc 1.45.0 (5c1f21c3b 2020-07-13)'
    println!("build::RUST_CHANNEL       {}", build::RUST_CHANNEL); // rust toolchain e.g. 'stable-x86_64-apple-darwin (default)'
    println!("build::CARGO_VERSION      {}", build::CARGO_VERSION); // cargo version e.g. 'cargo 1.45.0 (744bd1fbb 2020-06-15)'

    // println!("(tree) {}", build::CARGO_TREE); // e.g. the output of '$ cargo tree'

    println!("build::PROJECT_NAME       {}", build::PROJECT_NAME); // your project name, e.g. 'shadow-rs'
    println!("build::BUILD_TIME         {}", build::BUILD_TIME); // time when start build occurred, e.g. '2020-08-16 14:50:25'
    println!("build::BUILD_TIME_2822    {}", build::BUILD_TIME_2822); // time when start build occurred by rfc2822, e.g. 'Thu, 24 Jun 2021 21:33:59 +0800'
    println!("build::BUILD_TIME_3339    {}", build::BUILD_TIME_3339); // time when start build occurred by rfc3339, e.g. '2021-06-24T21:33:59.972494+08:00'
    println!("build::BUILD_RUST_CHANNEL {}", build::BUILD_RUST_CHANNEL); // e.g. 'debug'
    println!("build::GIT_CLEAN          {}", build::GIT_CLEAN); // e.g. 'true'
    println!("build::GIT_STATUS_FILE    {}", build::GIT_STATUS_FILE); // e.g. '* src/lib.rs (dirty)'

    Ok(())
}
