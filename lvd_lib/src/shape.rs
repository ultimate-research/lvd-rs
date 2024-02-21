//! Basic shape types.
//!
//! This module contains the [`Shape2`], [`ShapeArray2`] and [`ShapeArrayElement2`] types,
//! the [`Shape3`] type, the [`LvdPath`] type, and the [`Rect`] type.
use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    array::LvdArray,
    vector::Vector2,
    version::{Version, Versioned},
};

/// A two-dimensional shape type.
#[binrw]
#[br(import(_version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum Shape2 {
    /// Point shape type.
    #[brw(magic = 1u32)]
    Point {
        /// Position of the point along the x-axis.
        pos_x: f32,

        /// Position of the point along the y-axis.
        pos_y: f32,

        /// Collection of points forming the path shape.
        /// Should always be empty.
        #[brw(pad_before = 8)]
        path: Versioned<LvdPath>,
    },

    /// Circle shape type.
    #[brw(magic = 2u32)]
    Circle {
        /// Position of the circle along the x-axis.
        pos_x: f32,

        /// Position of the circle along the y-axis.
        pos_y: f32,

        /// Radius of the circle.
        radius: f32,

        /// Collection of points forming the path shape.
        /// Should always be empty.
        #[brw(pad_before = 4)]
        path: Versioned<LvdPath>,
    },

    /// Rectangle shape type.
    #[brw(magic = 3u32)]
    Rect {
        /// Coordinate of the rectangle's left edge.
        left: f32,

        /// Coordinate of the rectangle's right edge.
        right: f32,

        /// Coordinate of the rectangle's bottom edge.
        bottom: f32,

        /// Coordinate of the rectangle's top edge.
        top: f32,

        /// Collection of points forming the path shape.
        /// Should always be empty.
        path: Versioned<LvdPath>,
    },

    /// Path shape type.
    #[brw(magic = 4u32)]
    Path {
        /// Collection of points forming the path shape.
        #[brw(pad_before = 16)]
        path: Versioned<LvdPath>,
    },
}

impl Version for Shape2 {
    fn version(&self) -> u8 {
        3
    }
}

// TODO: Why is this type used for an array of two-dimensional shapes?
/// A fixed-size collection of two-dimensional shapes.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum ShapeArray2 {
    /// `ShapeArray2` version 1.
    #[br(pre_assert(version == 1))]
    V1 {
        /// Collection of two-dimensional shapes.
        shapes: Versioned<LvdArray<ShapeArrayElement2>>,
    },
}

impl Version for ShapeArray2 {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
        }
    }
}

// TODO: Why is this type used as the element type for an array of two-dimensional shapes?
/// The element type for a [`ShapeArray2`].
#[binrw]
#[br(import(_version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug)]
pub struct ShapeArrayElement2(pub Versioned<Shape2>);

impl Version for ShapeArrayElement2 {
    fn version(&self) -> u8 {
        1
    }
}

/// A three-dimensional shape type.
#[binrw]
#[br(import(_version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum Shape3 {
    /// Box shape type.
    #[brw(magic = 1u32)]
    Box {
        /// Coordinate of the box's left edge.
        left: f32,

        /// Coordinate of the box's right edge.
        right: f32,

        /// Coordinate of the box's bottom edge.
        bottom: f32,

        /// Coordinate of the box's top edge.
        top: f32,

        /// Coordinate of the box's back edge.
        back: f32,

        /// Coordinate of the box's front edge.
        #[brw(pad_after = 4)]
        front: f32,
    },

    /// Sphere shape type.
    #[brw(magic = 2u32)]
    Sphere {
        /// Position of the sphere along the x-axis.
        pos_x: f32,

        /// Position of the sphere along the y-axis.
        pos_y: f32,

        /// Position of the sphere along the z-axis.
        pos_z: f32,

        /// Radius of the sphere.
        #[brw(pad_after = 12)]
        radius: f32,
    },

    /// Capsule shape type.
    #[brw(magic = 3u32)]
    Capsule {
        /// Position of the capsule along the x-axis.
        pos_x: f32,

        /// Position of the capsule along the y-axis.
        pos_y: f32,

        /// Position of the capsule along the z-axis.
        pos_z: f32,

        /// Directional vector for the second endpoint of the capsule along the x-axis.
        vec_x: f32,

        /// Directional vector for the second endpoint of the capsule along the y-axis.
        vec_y: f32,

        /// Directional vector for the second endpoint of the capsule along the z-axis.
        vec_z: f32,

        /// Radius of the capsule.
        radius: f32,
    },

    /// Point shape type.
    #[brw(magic = 4u32)]
    Point {
        /// Position of the point along the x-axis.
        pos_x: f32,

        /// Position of the point along the y-axis.
        pos_y: f32,

        /// Position of the point along the z-axis.
        #[brw(pad_after = 16)]
        pos_z: f32,
    },
}

impl Version for Shape3 {
    fn version(&self) -> u8 {
        1
    }
}

/// A collection of two-dimensional points forming a path shape.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum LvdPath {
    /// `LvdPath` version 1.
    #[br(pre_assert(version == 1))]
    V1 {
        /// Collection of two-dimensional points forming the path shape.
        points: Versioned<LvdArray<Vector2>>,
    },
}

impl Version for LvdPath {
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
