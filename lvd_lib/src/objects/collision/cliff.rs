//! The `CollisionCliff` object stores data representing a grabbable edge.
use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{objects::base::Base, Vector2, Version, Versioned};

/// An LVD subobject to [`Collision`](crate::objects::collision::Collision) representing a grabbable edge.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum CollisionCliff {
    /// `CollisionCliff` version 1.
    /// This version is not known to be used.
    #[br(pre_assert(version == 1))]
    V1 {
        /// Position of the cliff.
        /// This should be equal to the position of the corresponding vertex.
        pos: Versioned<Vector2>,

        /// Facing direction of the cliff.
        /// -1.0 corresponds to the left and 1.0 corresponds to the right.
        lr: f32,
    },

    /// `CollisionCliff` version 2.
    /// Adds [base](#variant.V2.field.base).
    #[br(pre_assert(version == 2))]
    V2 {
        /// Common data for the object.
        base: Versioned<Base>,

        /// Position of the cliff.
        /// This should be equal to the position of the corresponding vertex.
        pos: Versioned<Vector2>,

        /// Facing direction of the cliff.
        /// -1.0 corresponds to the left and 1.0 corresponds to the right.
        lr: f32,
    },

    /// `CollisionCliff` version 3.
    /// Adds [line_index](#variant.V3.field.line_index).
    #[br(pre_assert(version == 3))]
    V3 {
        /// Common data for the object.
        base: Versioned<Base>,

        /// Position of the cliff.
        /// This should be equal to the position of the corresponding vertex.
        pos: Versioned<Vector2>,

        /// Facing direction of the cliff.
        /// -1.0 corresponds to the left and 1.0 corresponds to the right.
        lr: f32,

        /// Index of the edge in the associated collision to link the object with.
        line_index: u32,
    },
}

impl Version for CollisionCliff {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
            Self::V2 { .. } => 2,
            Self::V3 { .. } => 3,
        }
    }
}
