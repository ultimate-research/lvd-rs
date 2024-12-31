//! A fixed-size collection of contiguous versioned elements.
//!
//! This module contains the [`LvdArray`] type.

use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::version::{Version, Versioned};

/// A fixed-size collection of contiguous versioned elements.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum LvdArray<T: Version + 'static> {
    /// `LvdArray` version 1.
    #[br(pre_assert(version == 1))]
    V1 {
        #[br(temp)]
        #[bw(calc = elements.len() as u32)]
        count: u32,

        /// The collection of contiguous versioned elements.
        #[br(count = count)]
        elements: Vec<Versioned<T>>,
    },
}

impl<T> Version for LvdArray<T>
where
    T: Version,
{
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
        }
    }
}
