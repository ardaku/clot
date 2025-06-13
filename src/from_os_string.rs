use std::{
    error::Error,
    ffi::{CString, NulError, OsString},
    fmt,
    ops::Deref,
    path::PathBuf,
    str::{FromStr, Utf8Error},
    string::FromUtf8Error,
};

use specializer::Specializer;

/// Value that can be parsed from an [`OsString`]
///
/// This can be implemented manually, but is also implemented for all types that
/// implement [`FromStr`], with specialized behavior for:
///
///  - [`String`] - less allocation
///  - [`OsString`] - support non-UTF8 text
///  - [`CString`] - support non-UTF8 text
///  - [`PathBuf`] - support non-UTF8 paths
pub trait FromOsString: Sized {
    /// The associated error which can be returned from parsing
    type Err;

    /// Parses an [`OsString`] to return a value of this type.
    fn from_os_string(s: OsString) -> Result<Self, Self::Err>;
}

impl<T> FromOsString for T
where
    T: Sized + FromStr + 'static,
{
    type Err = FromStrError<<T as FromStr>::Err>;

    fn from_os_string(s: OsString) -> Result<Self, Self::Err> {
        type SpecializedResult<T> =
            Result<T, FromStrError<<T as FromStr>::Err>>;

        Specializer::new(s, |s: OsString| -> SpecializedResult<Self> {
            <&str>::try_from(s.deref())
                .map_err(FromStrError::Utf8)?
                .parse()
                .map_err(FromStrError::FromStr)
        })
        .specialize_return(|s| -> SpecializedResult<String> {
            String::from_utf8(s.into_encoded_bytes())
                .map_err(FromStrError::FromUtf8)
        })
        .specialize_return(|s| -> SpecializedResult<OsString> { Ok(s) })
        .specialize_return(|s| -> SpecializedResult<PathBuf> { Ok(s.into()) })
        .specialize_return(|s| -> SpecializedResult<CString> {
            Ok(CString::new(s.into_encoded_bytes()).unwrap())
        })
        .run()
    }
}

/// Error for implementation of [`FromOsString`] for `T`, where `T` implements
/// [`FromStr`]
#[derive(Debug)]
#[non_exhaustive]
pub enum FromStrError<E> {
    FromStr(E),
    FromUtf8(FromUtf8Error),
    Utf8(Utf8Error),
    Nul(NulError),
}

impl<E> fmt::Display for FromStrError<E>
where
    E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FromStr(e) => e.fmt(f),
            Self::FromUtf8(e) => e.fmt(f),
            Self::Utf8(e) => e.fmt(f),
            Self::Nul(e) => e.fmt(f),
        }
    }
}

impl<E> Error for FromStrError<E> where E: Error {}
