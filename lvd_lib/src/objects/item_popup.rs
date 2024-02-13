//! The `ItemPopup` object stores data representing a collection of shapes where items will appear from.
use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{objects::base::Base, LvdShape2Array, Tag, Version, Versioned};

/// An LVD object representing a collection of shapes where items will appear from.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum ItemPopup {
    /// `ItemPopup` version 1.
    #[br(pre_assert(version == 1))]
    V1 {
        /// Common data for the object.
        base: Versioned<Base>,

        /// Identifier for matching and filtering like objects.
        tag: Versioned<Tag>,

        /// Collection of shapes where items will appear from.
        shapes: Versioned<LvdShape2Array>,
    },
}

impl Version for ItemPopup {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
        }
    }
}
