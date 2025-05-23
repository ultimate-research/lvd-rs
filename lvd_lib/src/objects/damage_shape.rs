//! The [`DamageShape`] object stores data representing a three-dimensional damage or attack collision shape.

use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    objects::base::Base,
    shape::Shape3,
    version::{Version, Versioned},
};

/// An LVD object representing a three-dimensional damage or attack collision shape.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum DamageShape {
    /// The first version of the `DamageShape` type.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The common data of the object.
        base: Versioned<Base>,

        /// The three-dimensional geometric representation of the object.
        shape: Versioned<Shape3>,

        /// Determines if the damage shape is an attack collision.
        #[br(map = |b: u8| b != 0)]
        #[bw(map = |b| u8::from(*b))]
        is_damager: bool,

        // TODO: Field documentation.
        id: u32,
    },
}

impl Version for DamageShape {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
        }
    }
}
