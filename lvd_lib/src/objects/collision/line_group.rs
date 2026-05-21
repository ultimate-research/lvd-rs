//! The [`CollisionLineGroup`] type stores data representing a reference to an edge group.

use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    objects::base::Base,
    string::FixedString64,
    version::{Version, Versioned},
};

/// An LVD subobject to a [`Collision`](crate::objects::Collision) representing a reference to an edge group.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum CollisionLineGroup {
    /// The first version of the `CollisionLineGroup` type.
    ///
    /// This version is not known to be used.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The common data of the object.
        base: Versioned<Base>,

        /// The index of the edge in the associated object to link with.
        line_index: u32,

        /// The name of the edge group that the object is associated with.
        label: Versioned<FixedString64>,
    },

    /// The second version of the `CollisionLineGroup` type.
    ///
    /// Adds [`unk1`](#variant.V2.field.unk1), [`unk2`](#variant.V2.field.unk2), [`unk3`](#variant.V2.field.unk3),
    /// [`unk4`](#variant.V2.field.unk4), [`unk5`](#variant.V2.field.unk5), and [`unk6`](#variant.V2.field.unk6).
    #[br(pre_assert(version == 2))]
    V2 {
        /// The common data of the object.
        base: Versioned<Base>,

        /// The index of the edge in the associated object to link with.
        line_index: u32,

        /// The name of the edge group that the object is associated with.
        label: Versioned<FixedString64>,

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

impl Version for CollisionLineGroup {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
            Self::V2 { .. } => 2,
        }
    }
}
