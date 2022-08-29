use binrw::BinRead;
use binrw::{BinWrite, WriteOptions};
use modular_bitfield::prelude::*;

use std::io::Write;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

fn from_bytes(mut x: [u8; 4]) -> LineFlags {
    x.reverse();
    LineFlags::from_bytes(x)
}

/// The various properties of a given collision segment that affect gameplay in non-physics manner, such
/// as whether it is breakable or whether wall clings work on the given segment.
#[bitfield]
#[derive(BinRead, Debug, Clone, Copy)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde_support",
    serde(from = "LineFlagsSerde", into = "LineFlagsSerde")
)]
#[br(map = from_bytes)]
pub struct LineFlags {
    pub length_zero: bool,
    pub pacman_final_ignore: bool,
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
}

impl BinWrite for LineFlags {
    type Args = ();

    fn write_options<W: Write>(
        &self,
        writer: &mut W,
        _: &WriteOptions,
        _: Self::Args,
    ) -> Result<(), binrw::Error> {
        let mut bytes = self.into_bytes();
        bytes.reverse();
        writer.write_all(&bytes).map_err(Into::into)
    }
}

#[cfg(feature = "serde_support")]
#[derive(Serialize, Deserialize)]
struct LineFlagsSerde {
    length_zero: bool,
    pacman_final_ignore: bool,
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

#[cfg(feature = "serde_support")]
impl From<LineFlagsSerde> for LineFlags {
    fn from(lfs: LineFlagsSerde) -> Self {
        LineFlags::new()
            .with_length_zero(lfs.length_zero)
            .with_pacman_final_ignore(lfs.pacman_final_ignore)
            .with_fall(lfs.fall)
            .with_ignore_ray_check(lfs.ignore_ray_check)
            .with_dive(lfs.dive)
            .with_unpaintable(lfs.unpaintable)
            .with_item(lfs.item)
            .with_ignore_fighter_other(lfs.ignore_fighter_other)
            .with_right(lfs.right)
            .with_left(lfs.left)
            .with_upper(lfs.upper)
            .with_under(lfs.under)
            .with_not_attach(lfs.not_attach)
            .with_throughable(lfs.throughable)
            .with_hang_l(lfs.hang_l)
            .with_hang_r(lfs.hang_r)
            .with_ignore_link_from_left(lfs.ignore_link_from_left)
            .with_cloud(lfs.cloud)
            .with_ignore_link_from_right(lfs.ignore_link_from_right)
            .with_not_expand_near_search(lfs.not_expand_near_search)
            .with_ignore(lfs.ignore)
            .with_breakable(lfs.breakable)
            .with_immediate_relanding_ban(lfs.immediate_relanding_ban)
            .with_ignore_line_type1(lfs.ignore_line_type1)
            .with_pickel_block(lfs.pickel_block)
            .with_deceleration(lfs.deceleration)
            .with_virtual_hit_line_up(lfs.virtual_hit_line_up)
            .with_virtual_hit_line_left(lfs.virtual_hit_line_left)
            .with_virtual_hit_line_right(lfs.virtual_hit_line_right)
            .with_virtual_hit_line_down(lfs.virtual_hit_line_down)
            .with_virtual_wall_hit_line(lfs.virtual_wall_hit_line)
            .with_ignore_boss(lfs.ignore_boss)
    }
}

#[cfg(feature = "serde_support")]
impl From<LineFlags> for LineFlagsSerde {
    fn from(lf: LineFlags) -> Self {
        LineFlagsSerde {
            length_zero: lf.length_zero(),
            pacman_final_ignore: lf.pacman_final_ignore(),
            fall: lf.fall(),
            ignore_ray_check: lf.ignore_ray_check(),
            dive: lf.dive(),
            unpaintable: lf.unpaintable(),
            item: lf.item(),
            ignore_fighter_other: lf.ignore_fighter_other(),
            right: lf.right(),
            left: lf.left(),
            upper: lf.upper(),
            under: lf.under(),
            not_attach: lf.not_attach(),
            throughable: lf.throughable(),
            hang_l: lf.hang_l(),
            hang_r: lf.hang_r(),
            ignore_link_from_left: lf.ignore_link_from_left(),
            cloud: lf.cloud(),
            ignore_link_from_right: lf.ignore_link_from_right(),
            not_expand_near_search: lf.not_expand_near_search(),
            ignore: lf.ignore(),
            breakable: lf.breakable(),
            immediate_relanding_ban: lf.immediate_relanding_ban(),
            ignore_line_type1: lf.ignore_line_type1(),
            pickel_block: lf.pickel_block(),
            deceleration: lf.deceleration(),
            virtual_hit_line_up: lf.virtual_hit_line_up(),
            virtual_hit_line_left: lf.virtual_hit_line_left(),
            virtual_hit_line_right: lf.virtual_hit_line_right(),
            virtual_hit_line_down: lf.virtual_hit_line_down(),
            virtual_wall_hit_line: lf.virtual_wall_hit_line(),
            ignore_boss: lf.ignore_boss(),
        }
    }
}
