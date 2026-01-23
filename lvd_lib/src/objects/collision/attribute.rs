//! The [`CollisionAttribute`] type stores data representing the properties of an edge.

use bilge::prelude::*;
use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::version::Version;

/// The properties of an edge.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum CollisionAttribute {
    /// The first version of the `CollisionAttribute` type.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The preset for the configuration of sensory collision parameters.
        material: MaterialType,

        /// The collection of packed Boolean collision properties.
        flags: AttributeFlags,
    },
}

impl Version for CollisionAttribute {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
        }
    }
}

/// A preset for the configuration of sensory collision parameters.
#[binrw]
#[brw(repr(u32))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum MaterialType {
    None = 0,
    Rock = 1,
    Grass = 2,
    Soil = 3,
    Wood = 4,
    Iron = 5,
    Nibuiron = 6,
    Carpet = 7,
    Numenume = 8,
    Creature = 9,
    Asase = 10,
    Soft = 11,
    Turuturu = 12,
    Snow = 13,
    Ice = 14,
    Gamewatch = 15,
    Oil = 16,
    Danbouru = 17,
    Damage1 = 18,
    Damage2 = 19,
    Damage3 = 20,
    Plankton = 21,
    Cloud = 22,
    Akuukan = 23,
    Brick = 24,
    Noattr = 25,
    Mario = 26,
    Wirenetting = 27,
    Sand = 28,
    Homerun = 29,
    AsaseEarth = 30,
    Death = 31,
    Ringmat = 32,
    Glass = 33,
    Slipdx = 34,
    SpPoison = 35,
    SpFlame = 36,
    SpElectricShock = 37,
    SpSleep = 38,
    SpFreezing = 39,
    SpAdhesion = 40,
    IceNoSlip = 41,
    CloudNoThrough = 42,
    JackMementoes = 43,
}

/// A collection of packed Boolean collision properties.
#[bitsize(64)]
#[binrw]
#[cfg_attr(feature = "serde", derive(SerializeBits, DeserializeBits))]
#[derive(DebugBits, Clone, Copy, DefaultBits, Eq, PartialEq, FromBits)]
#[repr(transparent)]
pub struct AttributeFlags {
    pub length0: bool,
    pub packman_final_ignore: bool,
    pub fall: bool,

    // TODO: Used on Spring Stadium.
    pub ignore_ray_check: bool,

    pub dive: bool,

    /// Determines if an edge cannot be inked.
    pub unpaintable: bool,

    pub item: bool,
    pub ignore_fighter_other: bool,

    /// Determines if an edge will always register as a right-facing wall.
    pub right: bool,

    /// Determines if an edge will always register as a left-facing wall.
    pub left: bool,

    /// Determines if an edge will always register as a ceiling.
    pub upper: bool,

    /// Determines if an edge will always register as a floor.
    pub under: bool,

    /// Determines if an edge cannot be jumped off or clung to.
    pub not_attach: bool,

    /// Determines if an edge can be dropped through.
    pub throughable: bool,

    /// Determines if an edge can be grabbed from the left.
    pub hang_l: bool,

    /// Determines if an edge can be grabbed from the right.
    pub hang_r: bool,

    // TODO: Used on Moray Towers.
    pub ignore_link_from_left: bool,

    /// Determines if the edge can be passed through when launched at high speeds.
    pub cloud: bool,

    // TODO: Used on Moray Towers.
    pub ignore_link_from_right: bool,

    pub not_expand_near_search: bool,
    pub ignore: bool,
    pub breakable: bool,
    pub immediate_relanding_ban: bool,
    pub ignore_line_type1: bool,
    pub pickel_block: bool,
    pub deceleration: bool,
    pub virtual_hit_line_up: bool,
    pub virtual_hit_line_left: bool,
    pub virtual_hit_line_right: bool,
    pub virtual_hit_line_down: bool,
    pub virtual_wall_hit_line: bool,
    pub ignore_boss: bool,

    reserved: u32,
}
