//! The [`GeneralShape2`] and [`GeneralShape3`] objects store data representing general-purpose shapes.

use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    objects::base::Base,
    shape::{Shape2, Shape3},
    tag::Tag,
    version::{Version, Versioned},
};

/// An LVD object representing a general-purpose two-dimensional shape.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum GeneralShape2 {
    /// The first version of the `GeneralShape2` type.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The common data of the object.
        base: Versioned<Base>,

        /// The identifier for matching and filtering like objects.
        tag: Versioned<Tag>,

        /// The two-dimensional geometric representation of the object.
        shape: Versioned<Shape2>,
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
    /// The first version of the `GeneralShape3` type.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The common data of the object.
        base: Versioned<Base>,

        /// The identifier for matching and filtering like objects.
        tag: Versioned<Tag>,

        /// The three-dimensional geometric representation of the object.
        shape: Versioned<Shape3>,
    },
}

impl Version for GeneralShape3 {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
        }
    }
}
