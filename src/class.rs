//! A "Class" is required for any parameters being parsed by clot.
//!
//! Classes should document the set of possible values for help generation.
//!
//!  - Is it a closed interval? `{PARAM_NAME} Integer[MIN, MAX]`
//!  - Is it an open interval? `<OPTIONAL_PARAM_NAME> Number(MIN, MAX)`
//!  - Is it from a defined set?
//!     ```text
//!     {PARAM_NAME}
//!     | A
//!     | B
//!     | C
//!     ```
//!  - Is it a list? `<PARAM_NAME, …>`
//!  - Is it a list of at least one? `{PARAM_NAME, …}`

use std::{
    fmt::{self, Display},
    marker::PhantomData,
};

use ranch::range::Range;

/// Any floating point number - Excludes infinity and NaN
#[allow(dead_code)]
pub struct Number<T, R: Range<T>>(T, PhantomData<fn() -> R>);

impl<T, R> Class for Number<T, R>
where
    R: Range<T>,
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Specialize for full range default generic
        let min = R::MIN;
        let max = R::MAX;

        write!(f, "Number[{min}, {max}]")
    }
}

#[allow(dead_code)]
pub trait Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl<T> Class for T
where
    T: Range + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let min = T::MIN;
        let max = T::MAX;

        write!(f, "Integer[{min}, {max}]")
    }
}
