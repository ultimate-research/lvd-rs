//! Basic shape types.
//!
//! This module contains the [`Shape2`], [`ShapeArray2`] and [`ShapeArrayElement2`] types,
//! the [`Shape3`] type, the [`LvdPath`] type, and the [`Rect`] type.

use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    array::Array,
    vector::Vector2,
    version::{Version, Versioned},
};

/// A two-dimensional shape type.
#[binrw]
#[br(import(_version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum Shape2 {
    /// The point shape type.
    #[brw(magic = 1u32)]
    Point {
        /// The position along the x-axis.
        pos_x: f32,

        /// The position along the y-axis.
        pos_y: f32,

        /// The collection of points forming the path shape.
        /// This collection should always be empty.
        #[brw(pad_before = 8)]
        path: Versioned<LvdPath>,
    },

    /// The circle shape type.
    #[brw(magic = 2u32)]
    Circle {
        /// The position along the x-axis.
        pos_x: f32,

        /// The position along the y-axis.
        pos_y: f32,

        /// The radius of the circle.
        radius: f32,

        /// The collection of points forming the path shape.
        /// This collection should always be empty.
        #[brw(pad_before = 4)]
        path: Versioned<LvdPath>,
    },

    /// The rectangle shape type.
    #[brw(magic = 3u32)]
    Rect {
        /// The coordinate of the left edge.
        left: f32,

        /// The coordinate of the right edge.
        right: f32,

        /// The coordinate of the bottom edge.
        bottom: f32,

        /// The coordinate of the top edge.
        top: f32,

        /// The collection of points forming the path shape.
        /// This collection should always be empty.
        path: Versioned<LvdPath>,
    },

    /// The path shape type.
    #[brw(magic = 4u32)]
    Path {
        /// The collection of points forming the path shape.
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
    /// The first version of the `ShapeArray2` type.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The collection of two-dimensional shapes.
        shapes: Versioned<Array<ShapeArrayElement2>>,
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
    /// The box shape type.
    #[brw(magic = 1u32)]
    Box {
        /// The coordinate of the left edge.
        left: f32,

        /// The coordinate of the right edge.
        right: f32,

        /// The coordinate of the bottom edge.
        bottom: f32,

        /// The coordinate of the top edge.
        top: f32,

        /// The coordinate of the back edge.
        back: f32,

        /// The coordinate of the front edge.
        #[brw(pad_after = 4)]
        front: f32,
    },

    /// The sphere shape type.
    #[brw(magic = 2u32)]
    Sphere {
        /// The position along the x-axis.
        pos_x: f32,

        /// The position along the y-axis.
        pos_y: f32,

        /// The position along the z-axis.
        pos_z: f32,

        /// The radius of the sphere.
        #[brw(pad_after = 12)]
        radius: f32,
    },

    /// The capsule shape type.
    #[brw(magic = 3u32)]
    Capsule {
        /// The position of the start point along the x-axis.
        pos_x: f32,

        /// The position of the start point along the y-axis.
        pos_y: f32,

        /// The position of the start point along the z-axis.
        pos_z: f32,

        /// The displacement of the endpoint from the start point along the x-axis.
        vec_x: f32,

        /// The displacement of the endpoint from the start point along the y-axis.
        vec_y: f32,

        /// The displacement of the endpoint from the start point along the z-axis.
        vec_z: f32,

        /// The radius of the capsule.
        radius: f32,
    },

    /// The point shape type.
    #[brw(magic = 4u32)]
    Point {
        /// The position along the x-axis.
        pos_x: f32,

        /// The position along the y-axis.
        pos_y: f32,

        /// The position along the z-axis.
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
    /// The first version of the `LvdPath` type.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The collection of two-dimensional points forming the path shape.
        points: Versioned<Array<Vector2>>,
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
    /// The first version of the `Rect` type.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The coordinate of the left edge.
        left: f32,

        /// The coordinate of the right edge.
        right: f32,

        /// The coordinate of the top edge.
        top: f32,

        /// The coordinate of the bottom edge.
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
