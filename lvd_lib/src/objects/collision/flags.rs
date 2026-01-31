//! The [`CollisionFlags`] type represents the global attributes of a [`Collision`](crate::objects::Collision).

use bilge::prelude::*;
use binrw::binrw;

/// The global attributes of a [`Collision`](crate::objects::Collision).
#[bitsize(32)]
#[binrw]
#[cfg_attr(feature = "serde", derive(SerializeBits, DeserializeBits))]
#[derive(DebugBits, Clone, Copy, DefaultBits, FromBits, PartialEq, Eq)]
#[repr(transparent)]
pub struct CollisionFlags {
    /// Determines if all the edges can be dropped through.
    pub throughable: bool,

    reserved: u15,

    /// Determines if the object assumes additional privileges.
    pub dynamic: bool,

    reserved: u15,
}
