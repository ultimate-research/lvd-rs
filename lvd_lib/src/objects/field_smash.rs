//! The objects exclusive to Smash Run in Super Smash Bros. for Nintendo 3DS.

use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    id::Id,
    objects::{base::Base, region::Region},
    shape::{Path, Rect, Shape2, Shape3},
    string::FixedString32,
    tag::Tag,
    vector::Vector2,
    version::{Version, Versioned},
};

/// An LVD object representing a two-dimensional shape where a stat boost or item can appear when in view.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum FsItem {
    /// The first version of the `FsItem` type.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The common data of the object.
        base: Versioned<Base>,

        /// The two-dimensional geometric representation of the object
        shape: Versioned<Shape2>,

        /// The identifier for matching and filtering like objects.
        tag: Versioned<Tag>,
    },
}

impl Version for FsItem {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
        }
    }
}

// TODO: Type documentation.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum FsUnknown {
    /// The first version of the `FsUnknown` type.
    ///
    /// This version is not known to be used.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The common data of the object.
        base: Versioned<Base>,

        // TODO: Field documentation.
        unk1: Versioned<Rect>,

        // TODO: Field documentation.
        unk2: Versioned<FsCamLimit>,
    },

    /// The second version of the `FsUnknown` type.
    ///
    /// Adds [`unk3`](#variant.V2.field.unk3).
    /// This version is not known to be used.
    #[br(pre_assert(version == 2))]
    V2 {
        /// The common data of the object.
        base: Versioned<Base>,

        // TODO: Field documentation.
        unk1: Versioned<Rect>,

        // TODO: Field documentation.
        unk2: Versioned<FsCamLimit>,

        // TODO: Field documentation.
        unk3: u32,
    },
}

impl Version for FsUnknown {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
            Self::V2 { .. } => 2,
        }
    }
}

// TODO: Type documentation.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum FsAreaCam {
    /// The first version of the `FsAreaCam` type.
    #[br(pre_assert(version == 1))]
    V1 {
        // TODO: Field documentation.
        region: Versioned<Region>,

        // TODO: Field documentation.
        unk: u32,
    },
}

impl Version for FsAreaCam {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
        }
    }
}

/// An LVD object representing a region to restrict camera movement within on entrance of a trigger.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum FsAreaLock {
    /// The first version of the `FsAreaLock` type.
    ///
    /// This version is not known to be used.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The common data of the object.
        base: Versioned<Base>,

        /// The edge coordinates of the region for restricted camera movement.
        camera_region: Versioned<Rect>,

        /// The edge coordinates of the trigger region for activating the restricted camera movement.
        trigger_region: Versioned<Rect>,

        // TODO: Field documentation.
        unk1: u32,
    },

    /// The second version of the `FsAreaLock` type.
    ///
    /// Adds [`unk2`](#variant.V2.field.unk2).
    #[br(pre_assert(version == 2))]
    V2 {
        /// The common data of the object.
        base: Versioned<Base>,

        /// The edge coordinates of the region for restricted camera movement.
        camera_region: Versioned<Rect>,

        /// The edge coordinates of the trigger region for activating the restricted camera movement.
        trigger_region: Versioned<Rect>,

        // TODO: Field documentation.
        unk1: u32,

        // TODO: Field documentation.
        unk2: Versioned<Vector2>,
    },
}

impl Version for FsAreaLock {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
            Self::V2 { .. } => 2,
        }
    }
}

/// An LVD object representing a region to restrict camera movement within.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum FsCamLimit {
    /// The first version of the `FsCamLimit` type.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The common data of the object.
        base: Versioned<Base>,

        /// The path shape forming the camera limit region.
        path: Versioned<Path>,
    },
}

impl Version for FsCamLimit {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
        }
    }
}

// TODO: Type documentation.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum AreaLight {
    /// The first version of the `AreaLight` type.
    ///
    /// This version is not known to be used.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The common data of the object.
        base: Versioned<Base>,

        /// The two-dimensional geometric representation of the object.
        shape: Versioned<Shape2>,
    },

    /// The second version of the `AreaLight` type.
    ///
    /// Adds [`unk1`](#variant.V2.field.unk1) and [`unk2`](#variant.V2.field.unk2).
    #[br(pre_assert(version == 2))]
    V2 {
        /// The common data of the object.
        base: Versioned<Base>,

        /// The two-dimensional geometric representation of the object.
        shape: Versioned<Shape2>,

        // TODO: Field documentation.
        unk1: Versioned<FixedString32>,

        // TODO: Field documentation.
        unk2: Versioned<FixedString32>,
    },
}

impl Version for AreaLight {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
            Self::V2 { .. } => 2,
        }
    }
}

/// An LVD object representing a two-dimensional point where a fighter can start and restart from.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum FsStartPoint {
    /// The first version of the `FsStartPoint` type.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The common data of the object.
        base: Versioned<Base>,

        /// The position of the point.
        pos: Versioned<Vector2>,

        // TODO: Field documentation.
        id: Versioned<Id>,
    },
}

impl Version for FsStartPoint {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
        }
    }
}

// TODO: Type documentation.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum AreaHint {
    /// The first version of the `AreaHint` type.
    ///
    /// This version is not known to be used.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The common data of the object.
        base: Versioned<Base>,

        /// The three-dimensional geometric representation of the object.
        shape: Versioned<Shape3>,

        // TODO: Field documentation.
        unk1: i32,

        // TODO: Field documentation.
        unk2: i32,

        // TODO: Field documentation.
        unk3: i32,

        // TODO: Field documentation.
        unk4: i32,
    },

    /// The second version of the `AreaHint` type.
    ///
    /// Adds [`unk5`](#variant.V2.field.unk5).
    /// This version is not known to be used.
    #[br(pre_assert(version == 2))]
    V2 {
        /// The common data of the object.
        base: Versioned<Base>,

        /// The three-dimensional geometric representation of the object.
        shape: Versioned<Shape3>,

        // TODO: Field documentation.
        unk1: i32,

        // TODO: Field documentation.
        unk2: i32,

        // TODO: Field documentation.
        unk3: i32,

        // TODO: Field documentation.
        unk4: i32,

        // TODO: Field documentation.
        unk5: u8,
    },

    /// The third version of the `AreaHint` type.
    ///
    /// Adds [`unk6`](#variant.V3.field.unk6) and [`unk7`](#variant.V3.field.unk7).
    #[br(pre_assert(version == 3))]
    V3 {
        /// The common data of the object.
        base: Versioned<Base>,

        /// The three-dimensional geometric representation of the object.
        shape: Versioned<Shape3>,

        // TODO: Field documentation.
        unk1: i32,

        // TODO: Field documentation.
        unk2: i32,

        // TODO: Field documentation.
        unk3: i32,

        // TODO: Field documentation.
        unk4: i32,

        // TODO: Field documentation.
        unk5: u8,

        // TODO: Field documentation.
        unk6: i32,

        // TODO: Field documentation.
        unk7: i32,
    },
}

impl Version for AreaHint {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
            Self::V2 { .. } => 2,
            Self::V3 { .. } => 3,
        }
    }
}

// TODO: Type documentation.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum SplitArea {
    /// The first version of the `SplitArea` type.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The common data of the object.
        base: Versioned<Base>,

        /// The three-dimensional geometric representation of the object.
        shape: Versioned<Shape3>,
    },
}

impl Version for SplitArea {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
        }
    }
}
