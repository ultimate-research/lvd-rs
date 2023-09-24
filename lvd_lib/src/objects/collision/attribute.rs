//! The `CollisionAttribute` object stores data representing an edge's properties and attributes.
use std::io::{Read, Seek, Write};

use binrw::{binrw, BinRead, BinReaderExt, BinResult, BinWrite, Endian};
use modular_bitfield::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Version;

/// An LVD subobject to [`Collision`](crate::objects::collision::Collision) representing an edge's properties and attributes.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum CollisionAttribute {
    /// `CollisionAttribute` version 1.
    #[br(pre_assert(version == 1))]
    V1 {
        /// Material preset ID defining how an edge is visually, audibly, and physically interacted with.
        material: MaterialType,

        /// Flags for enabling or disabling the corresponding edge's attributes.
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

/// Material preset IDs defining how an edge is visually, audibly, and physically interacted with.
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

/// Flags for enabling or disabling attributes of an edge.
#[bitfield(bits = 64)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(from = "AttributeFlagsSerde", into = "AttributeFlagsSerde")
)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

    #[skip]
    __: B32,
}

impl BinRead for AttributeFlags {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let mut bytes = reader.read_be::<[u8; 8]>()?;

        bytes.reverse();

        Ok(Self::from_bytes(bytes))
    }
}

impl BinWrite for AttributeFlags {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        _endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        let mut bytes = self.into_bytes();

        bytes.reverse();
        writer.write_all(&bytes).map_err(Into::into)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct AttributeFlagsSerde {
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
impl From<AttributeFlagsSerde> for AttributeFlags {
    fn from(f: AttributeFlagsSerde) -> Self {
        AttributeFlags::new()
            .with_length0(f.length0)
            .with_packman_final_ignore(f.packman_final_ignore)
            .with_fall(f.fall)
            .with_ignore_ray_check(f.ignore_ray_check)
            .with_dive(f.dive)
            .with_unpaintable(f.unpaintable)
            .with_item(f.item)
            .with_ignore_fighter_other(f.ignore_fighter_other)
            .with_right(f.right)
            .with_left(f.left)
            .with_upper(f.upper)
            .with_under(f.under)
            .with_not_attach(f.not_attach)
            .with_throughable(f.throughable)
            .with_hang_l(f.hang_l)
            .with_hang_r(f.hang_r)
            .with_ignore_link_from_left(f.ignore_link_from_left)
            .with_cloud(f.cloud)
            .with_ignore_link_from_right(f.ignore_link_from_right)
            .with_not_expand_near_search(f.not_expand_near_search)
            .with_ignore(f.ignore)
            .with_breakable(f.breakable)
            .with_immediate_relanding_ban(f.immediate_relanding_ban)
            .with_ignore_line_type1(f.ignore_line_type1)
            .with_pickel_block(f.pickel_block)
            .with_deceleration(f.deceleration)
            .with_virtual_hit_line_up(f.virtual_hit_line_up)
            .with_virtual_hit_line_left(f.virtual_hit_line_left)
            .with_virtual_hit_line_right(f.virtual_hit_line_right)
            .with_virtual_hit_line_down(f.virtual_hit_line_down)
            .with_virtual_wall_hit_line(f.virtual_wall_hit_line)
            .with_ignore_boss(f.ignore_boss)
    }
}

#[cfg(feature = "serde")]
impl From<AttributeFlags> for AttributeFlagsSerde {
    fn from(f: AttributeFlags) -> Self {
        AttributeFlagsSerde {
            length0: f.length0(),
            packman_final_ignore: f.packman_final_ignore(),
            fall: f.fall(),
            ignore_ray_check: f.ignore_ray_check(),
            dive: f.dive(),
            unpaintable: f.unpaintable(),
            item: f.item(),
            ignore_fighter_other: f.ignore_fighter_other(),
            right: f.right(),
            left: f.left(),
            upper: f.upper(),
            under: f.under(),
            not_attach: f.not_attach(),
            throughable: f.throughable(),
            hang_l: f.hang_l(),
            hang_r: f.hang_r(),
            ignore_link_from_left: f.ignore_link_from_left(),
            cloud: f.cloud(),
            ignore_link_from_right: f.ignore_link_from_right(),
            not_expand_near_search: f.not_expand_near_search(),
            ignore: f.ignore(),
            breakable: f.breakable(),
            immediate_relanding_ban: f.immediate_relanding_ban(),
            ignore_line_type1: f.ignore_line_type1(),
            pickel_block: f.pickel_block(),
            deceleration: f.deceleration(),
            virtual_hit_line_up: f.virtual_hit_line_up(),
            virtual_hit_line_left: f.virtual_hit_line_left(),
            virtual_hit_line_right: f.virtual_hit_line_right(),
            virtual_hit_line_down: f.virtual_hit_line_down(),
            virtual_wall_hit_line: f.virtual_wall_hit_line(),
            ignore_boss: f.ignore_boss(),
        }
    }
}
