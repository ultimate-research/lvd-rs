//! The [`ItemPopup`] object stores data representing a collection of shapes where items will appear from.

use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    objects::base::Base,
    shape::ShapeArray2,
    tag::Tag,
    version::{Version, Versioned},
};

/// An LVD object representing a collection of shapes where items will appear from.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum ItemPopup {
    /// The first version of the `ItemPopup` type.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The common data of the object.
        base: Versioned<Base>,

        /// The identifier for matching and filtering like objects.
        tag: Versioned<Tag>,

        /// The collection of shapes where items will appear from.
        shapes: Versioned<ShapeArray2>,
    },
}

impl Version for ItemPopup {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
        }
    }
}
