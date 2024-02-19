//! The [`GeneralShape2`] and [`GeneralShape3`] objects store data representing general-purpose shapes.
use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    objects::base::Base,
    shape::{LvdShape2, LvdShape3},
    tag::Tag,
    version::{Version, Versioned},
};

/// An LVD object representing a general-purpose two-dimensional shape.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum GeneralShape2 {
    /// `GeneralShape2` version 1.
    #[br(pre_assert(version == 1))]
    V1 {
        /// Common data for the object.
        base: Versioned<Base>,

        /// Identifier for matching and filtering like objects.
        tag: Versioned<Tag>,

        /// Shape represented by the object.
        shape: Versioned<LvdShape2>,
    },
}

impl Version for GeneralShape2 {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
        }
    }
}

/// An LVD object representing a general-purpose three-dimensional shape.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum GeneralShape3 {
    /// `GeneralShape3` version 1.
    #[br(pre_assert(version == 1))]
    V1 {
        /// Common data for the object.
        base: Versioned<Base>,

        /// Identifier for matching and filtering like objects.
        tag: Versioned<Tag>,

        /// Shape represented by the object.
        shape: Versioned<LvdShape3>,
    },
}

impl Version for GeneralShape3 {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
        }
    }
}
