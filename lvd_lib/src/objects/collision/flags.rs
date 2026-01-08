//! The [`CollisionFlags`] type represents the global attributes of a collision.

use bilge::prelude::*;
use binrw::binrw;

/// The global attributes of a collision.
#[bitsize(32)]
#[binrw]
#[cfg_attr(feature = "serde", derive(SerializeBits, DeserializeBits))]
#[derive(DebugBits, Clone, Copy, DefaultBits, Eq, PartialEq, FromBits)]
#[repr(transparent)]
pub struct CollisionFlags {
    /// Determines if the collision's floor edges can be dropped through.
    pub throughable: bool,

    reserved: u15,

    /// Determines if the collision is classed as dynamic.
    pub dynamic: bool,

    reserved: u15,
}
