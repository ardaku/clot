use std::{
    convert::Infallible,
    ffi::OsStr,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8,
        NonZeroIsize, NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64,
        NonZeroU8, NonZeroUsize,
    },
    path::Path,
    str::FromStr,
};

/// Autogenerate `FromOsStr` impl from `FromStr` impl
macro_rules! from_os_str {
    ($from_str:ty) => {
        impl<'a> FromOsStr<'a> for $from_str {
            type Err = Error<<Self as FromStr>::Err>;

            fn from_str(s: &'a OsStr) -> Result<Self, Self::Err> {
                Ok(s.to_str().ok_or(Error::Utf8)?.parse()?)
            }
        }
    };
}

pub struct Utf8Error;

pub enum Error<E> {
    Generic(E),
    Utf8,
}

impl<E> From<E> for Error<E> {
    fn from(generic: E) -> Self {
        Self::Generic(generic)
    }
}

/// Parse a value from an [`OsStr`].
///
/// Similar to [`FromStr`], but on `&OsStr` instead of `&str`, and borrowed
/// when possible.
pub trait FromOsStr<'a>: Sized {
    type Err;

    fn from_str(s: &'a OsStr) -> Result<Self, Self::Err>;
}

from_os_str!(IpAddr);
from_os_str!(SocketAddr);
from_os_str!(Ipv4Addr);
from_os_str!(Ipv6Addr);
from_os_str!(bool);
from_os_str!(char);
from_os_str!(f32);
from_os_str!(f64);
from_os_str!(i8);
from_os_str!(i16);
from_os_str!(i32);
from_os_str!(i64);
from_os_str!(i128);
from_os_str!(isize);
from_os_str!(u8);
from_os_str!(u16);
from_os_str!(u32);
from_os_str!(u64);
from_os_str!(u128);
from_os_str!(usize);
from_os_str!(NonZeroI8);
from_os_str!(NonZeroI16);
from_os_str!(NonZeroI32);
from_os_str!(NonZeroI64);
from_os_str!(NonZeroI128);
from_os_str!(NonZeroIsize);
from_os_str!(NonZeroU8);
from_os_str!(NonZeroU16);
from_os_str!(NonZeroU32);
from_os_str!(NonZeroU64);
from_os_str!(NonZeroU128);
from_os_str!(NonZeroUsize);

impl<'a> FromOsStr<'a> for &'a str {
    type Err = Utf8Error;

    fn from_str(s: &'a OsStr) -> Result<Self, Self::Err> {
        s.to_str().ok_or(Utf8Error)
    }
}

impl<'a> FromOsStr<'a> for &'a OsStr {
    type Err = Infallible;

    fn from_str(s: &'a OsStr) -> Result<Self, Self::Err> {
        Ok(s)
    }
}

impl<'a> FromOsStr<'a> for &'a Path {
    type Err = Infallible;

    fn from_str(s: &'a OsStr) -> Result<Self, Self::Err> {
        Ok(s.as_ref())
    }
}
