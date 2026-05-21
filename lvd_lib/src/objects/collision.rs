//! The [`Collision`] object stores data representing a two-dimensional polygonal collision.

use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    array::Array,
    objects::base::{Base, MetaInfo},
    vector::Vector2,
    version::{Version, Versioned},
};

pub mod attribute;
pub mod cliff;
pub mod flags;
pub mod line_group;

pub use attribute::CollisionAttribute;
pub use cliff::CollisionCliff;
pub use flags::CollisionFlags;
pub use line_group::CollisionLineGroup;

/// An object representing a two-dimensional polygonal collision.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum Collision {
    /// The first version of the `Collision` type.
    ///
    /// This version is not known to be used.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The metadata of the object.
        meta_info: Versioned<MetaInfo>,

        /// The global attributes of the object.
        flags: CollisionFlags,

        /// The collection of vertices forming the geometry of the object.
        vertices: Versioned<Array<Vector2>>,

        /// The collection of unit normal vectors defining the tangible side of each edge.
        normals: Versioned<Array<Vector2>>,

        /// The collection of supplementary data for edges flagged as grabbable.
        cliffs: Versioned<Array<CollisionCliff>>,
    },

    /// The second version of the `Collision` type.
    ///
    /// Replaces [`meta_info`](#variant.V1.field.meta_info) with [`base`](#variant.V2.field.base).
    #[br(pre_assert(version == 2))]
    V2 {
        /// The common data of the object.
        base: Versioned<Base>,

        /// The global attributes of the object.
        flags: CollisionFlags,

        /// The collection of vertices forming the geometry of the object.
        vertices: Versioned<Array<Vector2>>,

        /// The collection of unit normal vectors defining the tangible side of each edge.
        normals: Versioned<Array<Vector2>>,

        /// The collection of supplementary data for edges flagged as grabbable.
        cliffs: Versioned<Array<CollisionCliff>>,
    },

    /// The third version of the `Collision` type.
    ///
    /// Adds [`attributes`](#variant.V3.field.attributes).
    #[br(pre_assert(version == 3))]
    V3 {
        /// The common data of the object.
        base: Versioned<Base>,

        /// The global attributes of the object.
        flags: CollisionFlags,

        /// The collection of vertices forming the geometry of the object.
        vertices: Versioned<Array<Vector2>>,

        /// The collection of unit normal vectors defining the tangible side of each edge.
        normals: Versioned<Array<Vector2>>,

        /// The collection of supplementary data for edges flagged as grabbable.
        cliffs: Versioned<Array<CollisionCliff>>,

        /// The collection of attributes for each edge in the object.
        attributes: Versioned<Array<CollisionAttribute>>,
    },

    /// The fourth version of the `Collision` type.
    ///
    /// Adds [`line_groups`](#variant.V4.field.line_groups).
    #[br(pre_assert(version == 4))]
    V4 {
        /// The common data of the object.
        base: Versioned<Base>,

        /// The global attributes of the object.
        flags: CollisionFlags,

        /// The collection of vertices forming the geometry of the object.
        vertices: Versioned<Array<Vector2>>,

        /// The collection of unit normal vectors defining the tangible side of each edge.
        normals: Versioned<Array<Vector2>>,

        /// The collection of supplementary data for edges flagged as grabbable.
        cliffs: Versioned<Array<CollisionCliff>>,

        /// The collection of attributes for each edge in the object.
        attributes: Versioned<Array<CollisionAttribute>>,

        /// The collection of references to edge groups.
        line_groups: Versioned<Array<CollisionLineGroup>>,
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
