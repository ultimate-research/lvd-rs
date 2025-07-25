//! The [`CollisionAttribute`] object stores data representing the properties and attributes of an edge.

use bilge::prelude::*;
use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::version::Version;

/// The properties and attributes of an edge.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum CollisionAttribute {
    /// The first version of the `CollisionAttribute` type.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The material preset representing how the edge is visually, audibly, and physically interacted with.
        material: MaterialType,

        /// The attributes of the edge.
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

/// The material presets representing how an edge is visually, audibly, and physically interacted with.
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

/// The attributes of an edge.
#[bitsize(64)]
#[binrw]
#[br(map = u64::into)]
#[bw(map = |&x| u64::from(x))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(from = "AttributeDataFlags", into = "AttributeDataFlags")
)]
#[derive(DebugBits, Clone, Copy, DefaultBits, Eq, PartialEq, FromBits)]
pub struct AttributeFlags {
    pub length0: bool,
    pub packman_final_ignore: bool,
    pub fall: bool,
    pub ignore_ray_check: bool,
    pub dive: bool,
    pub unpaintable: bool,
    pub item: bool,
    pub ignore_fighter_other: bool,
    pub right: bool,
    pub left: bool,
    pub upper: bool,
    pub under: bool,
    pub not_attach: bool,
    pub throughable: bool,
    pub hang_l: bool,
    pub hang_r: bool,
    pub ignore_link_from_left: bool,
    pub cloud: bool,
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

#[cfg(feature = "serde")]
impl From<AttributeDataFlags> for AttributeFlags {
    fn from(value: AttributeDataFlags) -> Self {
        Self::new(
            value.length0,
            value.packman_final_ignore,
            value.fall,
            value.ignore_ray_check,
            value.dive,
            value.unpaintable,
            value.item,
            value.ignore_fighter_other,
            value.right,
            value.left,
            value.upper,
            value.under,
            value.not_attach,
            value.throughable,
            value.hang_l,
            value.hang_r,
            value.ignore_link_from_left,
            value.cloud,
            value.ignore_link_from_right,
            value.not_expand_near_search,
            value.ignore,
            value.breakable,
            value.immediate_relanding_ban,
            value.ignore_line_type1,
            value.pickel_block,
            value.deceleration,
            value.virtual_hit_line_up,
            value.virtual_hit_line_left,
            value.virtual_hit_line_right,
            value.virtual_hit_line_down,
            value.virtual_wall_hit_line,
            value.ignore_boss,
        )
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct AttributeDataFlags {
    length0: bool,
    packman_final_ignore: bool,
    fall: bool,
    ignore_ray_check: bool,
    dive: bool,
    unpaintable: bool,
    item: bool,
    ignore_fighter_other: bool,
    right: bool,
    left: bool,
    upper: bool,
    under: bool,
    not_attach: bool,
    throughable: bool,
    hang_l: bool,
    hang_r: bool,
    ignore_link_from_left: bool,
    cloud: bool,
    ignore_link_from_right: bool,
    not_expand_near_search: bool,
    ignore: bool,
    breakable: bool,
    immediate_relanding_ban: bool,
    ignore_line_type1: bool,
    pickel_block: bool,
    deceleration: bool,
    virtual_hit_line_up: bool,
    virtual_hit_line_left: bool,
    virtual_hit_line_right: bool,
    virtual_hit_line_down: bool,
    virtual_wall_hit_line: bool,
    ignore_boss: bool,
}

#[cfg(feature = "serde")]
impl From<AttributeFlags> for AttributeDataFlags {
    fn from(value: AttributeFlags) -> Self {
        Self {
            length0: value.length0(),
            packman_final_ignore: value.packman_final_ignore(),
            fall: value.fall(),
            ignore_ray_check: value.ignore_ray_check(),
            dive: value.dive(),
            unpaintable: value.unpaintable(),
            item: value.item(),
            ignore_fighter_other: value.ignore_fighter_other(),
            right: value.right(),
            left: value.left(),
            upper: value.upper(),
            under: value.under(),
            not_attach: value.not_attach(),
            throughable: value.throughable(),
            hang_l: value.hang_l(),
            hang_r: value.hang_r(),
            ignore_link_from_left: value.ignore_link_from_left(),
            cloud: value.cloud(),
            ignore_link_from_right: value.ignore_link_from_right(),
            not_expand_near_search: value.not_expand_near_search(),
            ignore: value.ignore(),
            breakable: value.breakable(),
            immediate_relanding_ban: value.immediate_relanding_ban(),
            ignore_line_type1: value.ignore_line_type1(),
            pickel_block: value.pickel_block(),
            deceleration: value.deceleration(),
            virtual_hit_line_up: value.virtual_hit_line_up(),
            virtual_hit_line_left: value.virtual_hit_line_left(),
            virtual_hit_line_right: value.virtual_hit_line_right(),
            virtual_hit_line_down: value.virtual_hit_line_down(),
            virtual_wall_hit_line: value.virtual_wall_hit_line(),
            ignore_boss: value.ignore_boss(),
        }
    }
}
