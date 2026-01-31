//! Basic type-versioning utilities.

use std::ops::{Deref, DerefMut};

use binrw::{BinRead, BinWrite, binrw};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The wrapper type for a versioned, non-primitive type.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Default)]
#[repr(transparent)]
pub struct Versioned<T: Version> {
    /// The version number of the wrapped value.
    #[br(temp)]
    #[bw(calc = inner.version())]
    version: u8,

    /// The wrapped value.
    #[br(args(version))]
    pub inner: T,
}

impl<T: Version> Versioned<T> {
    /// Creates a new `Versioned<T>`.
    pub const fn new(value: T) -> Self {
        Self { inner: value }
    }
}

impl<T: Version> Deref for Versioned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Version> DerefMut for Versioned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
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
