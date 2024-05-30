//! The [`CollisionFlags`] type represents the global attributes of a collision.
use binrw::binrw;
use modular_bitfield::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Flags for a [`Collision`](crate::objects::collision::Collision) representing the global attributes of a collision.
#[bitfield]
#[binrw]
#[br(map = |f: u32| Self::from_bytes(f.to_le_bytes()))]
#[bw(map = |f: &Self| u32::from_le_bytes(f.into_bytes()))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(from = "CollisionDataFlags", into = "CollisionDataFlags")
)]
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct CollisionFlags {
    /// Boolean flag determining if the collision can be dropped through.
    pub throughable: bool,

    #[skip]
    __: B15,

    /// Boolean flag determining if the collision is dynamic.
    pub dynamic: bool,

    #[skip]
    __: B15,
}

#[cfg(feature = "serde")]
impl From<CollisionDataFlags> for CollisionFlags {
    fn from(value: CollisionDataFlags) -> Self {
        Self::new()
            .with_throughable(value.throughable)
            .with_dynamic(value.dynamic)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct CollisionDataFlags {
    throughable: bool,
    dynamic: bool,
}

#[cfg(feature = "serde")]
impl From<CollisionFlags> for CollisionDataFlags {
    fn from(value: CollisionFlags) -> Self {
        Self {
            throughable: value.throughable(),
            dynamic: value.dynamic(),
        }
    }
}
