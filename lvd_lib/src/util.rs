//! Basic vector types.
//!
//! This module contains the [`Vector2`] and [`Vector3`] types as well as the [`Rect`] type.
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
    /// `Vector2` version 1.
    #[br(pre_assert(version == 1))]
    V1 {
        /// Component along the x-axis.
        x: f32,

        /// Component along the y-axis.
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
    /// `Vector3` version 1.
    #[br(pre_assert(version == 1))]
    V1 {
        /// Component along the x-axis.
        x: f32,

        /// Component along the y-axis.
        y: f32,

        /// Component along the z-axis.
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

/// A two-dimensional rectangle type.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rect {
    /// `Rect` version 1.
    #[br(pre_assert(version == 1))]
    V1 {
        /// Coordinate of the left edge.
        left: f32,

        /// Coordinate of the right edge.
        right: f32,

        /// Coordinate of the top edge.
        top: f32,

        /// Coordinate of the bottom edge.
        bottom: f32,
    },
}

impl Version for Rect {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
        }
    }
}
