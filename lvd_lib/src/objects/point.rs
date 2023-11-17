//! The `Point` object stores data representing a two-dimensional point.
use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    objects::base::{Base, MetaInfo},
    Vector2, Version, Versioned,
};

/// An LVD object representing a two-dimensional point.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum Point {
    /// `Point` version 1.
    /// This version is not known to be used.
    #[br(pre_assert(version == 1))]
    V1 {
        /// Metadata for the object.
        meta_info: Versioned<MetaInfo>,

        /// Position of the point.
        pos: Versioned<Vector2>,
    },

    /// `Point` version 2.
    /// Replaces [meta_info](#variant.V1.field.meta_info) with [base](#variant.V2.field.base).
    #[br(pre_assert(version == 2))]
    V2 {
        /// Common data for the object.
        base: Versioned<Base>,

        /// Position of the point.
        pos: Versioned<Vector2>,
    },
}

impl Version for Point {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
            Self::V2 { .. } => 2,
        }
    }
}
