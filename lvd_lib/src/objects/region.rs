//! The [`Region`] object stores data representing a two-dimensional rectangle.

use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    objects::base::{Base, MetaInfo},
    shape::Rect,
    version::{Version, Versioned},
};

/// An LVD object representing a two-dimensional rectangle.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum Region {
    /// The first version of the `Region` type.
    ///
    /// This version is not known to be used.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The metadata of the object.
        meta_info: Versioned<MetaInfo>,

        /// The edge coordinates of the region.
        rect: Versioned<Rect>,
    },

    /// The second version of the `Region` type.
    ///
    /// Replaces [`meta_info`](#variant.V1.field.meta_info) with [`base`](#variant.V2.field.base).
    #[br(pre_assert(version == 2))]
    V2 {
        /// The common data of the object.
        base: Versioned<Base>,

        /// The edge coordinates of the region.
        rect: Versioned<Rect>,
    },
}

impl Version for Region {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
            Self::V2 { .. } => 2,
        }
    }
}
