//! The [`Collision`] object stores data representing a two-dimensional polygonal collision.
//!
//! Extra data is stored to define properties of each edge in the collision.

use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    array::LvdArray,
    objects::base::{Base, MetaInfo},
    vector::Vector2,
    version::{Version, Versioned},
};

pub mod attribute;
pub mod cliff;
pub mod flags;
pub mod spirits_floor;

pub use attribute::CollisionAttribute;
pub use cliff::CollisionCliff;
pub use flags::CollisionFlags;
pub use spirits_floor::CollisionSpiritsFloor;

/// An LVD object representing a two-dimensional polygonal collision.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum Collision {
    /// `Collision` version 1.
    ///
    /// This version is not known to be used.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The metadata for the object.
        meta_info: Versioned<MetaInfo>,

        /// The flags representing the global attributes of the collision.
        flags: CollisionFlags,

        /// The collection of vertices forming the geometry of the collision.
        vertices: Versioned<LvdArray<Vector2>>,

        /// The collection of unit normal vectors defining the tangible side of each edge.
        normals: Versioned<LvdArray<Vector2>>,

        /// The collection of extra data for edges flagged as grabbable.
        cliffs: Versioned<LvdArray<CollisionCliff>>,
    },

    /// `Collision` version 2.
    ///
    /// Replaces [`meta_info`](#variant.V1.field.meta_info) with [`base`](#variant.V2.field.base).
    #[br(pre_assert(version == 2))]
    V2 {
        /// The common data for the object.
        base: Versioned<Base>,

        /// The flags representing the global attributes of the collision.
        flags: CollisionFlags,

        /// The collection of vertices forming the geometry of the collision.
        vertices: Versioned<LvdArray<Vector2>>,

        /// The collection of unit normal vectors defining the tangible side of each edge.
        normals: Versioned<LvdArray<Vector2>>,

        /// The collection of extra data for edges flagged as grabbable.
        cliffs: Versioned<LvdArray<CollisionCliff>>,
    },

    /// `Collision` version 3.
    ///
    /// Adds [`attributes`](#variant.V3.field.attributes).
    #[br(pre_assert(version == 3))]
    V3 {
        /// The common data for the object.
        base: Versioned<Base>,

        /// The flags representing the global attributes of the collision.
        flags: CollisionFlags,

        /// The collection of vertices forming the geometry of the collision.
        vertices: Versioned<LvdArray<Vector2>>,

        /// The collection of unit normal vectors defining the tangible side of each edge.
        normals: Versioned<LvdArray<Vector2>>,

        /// The collection of extra data for edges flagged as grabbable.
        cliffs: Versioned<LvdArray<CollisionCliff>>,

        /// The collection of material presets and flags for each edge in the collision.
        attributes: Versioned<LvdArray<CollisionAttribute>>,
    },

    /// `Collision` version 4.
    ///
    /// Adds [`spirits_floors`](#variant.V4.field.spirits_floors).
    #[br(pre_assert(version == 4))]
    V4 {
        /// The common data for the object.
        base: Versioned<Base>,

        /// The flags representing the global attributes of the collision.
        flags: CollisionFlags,

        /// The collection of vertices forming the geometry of the collision.
        vertices: Versioned<LvdArray<Vector2>>,

        /// The collection of unit normal vectors defining the tangible side of each edge.
        normals: Versioned<LvdArray<Vector2>>,

        /// The collection of extra data for edges flagged as grabbable.
        cliffs: Versioned<LvdArray<CollisionCliff>>,

        /// The collection of properties and attributes for each edge in the collision.
        attributes: Versioned<LvdArray<CollisionAttribute>>,

        /// The collection of entries related to hazardous floors in spirit battles.
        spirits_floors: Versioned<LvdArray<CollisionSpiritsFloor>>,
    },
}

impl Version for Collision {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
            Self::V2 { .. } => 2,
            Self::V3 { .. } => 3,
            Self::V4 { .. } => 4,
        }
    }
}
