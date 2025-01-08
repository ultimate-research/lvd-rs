//! The [`Base`] and [`MetaInfo`] types store common data for an LVD object.

use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    id::Id,
    string::{FixedString56, FixedString64},
    vector::Vector3,
    version::{Version, Versioned},
};

/// The common data for an LVD object.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum Base {
    /// The first version of the `Base` type.
    ///
    /// This version is not known to be used.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The metadata of the object.
        meta_info: Versioned<MetaInfo>,

        /// The name of the object as seen by the game when classed as dynamic.
        dynamic_name: Versioned<FixedString64>,
    },

    /// The second version of the `Base` type.
    ///
    /// Adds [`dynamic_offset`](#variant.V2.field.dynamic_offset).
    #[br(pre_assert(version == 2))]
    V2 {
        /// The metadata of the object.
        meta_info: Versioned<MetaInfo>,

        /// The name of the object as seen by the game when classed as dynamic.
        dynamic_name: Versioned<FixedString64>,

        /// The displacement of the object when classed as dynamic.
        dynamic_offset: Versioned<Vector3>,
    },

    /// The third version of the `Base` type.
    ///
    /// Adds [`is_dynamic`](#variant.V3.field.is_dynamic), [`instance_id`](#variant.V3.field.instance_id), and [`instance_offset`](#variant.V3.field.instance_offset).
    /// This version is not known to be used.
    #[br(pre_assert(version == 3))]
    V3 {
        /// The metadata of the object.
        meta_info: Versioned<MetaInfo>,

        /// The name of the object as seen by the game when classed as dynamic.
        dynamic_name: Versioned<FixedString64>,

        /// The displacement of the object when classed as dynamic.
        dynamic_offset: Versioned<Vector3>,

        /// Determines if the object is classed as dynamic.
        #[br(map = |b: u8| b != 0)]
        #[bw(map = |b| u8::from(*b))]
        is_dynamic: bool,

        /// The numeric ID of the instanced object.
        ///
        /// Must be nonzero for the object to be classed as an instanced object.
        instance_id: Versioned<Id>,

        /// The displacement of the object when classed as an instanced object.
        instance_offset: Versioned<Vector3>,
    },

    /// The fourth version of the `Base` type.
    ///
    /// Adds [`joint_index`](#variant.V4.field.joint_index) and [`joint_name`](#variant.V4.field.joint_name).
    #[br(pre_assert(version == 4))]
    V4 {
        /// The metadata of the object.
        meta_info: Versioned<MetaInfo>,

        /// The name of the object as seen by the game when classed as dynamic.
        dynamic_name: Versioned<FixedString64>,

        /// The displacement of the object when classed as dynamic.
        dynamic_offset: Versioned<Vector3>,

        /// Determines if the object is classed as dynamic.
        #[br(map = |b: u8| b != 0)]
        #[bw(map = |b| u8::from(*b))]
        is_dynamic: bool,

        /// The numeric identifier of the instanced object.
        ///
        /// Must be nonzero for the object to be classed as an instanced object.
        instance_id: Versioned<Id>,

        /// The displacement of the object when classed as an instanced object.
        instance_offset: Versioned<Vector3>,

        /// The index of the joint from the parent model to parent the object to when classed as dynamic.
        joint_index: i32,

        /// The name of the joint from the parent model to parent the object to when classed as dynamic.
        joint_name: Versioned<FixedString64>,
    },
}

impl Version for Base {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
            Self::V2 { .. } => 2,
            Self::V3 { .. } => 3,
            Self::V4 { .. } => 4,
        }
    }
}

/// The metadata for an LVD object.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum MetaInfo {
    /// The first version of the `MetaInfo` type.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The version metadata of the object.
        version_info: Versioned<VersionInfo>,

        /// The name of the object.
        name: Versioned<FixedString56>,
    },
}

impl Version for MetaInfo {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
        }
    }
}

/// The version metadata for an LVD object.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum VersionInfo {
    /// The first version of the `VersionInfo` type.
    #[br(pre_assert(version == 1))]
    V1 {
        // TODO: Field documentation.
        editor_version: u32,

        // TODO: Field documentation.
        format_version: u32,
    },
}

impl Version for VersionInfo {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
        }
    }
}
