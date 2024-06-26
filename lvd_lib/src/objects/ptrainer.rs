//! The [`PTrainerRange`] and [`PTrainerFloatingFloor`] objects store data representing locations or objects where one or more Pokémon Trainers can reside.

use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    array::LvdArray,
    objects::base::Base,
    string::FixedString64,
    vector::Vector3,
    version::{Version, Versioned},
};

/// An LVD object representing the range in which one or more Pokémon Trainers can move around within.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum PTrainerRange {
    /// `PTrainerRange` version 1.
    ///
    /// This version is not known to be used.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The common data for the object.
        base: Versioned<Base>,

        /// The minimum position that a Pokémon Trainer can travel to.
        range_min: Versioned<Vector3>,

        /// The maximum position that a Pokémon Trainer can travel to.
        range_max: Versioned<Vector3>,

        /// The collection of starting positions for each Pokémon Trainer in the range.
        trainers: Versioned<LvdArray<Vector3>>,
    },

    /// `PTrainerRange` version 4.
    ///
    /// Adds [`parent_model_name`](#variant.V4.field.parent_model_name) and [`parent_joint_name`](#variant.V4.field.parent_joint_name).
    /// Versions 2 and 3 do not formally exist.
    #[br(pre_assert(version == 4))]
    V4 {
        /// The common data for the object.
        base: Versioned<Base>,

        /// The minimum position that a Pokémon Trainer can travel to.
        range_min: Versioned<Vector3>,

        /// The maximum position that a Pokémon Trainer can travel to.
        range_max: Versioned<Vector3>,

        /// The collection of starting positions for each Pokémon Trainer in the range.
        trainers: Versioned<LvdArray<Vector3>>,

        /// The name of a model for the range to inherit select transformations from.
        parent_model_name: Versioned<FixedString64>,

        /// The name of a joint from the parent model for the range to inherit select transformations from.
        parent_joint_name: Versioned<FixedString64>,
    },
}

impl Version for PTrainerRange {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
            Self::V4 { .. } => 4,
        }
    }
}

/// An LVD object representing a Pokémon Trainer's floating platform position.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum PTrainerFloatingFloor {
    /// `PTrainerFloatingFloor` version 1.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The common data for the object.
        base: Versioned<Base>,

        /// The position of the floating platform.
        pos: Versioned<Vector3>,
    },
}

impl Version for PTrainerFloatingFloor {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
        }
    }
}
