//! Basic vector utilities.

use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::version::Version;

/// A two-dimensional vector type.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Vector2 {
    /// The first version of the `Vector2` type.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The component along the x-axis.
        x: f32,

        /// The component along the y-axis.
        y: f32,
    },
}

impl Version for Vector2 {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
        }
    }
}

/// A three-dimensional vector type.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Vector3 {
    /// The first version of the `Vector3` type.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The component along the x-axis.
        x: f32,

        /// The component along the y-axis.
        y: f32,

        /// The component along the z-axis.
        z: f32,
    },
}

impl Version for Vector3 {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
        }
    }
}
