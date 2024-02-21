//! The [`EnemyGenerator`] object stores data representing a collection of shapes to generate enemies from.
use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    array::LvdArray,
    objects::base::Base,
    shape::ShapeArray2,
    tag::Tag,
    version::{Version, Versioned},
};

/// An LVD object representing a collection of shapes to generate enemies from.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum EnemyGenerator {
    /// `EnemyGenerator` version 1.
    /// This version is not known to be used.
    #[br(pre_assert(version == 1))]
    V1 {
        /// Common data for the object.
        base: Versioned<Base>,

        /// Collection of shapes where enemies can appear from.
        appear_shapes: Versioned<ShapeArray2>,

        /// Collection of shapes for responding to fighter presence.
        trigger_shapes: Versioned<ShapeArray2>,

        // TODO: Field documentation.
        unk1: Versioned<ShapeArray2>,

        /// Identifier for matching and filtering like objects.
        tag: Versioned<Tag>,
    },

    /// `EnemyGenerator` version 2.
    /// Adds [appear_tags](#variant.V2.field.appear_tags) and [unk2](#variant.V2.field.unk2).
    /// This version is not known to be used.
    #[br(pre_assert(version == 2))]
    V2 {
        /// Common data for the object.
        base: Versioned<Base>,

        /// Collection of shapes where enemies can appear from.
        appear_shapes: Versioned<ShapeArray2>,

        /// Collection of shapes for responding to fighter presence.
        trigger_shapes: Versioned<ShapeArray2>,

        // TODO: Field documentation.
        unk1: Versioned<ShapeArray2>,

        /// Identifier for matching and filtering like objects.
        tag: Versioned<Tag>,

        /// Collection of identifiers for matching and filtering appear regions.
        appear_tags: Versioned<LvdArray<Tag>>,

        // TODO: Field documentation.
        unk2: Versioned<LvdArray<Tag>>,
    },

    /// `EnemyGenerator` version 3.
    /// Adds [trigger_tags](#variant.V3.field.trigger_tags).
    #[br(pre_assert(version == 3))]
    V3 {
        /// Common data for the object.
        base: Versioned<Base>,

        /// Collection of shapes where enemies can appear from.
        appear_shapes: Versioned<ShapeArray2>,

        /// Collection of shapes for responding to fighter presence.
        trigger_shapes: Versioned<ShapeArray2>,

        // TODO: Field documentation.
        unk1: Versioned<ShapeArray2>,

        /// Identifier for matching and filtering like objects.
        tag: Versioned<Tag>,

        /// Collection of identifiers for matching and filtering appear regions.
        appear_tags: Versioned<LvdArray<Tag>>,

        // TODO: Field documentation.
        unk2: Versioned<LvdArray<Tag>>,

        /// Collection of identifiers for matching and filtering trigger regions.
        trigger_tags: Versioned<LvdArray<Tag>>,
    },
}

impl Version for EnemyGenerator {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
            Self::V2 { .. } => 2,
            Self::V3 { .. } => 3,
        }
    }
}
