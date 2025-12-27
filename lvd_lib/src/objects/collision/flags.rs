//! The [`CollisionFlags`] type represents the global attributes of a collision.

use bilge::prelude::*;
use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The global attributes of a collision.
#[bitsize(32)]
#[binrw]
#[br(map = u32::into)]
#[bw(map = |&x| u32::from(x))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(from = "ExpandedCollisionFlags", into = "ExpandedCollisionFlags")
)]
#[derive(DebugBits, Clone, Copy, DefaultBits, Eq, PartialEq, FromBits)]
pub struct CollisionFlags {
    /// Determines if the collision's floor edges can be dropped through.
    pub throughable: bool,

    reserved: u15,

    /// Determines if the collision is classed as dynamic.
    pub dynamic: bool,

    reserved: u15,
}

#[cfg(feature = "serde")]
impl From<ExpandedCollisionFlags> for CollisionFlags {
    fn from(value: ExpandedCollisionFlags) -> Self {
        Self::new(value.throughable, value.dynamic)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct ExpandedCollisionFlags {
    throughable: bool,
    dynamic: bool,
}

#[cfg(feature = "serde")]
impl From<CollisionFlags> for ExpandedCollisionFlags {
    fn from(value: CollisionFlags) -> Self {
        Self {
            throughable: value.throughable(),
            dynamic: value.dynamic(),
        }
    }
}
