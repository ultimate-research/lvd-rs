//! Types and traits for working with versioned types.

use binrw::{binrw, BinRead, BinWrite};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The wrapper type for a versioned, non-primitive type.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug)]
pub struct Versioned<T>
where
    T: BinRead + BinWrite + Version,
    T: for<'a> BinRead<Args<'a> = (u8,)>,
    T: for<'a> BinWrite<Args<'a> = ()>,
{
    /// The version number of the wrapped value.
    #[br(temp)]
    #[bw(calc = inner.version())]
    version: u8,

    /// The wrapped value.
    #[br(args(version))]
    pub inner: T,
}

/// A trait for determining a type's version.
pub trait Version {
    /// Returns the version number from `self`.
    fn version(&self) -> u8;
}
