//! The `EnemyGenerator` object stores data representing a collection of shapes to generate enemies from.
use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{objects::base::Base, LvdArray, LvdShape2Array, Tag, Version, Versioned};

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

        // TODO: Field documentation.
        unk1: Versioned<LvdShape2Array>,

        // TODO: Field documentation.
        unk2: Versioned<LvdShape2Array>,

        // TODO: Field documentation.
        unk3: Versioned<LvdShape2Array>,

        /// Unique integer identifier for programmatic discovery.
        tag: Versioned<Tag>,
    },

    /// `EnemyGenerator` version 2.
    /// Adds [unk4](#variant.V2.field.unk4) and [unk5](#variant.V2.field.unk5).
    /// This version is not known to be used.
    #[br(pre_assert(version == 2))]
    V2 {
        /// Common data for the object.
        base: Versioned<Base>,

        // TODO: Field documentation.
        unk1: Versioned<LvdShape2Array>,

        // TODO: Field documentation.
        unk2: Versioned<LvdShape2Array>,

        // TODO: Field documentation.
        unk3: Versioned<LvdShape2Array>,

        /// Unique integer identifier for programmatic discovery.
        tag: Versioned<Tag>,

        // TODO: Field documentation.
        unk4: Versioned<LvdArray<Tag>>,

        // TODO: Field documentation.
        unk5: Versioned<LvdArray<Tag>>,
    },

    /// `EnemyGenerator` version 3.
    /// Adds [unk6](#variant.V3.field.unk6).
    #[br(pre_assert(version == 3))]
    V3 {
        /// Common data for the object.
        base: Versioned<Base>,

        // TODO: Field documentation.
        unk1: Versioned<LvdShape2Array>,

        // TODO: Field documentation.
        unk2: Versioned<LvdShape2Array>,

        // TODO: Field documentation.
        unk3: Versioned<LvdShape2Array>,

        /// Unique integer identifier for programmatic discovery.
        tag: Versioned<Tag>,

        // TODO: Field documentation.
        unk4: Versioned<LvdArray<Tag>>,

        // TODO: Field documentation.
        unk5: Versioned<LvdArray<Tag>>,

        // TODO: Field documentation.
        unk6: Versioned<LvdArray<Tag>>,
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
