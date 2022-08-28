use binrw::{
    binread, punctuated::Punctuated, BinRead, BinReaderExt, BinResult, NullString, VecArgs,
};
use binwrite::BinWrite;
use core::fmt;
use std::path::Path;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

mod writer;

mod line_flags;
pub use line_flags::LineFlags;

pub fn read_punctuated<T: BinRead<Args = ()>, R: binrw::io::Read + binrw::io::Seek>(
    reader: &mut R,
    options: &binrw::ReadOptions,
    args: VecArgs<()>,
) -> BinResult<Vec<T>> {
    Punctuated::<T, u8>::separated(reader, options, args).map(Punctuated::into_values)
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(BinRead, Debug)]
#[br(big, magic = b"\x00\x00\x00\x01\x0D\x01LVD\x31")]
pub struct LvdFile {
    pub collisions: Section<Collision>,
    pub spawns: Section<Spawn>,
    pub respawns: Section<Spawn>,
    pub camera_boundary: Section<Bounds>,
    pub blast_zone: Section<Bounds>,
    pub enemy_generators: UnsupportedSection,
    pub unk1: UnsupportedSection,
    pub unk2: UnsupportedSection,
    pub unk3: UnsupportedSection,
    pub fs_area_cam: UnsupportedSection,
    pub fs_cam_limit: UnsupportedSection,
    pub damage_shapes: Section<DamageShape>,
    pub item_spawners: Section<ItemSpawner>,
    pub ptrainer_ranges: Section<PokemonTrainerRange>, // version 13 only
    pub ptrainer_platforms: Section<PokemonTrainerPlatform>, // version 13 only
    pub general_shapes: Section<GeneralShape>,
    pub general_points: Section<Point>,
    pub unk4: UnsupportedSection,
    pub unk5: UnsupportedSection,
    pub unk6: UnsupportedSection,
    pub unk7: UnsupportedSection,
    pub shrunken_camera_boundary: Section<Bounds>, // version 13 only
    pub shrunken_blast_zone: Section<Bounds>,      // version 13 only
}

#[derive(BinRead, Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct LvdEntry {
    #[br(pad_before = 1, pad_size_to = 0x38, map = NullString::into_string)]
    pub name: String,
    #[br(pad_before = 1, pad_size_to = 0x40, map = NullString::into_string)]
    pub subname: String,
    #[br(pad_before = 1)]
    pub start_pos: Vector3,
    #[br(map = cbool)]
    pub use_start: bool,
    #[br(pad_before = 1)]
    pub unk: u32,
    #[br(pad_before = 1)]
    pub unk2: Vector3,
    pub unk3: u32,
    #[br(pad_before = 1, pad_size_to = 0x40, map = NullString::into_string)]
    pub bone_name: String,
}

#[binread]
#[derive(Debug)]
#[br(magic = b"\x04\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Collision {
    pub entry: LvdEntry,
    pub col_flags: ColFlags,
    #[br(temp, pad_before = 1)]
    pub vertex_count: u32,
    #[br(pad_before = 1, parse_with = read_punctuated, count = vertex_count)]
    pub vertices: Vec<Vector2>,
    #[br(temp, pad_before = 1)]
    pub normal_count: u32,
    #[br(pad_before = 1, parse_with = read_punctuated, count = normal_count)]
    pub normals: Vec<Vector2>,
    #[br(temp, pad_before = 1)]
    pub cliff_count: u32,
    #[br(count = cliff_count)]
    pub cliffs: Vec<CollisionCliff>,
    #[br(temp, pad_before = 1)]
    pub line_count: u32,
    #[br(pad_before = 1, parse_with = read_punctuated, count = line_count)]
    pub materials: Vec<CollisionMaterial>,
    #[br(temp, pad_before = 1)]
    pub unk_count: u32,
    #[br(count = unk_count)]
    pub unknowns: Vec<UnknownEntry>,
}

use writer::c_bool as to_c_bool;

#[derive(BinRead, BinWrite, Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct ColFlags {
    #[br(map = cbool)]
    #[binwrite(preprocessor(to_c_bool))]
    pub flag1: bool,

    #[br(map = cbool)]
    #[binwrite(preprocessor(to_c_bool))]
    pub rig_col: bool,

    #[br(map = cbool)]
    #[binwrite(preprocessor(to_c_bool))]
    pub flag3: bool,

    #[br(map = cbool)]
    #[binwrite(preprocessor(to_c_bool))]
    pub drop_through: bool,
}

#[derive(BinRead, Debug)]
#[br(magic = b"\x03\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct CollisionCliff {
    pub entry: LvdEntry,
    #[br(pad_before = 1)]
    pub pos: Vector2,
    pub angle: f32,
    pub line_index: i32,
}

#[derive(BinRead, Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct CollisionMaterial {
    #[br(pad_after = 4)]
    pub line_material: GroundCollAttr,
    pub line_flags: LineFlags,
}

#[allow(non_camel_case_types)]
#[derive(BinRead, Debug, Clone, Copy)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[br(repr(u32))]
pub enum GroundCollAttr {
    GROUND_COLL_ATTR_NONE = 0,
    GROUND_COLL_ATTR_ROCK = 1,
    GROUND_COLL_ATTR_GRASS = 2,
    GROUND_COLL_ATTR_SOIL = 3,
    GROUND_COLL_ATTR_WOOD = 4,
    GROUND_COLL_ATTR_IRON = 5,
    GROUND_COLL_ATTR_NIBUIRON = 6,
    GROUND_COLL_ATTR_CARPET = 7,
    GROUND_COLL_ATTR_NUMENUME = 8,
    GROUND_COLL_ATTR_CREATURE = 9,
    GROUND_COLL_ATTR_ASASE = 10,
    GROUND_COLL_ATTR_SOFT = 11,
    GROUND_COLL_ATTR_TURUTURU = 12,
    GROUND_COLL_ATTR_SNOW = 13,
    GROUND_COLL_ATTR_ICE = 14,
    GROUND_COLL_ATTR_GAMEWATCH = 15,
    GROUND_COLL_ATTR_OIL = 16,
    GROUND_COLL_ATTR_DANBOURU = 17,
    GROUND_COLL_ATTR_DAMAGE1 = 18,
    GROUND_COLL_ATTR_DAMAGE2 = 19,
    GROUND_COLL_ATTR_DAMAGE3 = 20,
    GROUND_COLL_ATTR_PLANKTON = 21,
    GROUND_COLL_ATTR_CLOUD = 22,
    GROUND_COLL_ATTR_AKUUKAN = 23,
    GROUND_COLL_ATTR_BRICK = 24,
    GROUND_COLL_ATTR_NOATTR = 25,
    GROUND_COLL_ATTR_MARIO = 26,
    GROUND_COLL_ATTR_WIRENETTING = 27,
    GROUND_COLL_ATTR_SAND = 28,
    GROUND_COLL_ATTR_HOMERUN = 29,
    GROUND_COLL_ATTR_ASASE_EARTH = 30,
    GROUND_COLL_ATTR_DEATH = 31,
    GROUND_COLL_ATTR_RINGMAT = 32,
    GROUND_COLL_ATTR_GLASS = 33,
    GROUND_COLL_ATTR_SLIPDX = 34,
    GROUND_COLL_ATTR_SP_POISON = 35,
    GROUND_COLL_ATTR_SP_FLAME = 36,
    GROUND_COLL_ATTR_SP_ELECTRIC_SHOCK = 37,
    GROUND_COLL_ATTR_SP_SLEEP = 38,
    GROUND_COLL_ATTR_SP_FREEZING = 39,
    GROUND_COLL_ATTR_SP_ADHESION = 40,
    GROUND_COLL_ATTR_ICE_NO_SLIP = 41,
    GROUND_COLL_ATTR_CLOUD_NO_THROUGH = 42,
    GROUND_COLL_ATTR_JACK_MEMENTOES = 43,
}

#[derive(BinRead, Debug)]
#[br(magic = b"\x01\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct DamageShape {
    pub entry: LvdEntry,
    #[br(pad_before = 1)]
    pub unk1: u32,
    #[br(pad_after = 1)]
    pub unk2: [f32; 8],
}

#[derive(BinRead, Debug)]
#[br(magic = b"\x01\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct GeneralShape {
    pub entry: LvdEntry,
    #[br(pad_before = 1)]
    pub unk1: u32,
    pub shape: LvdShape,
}

#[derive(BinRead, Debug)]
#[br(magic = b"\x02\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct UnknownEntry {
    pub entry: LvdEntry,
    pub unk: u32,
    #[br(pad_before = 1, pad_size_to = 0x40, map = NullString::into_string)]
    pub string: String,
    pub unk2: Vector2,
    pub unk3: Vector2,
    pub unk4: [u8; 8],
}

#[derive(BinRead, Debug)]
#[br(magic = b"\x02\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Spawn {
    pub entry: LvdEntry,
    #[br(pad_before = 1)]
    pub pos: Vector2,
}

#[derive(BinRead, Debug)]
#[br(magic = b"\x02\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Bounds {
    pub entry: LvdEntry,
    #[br(pad_before = 1)]
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

#[binread]
#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[br(magic = b"\x01\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
pub struct ItemSpawner {
    pub entry: LvdEntry,
    #[br(pad_before = 1)]
    pub id: u32,
    pub unk: u8,
    #[br(temp, pad_before = 1)]
    pub section_count: u32,
    #[br(pad_before = if section_count > 0 { 1 } else  {0 }, parse_with = read_punctuated, count = section_count)]
    pub sections: Vec<LvdShape>,
}

#[binread]
#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[br(magic = b"\x04\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
pub struct PokemonTrainerRange {
    pub entry: LvdEntry,
    #[br(pad_before = 1)]
    pub boundary_min: Vector3,
    #[br(pad_before = 1)]
    pub boundary_max: Vector3,
    #[br(temp, pad_before = 1)]
    pub trainer_count: u32,
    #[br(pad_before = if trainer_count > 0 { 1 } else { 0 }, parse_with = read_punctuated, count = trainer_count)]
    pub trainers: Vec<Vector3>,
    #[br(pad_before = 1, pad_size_to = 0x40, map = NullString::into_string)]
    pub platform_name: String,
    #[br(pad_before = 1, pad_size_to = 0x40, map = NullString::into_string)]
    pub sub_name: String,
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(BinRead, Debug)]
#[br(magic = b"\x01\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
pub struct PokemonTrainerPlatform {
    pub entry: LvdEntry,
    #[br(pad_before = 1)]
    pub pos: Vector3,
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(BinRead, Debug)]
#[br(magic = b"\x01\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
pub struct Point {
    pub entry: LvdEntry,
    #[br(pad_before = 1)]
    pub id: u32,
    #[br(pad_before = 1)]
    pub ty: u32,
    #[br(pad_after = 0x10)]
    pub pos: Vector3,
}

#[binread]
#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum LvdShape {
    #[br(magic = b"\x03\0\0\0\x01")]
    Point {
        x: f32,
        y: f32,
        #[br(temp, pad_before = 8)]
        unk: u8,
        #[br(temp, pad_before = 1)]
        point_count: u32,
    },
    #[br(magic = b"\x03\0\0\0\x02")]
    Circle {
        x: f32,
        y: f32,
        radius: f32,
        #[br(temp, pad_before = 4)]
        unk: u8,
        #[br(temp, pad_after = 1)]
        point_count: u32,
    },
    #[br(magic = b"\x03\0\0\0\x03")]
    Rectangle {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        #[br(temp)]
        unk: u8,
        #[br(temp, pad_before = 1)]
        point_count: u32,
    },
    #[br(magic = b"\x03\0\0\0\x04")]
    Path {
        #[br(temp, pad_before = 0x12)]
        point_count: u32,
        #[br(pad_before = 1, parse_with = read_punctuated, count = point_count)]
        points: Vec<Vector2>,
    },
    Invalid {
        magic: u32,
    },
}

#[derive(BinRead, Debug)]
#[br(assert(count == 0))]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct UnsupportedSection {
    #[br(pad_before = 1)]
    pub count: u32,
}

#[binread]
#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde_support", serde(transparent))]
pub struct Section<T: BinRead<Args = ()>> {
    #[br(temp, pad_before = 1)]
    pub count: u32,

    #[br(count = count)]
    pub data: Vec<T>,
}

impl<T: BinRead<Args = ()>> core::ops::Deref for Section<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: BinRead<Args = ()>> core::ops::DerefMut for Section<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[derive(BinRead, BinWrite)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Vec").field(&self.x).field(&self.y).finish()
    }
}

#[derive(BinRead, BinWrite)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl fmt::Debug for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Vec")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

fn cbool(x: u8) -> bool {
    x != 0
}

impl LvdFile {
    pub fn open<P: AsRef<Path>>(path: P) -> BinResult<Self> {
        let mut f = std::io::BufReader::new(std::fs::File::open(path.as_ref())?);

        f.read_be()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut f = std::fs::File::open("/home/jam/Downloads/param/pickel_world_00.lvd").unwrap();

        let x: LvdFile = f.read_be().unwrap();
        //dbg!(x);
        //dbg!(&x.collisions.collisions[0].vertices.seperators);
    }
}
