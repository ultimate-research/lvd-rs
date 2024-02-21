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

/// Common data for an LVD object.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum Base {
    /// `Base` version 1.
    /// This version is not known to be used.
    #[br(pre_assert(version == 1))]
    V1 {
        /// Metadata for the object.
        meta_info: Versioned<MetaInfo>,

        /// Name of the object as seen by the game when flagged as dynamic.
        dynamic_name: Versioned<FixedString64>,
    },

    /// `Base` version 2.
    /// Adds [dynamic_offset](#variant.V2.field.dynamic_offset).
    #[br(pre_assert(version == 2))]
    V2 {
        /// Metadata for the object.
        meta_info: Versioned<MetaInfo>,

        /// Name of the object as seen by the game when flagged as dynamic.
        dynamic_name: Versioned<FixedString64>,

        /// Positional offset of the object when flagged as dynamic.
        dynamic_offset: Versioned<Vector3>,
    },

    /// `Base` version 3.
    /// Adds [is_dynamic](#variant.V3.field.is_dynamic), [instance_id](#variant.V3.field.instance_id), and [instance_offset](#variant.V3.field.instance_offset).
    /// This version is not known to be used.
    #[br(pre_assert(version == 3))]
    V3 {
        /// Metadata for the object.
        meta_info: Versioned<MetaInfo>,

        /// Name of the object as seen by the game when flagged as dynamic.
        dynamic_name: Versioned<FixedString64>,

        /// Positional offset of the object when flagged as dynamic.
        dynamic_offset: Versioned<Vector3>,

        /// Boolean flag determining if the object is dynamic.
        #[br(map = |b: u8| b != 0)]
        #[bw(map = |b| u8::from(*b))]
        is_dynamic: bool,

        /// Numeric ID of the instanced object.
        /// Must be nonzero for the object to be classified as an instanced object.
        instance_id: Versioned<Id>,

        /// Positional offset of the object when classified as an instanced object.
        instance_offset: Versioned<Vector3>,
    },

    /// `Base` version 4.
    /// Adds [joint_index](#variant.V4.field.joint_index) and [joint_name](#variant.V4.field.joint_name).
    #[br(pre_assert(version == 4))]
    V4 {
        /// Metadata for the object.
        meta_info: Versioned<MetaInfo>,

        /// Name of the object as seen by the game when flagged as dynamic.
        dynamic_name: Versioned<FixedString64>,

        /// Positional offset of the object when flagged as dynamic.
        dynamic_offset: Versioned<Vector3>,

        /// Boolean flag determining if the object is dynamic.
        #[br(map = |b: u8| b != 0)]
        #[bw(map = |b| u8::from(*b))]
        is_dynamic: bool,

        /// Numeric ID of the instanced object.
        /// Must be nonzero for the object to be classified as an instanced object.
        instance_id: Versioned<Id>,

        /// Positional offset of the object when classified as an instanced object.
        instance_offset: Versioned<Vector3>,

        /// Index of the target joint from the parent model to parent the dynamic object to.
        joint_index: i32,

        /// Name of the target joint from the parent model to parent the dynamic object to.
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

/// Metadata for an LVD object.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum MetaInfo {
    /// `MetaInfo` version 1.
    #[br(pre_assert(version == 1))]
    V1 {
        /// Version metadata for the object.
        version_info: Versioned<VersionInfo>,

        /// Name of the object.
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

/// Version metadata for an LVD object.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum VersionInfo {
    /// `VersionInfo` version 1.
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
