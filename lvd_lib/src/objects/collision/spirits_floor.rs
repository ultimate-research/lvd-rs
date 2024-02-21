//! The [`CollisionSpiritsFloor`] object stores data representing hazardous floors in spirit battles.
use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    objects::base::Base,
    string::FixedString64,
    version::{Version, Versioned},
};

/// An LVD subobject to [`Collision`](crate::objects::collision::Collision) representing hazardous floors in spirit battles.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum CollisionSpiritsFloor {
    /// `CollisionSpiritsFloor` version 1.
    /// This version is not known to be used.
    #[br(pre_assert(version == 1))]
    V1 {
        /// Common data for the object.
        base: Versioned<Base>,

        /// Index of the edge in the associated collision to link the object with.
        line_index: u32,

        /// Name of the line group that the object is associated with.
        line_group: Versioned<FixedString64>,
    },

    /// `CollisionSpiritsFloor` version 2.
    /// Adds [unk1](#variant.V2.field.unk1), [unk2](#variant.V2.field.unk2), [unk3](#variant.V2.field.unk3), [unk4](#variant.V2.field.unk4), [unk5](#variant.V2.field.unk5), and [unk6](#variant.V2.field.unk6).
    #[br(pre_assert(version == 2))]
    V2 {
        /// Common data for the object.
        base: Versioned<Base>,

        /// Index of the edge in the associated collision to link the object with.
        line_index: u32,

        /// Name of the line group that the object is associated with.
        line_group: Versioned<FixedString64>,

        // TODO: Field documentation. Usually 1.0. Unused?
        unk1: f32,

        // TODO: Field documentation. Usually 1.0. Unused?
        unk2: f32,

        // TODO: Field documentation. Always 1.0. Unused?
        unk3: f32,

        // TODO: Field documentation. Always 1.0. Unused?
        unk4: f32,

        // TODO: Field documentation. Always 0.0. Unused?
        unk5: f32,

        // TODO: Field documentation. Always 0.0. Unused?
        unk6: f32,
    },
}

impl Version for CollisionSpiritsFloor {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
            Self::V2 { .. } => 2,
        }
    }
}
