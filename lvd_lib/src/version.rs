//! Basic type-versioning utilities.

use binrw::{BinRead, BinWrite, binrw};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The wrapper type for a versioned, non-primitive type.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug)]
pub struct Versioned<T: Version> {
    /// The version number of the wrapped value.
    #[br(temp)]
    #[bw(calc = inner.version())]
    version: u8,

    /// The wrapped value.
    #[br(args(version))]
    pub inner: T,
}

/// A trait for determining a type's version.
pub trait Version
where
    Self: BinRead + BinWrite,
    Self: for<'a> BinRead<Args<'a> = (u8,)>,
    Self: for<'a> BinWrite<Args<'a> = ()>,
{
    /// Returns the version number from `self`.
    fn version(&self) -> u8;
}
