//! The `Base`, `MetaInfo`, and `VersionInfo` types store common data for an LVD object.
use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Bool, Id, LvdFixedString56, LvdFixedString64, Vector3, Version, Versioned};

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
        dynamic_name: Versioned<LvdFixedString64>,
    },

    /// `Base` version 2.
    /// Adds [dynamic_offset](#variant.V2.field.dynamic_offset).
    #[br(pre_assert(version == 2))]
    V2 {
        /// Metadata for the object.
        meta_info: Versioned<MetaInfo>,

        /// Name of the object as seen by the game when flagged as dynamic.
        dynamic_name: Versioned<LvdFixedString64>,

        /// Initial position of the object when flagged as dynamic.
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
        dynamic_name: Versioned<LvdFixedString64>,

        /// Initial position of the object when flagged as dynamic.
        dynamic_offset: Versioned<Vector3>,

        /// Boolean flag determining if the object is dynamic.
        is_dynamic: Bool,

        /// Numeric ID of the instanced object.
        /// Must be nonzero for the object to be considered an instanced object.
        instance_id: Versioned<Id>,

        // TODO: Field documentation.
        instance_offset: Versioned<Vector3>,
    },

    /// `Base` version 4.
    /// Adds [joint_index](#variant.V4.field.joint_index) and [joint_name](#variant.V4.field.joint_name).
    #[br(pre_assert(version == 4))]
    V4 {
        /// Metadata for the object.
        meta_info: Versioned<MetaInfo>,

        /// Name of the object as seen by the game when flagged as dynamic.
        dynamic_name: Versioned<LvdFixedString64>,

        /// Initial position of the object when flagged as dynamic.
        dynamic_offset: Versioned<Vector3>,

        /// Boolean flag determining if the object is dynamic.
        is_dynamic: Bool,

        /// Numeric ID of the instanced object.
        /// Must be nonzero for the object to be considered an instanced object.
        instance_id: Versioned<Id>,

        // TODO: Field documentation.
        instance_offset: Versioned<Vector3>,

        /// Index of a joint from the given [dynamic_name](#variant.V4.field.dynamic_name) which a dynamic object is parented to.
        joint_index: i32,

        /// Name of a joint from the given [dynamic_name](#variant.V4.field.dynamic_name) which a dynamic object is parented to.
        joint_name: Versioned<LvdFixedString64>,
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
        name: Versioned<LvdFixedString56>,
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