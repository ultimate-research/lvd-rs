//! The [`PTrainerRange`] and [`PTrainerFloatingFloor`] objects store data representing locations or objects where one or more Pokémon Trainers can reside.

use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    array::Array,
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
    /// The first version of the `PTrainerRange` type.
    ///
    /// This version is not known to be used.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The common data of the object.
        base: Versioned<Base>,

        /// The minimum position a Pokémon Trainer can move to.
        range_min: Versioned<Vector3>,

        /// The maximum position a Pokémon Trainer can move to.
        range_max: Versioned<Vector3>,

        /// The collection of starting positions for each Pokémon Trainer in the range.
        trainers: Versioned<Array<Vector3>>,
    },

    /// The fourth version of the `PTrainerRange` type.
    ///
    /// Adds [`parent_model_name`](#variant.V4.field.parent_model_name) and [`parent_joint_name`](#variant.V4.field.parent_joint_name).
    /// The second and third versions do not formally exist.
    #[br(pre_assert(version == 4))]
    V4 {
        /// The common data of the object.
        base: Versioned<Base>,

        /// The minimum position a Pokémon Trainer can move to.
        range_min: Versioned<Vector3>,

        /// The maximum position a Pokémon Trainer can move to.
        range_max: Versioned<Vector3>,

        /// The collection of starting positions for each Pokémon Trainer in the range.
        trainers: Versioned<Array<Vector3>>,

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

/// An LVD object representing a Pokémon Trainer's floating platform.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum PTrainerFloatingFloor {
    /// The first version of the `PTrainerFloatingFloor` type.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The common data of the object.
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
