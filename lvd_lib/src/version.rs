//! Types and traits for working with versioned types.
use binrw::{binrw, BinRead, BinWrite};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A versioned non-primitive type.
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
    #[br(temp)]
    #[bw(calc = data.version())]
    version: u8,

    /// Associated data for the type's version.
    #[br(args(version))]
    pub data: T,
}

/// A trait for determining a type's version.
pub trait Version {
    /// Returns a type's version.
    fn version(&self) -> u8;
}
