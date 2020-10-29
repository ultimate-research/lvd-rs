use binread::{prelude::*, punctuated::Punctuated, NullString, derive_binread};
use core::fmt;

mod writer;

#[derive(BinRead, Debug)]
#[br(big, magic = b"\x00\x00\x00\x01\x0D\x01\x4C\x56\x44\x31")]
pub struct LvdFile {
    pub collisions: Section<Collision>,
    pub spawns: Section<Spawn>,
    pub respawns: Section<Spawn>,
    pub camera: Section<Bounds>,
    pub blastzones: Section<Bounds>,
    pub enemy_generators: UnsupportedSection,
    pub unk1: UnsupportedSection,
    pub unk2: UnsupportedSection,
    pub unk3: UnsupportedSection,
    pub fs_area_cam: UnsupportedSection,
    pub fs_cam_limit: UnsupportedSection,
    pub damage_shapes: UnsupportedSection,
    pub item_spawners: Section<ItemSpawner>,
    pub ptrainers: Section<PokemonTrainer>,
    pub ptrainer_platform: Section<PokemonTrainerPlatform>,
    pub general_shapes: UnsupportedSection,
    pub general_points: Section<Point>,
    pub unk4: UnsupportedSection,
    pub unk5: UnsupportedSection,
    pub unk6: UnsupportedSection,
    pub unk7: UnsupportedSection,
    pub shrunk_cameras: Section<Bounds>,
    pub shrunk_blastzones: Section<Bounds>,
}

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

#[derive(BinRead, Debug)]
#[br(magic = b"\x01\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
pub struct PokemonTrainerPlatform {
    pub entry: LvdEntry,
    #[br(pad_before = 1)]
    pub pos: Vector3,
}

#[derive_binread]
#[derive(Debug)]
#[br(magic = b"\x04\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
pub struct PokemonTrainer {
    pub entry: LvdEntry,
    #[br(pad_before = 1)]
    pub boundary_min: Vector3,
    #[br(pad_before = 1)]
    pub boundary_max: Vector3,
    #[br(temp, pad_before = 1)]
    pub trainer_count: u32,
    #[br(pad_before = 1, parse_with = Punctuated::separated, count = trainer_count)]
    pub trainers: Punctuated<Vector3, u8>,
    #[br(pad_before = 1, pad_size_to = 0x40)]
    pub platform_name: NullString,
    #[br(pad_before = 1, pad_size_to = 0x40)]
    pub sub_name: NullString,
}

#[derive_binread]
#[derive(Debug)]
#[br(magic = b"\x01\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
pub struct ItemSpawner {
    pub entry: LvdEntry,
    #[br(pad_before = 1)]
    pub id: u32,
    pub unk: u8,
    #[br(temp, pad_before = 1)]
    pub section_count: u32,
    #[br(pad_before = 1, parse_with = Punctuated::separated, count = section_count)]
    pub sections: Punctuated<LvdShape, u8>,
}

#[derive_binread]
#[derive(Debug)]
pub enum LvdShape {
    #[br(magic = b"\x03\0\0\0\x01")]
    Point {
        x: f32,
        y: f32,
        #[br(temp, pad_before = 8)]
        unk: u8,
        #[br(temp)]
        point_count: u32,
    },
    #[br(magic = b"\x03\0\0\0\x02")]
    Circle {
        x: f32,
        y: f32,
        r: f32,
        #[br(temp, pad_before = 4)]
        unk: u8,
        #[br(temp)]
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
        #[br(temp)]
        point_count: u32,
    },
    #[br(magic = b"\x03\0\0\0\x04")]
    Path {
        #[br(temp, pad_before = 0x12)]
        point_count: u32,
        #[br(pad_before = 1, parse_with = Punctuated::separated, count = point_count)]
        points: Punctuated<Vector2, u8>,
    },
    Invalid {
        magic: u32,
    },
}

#[derive(BinRead, Debug)]
#[br(assert(count == 0))]
pub struct UnsupportedSection {
    #[br(pad_before = 1)]
    pub count: u32,
}

#[derive_binread]
#[derive(Debug)]
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

#[derive(BinRead, Debug)]
#[br(magic = b"\x02\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
pub struct Spawn {
    pub entry: LvdEntry,
    #[br(pad_before = 1)]
    pub pos: Vector2,
}

#[derive(BinRead, Debug)]
#[br(magic = b"\x02\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
pub struct Bounds {
    pub entry: LvdEntry,
    #[br(pad_before = 1)]
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

#[derive(BinRead)]
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

#[derive(BinRead)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Vec").field(&self.x).field(&self.y).finish()
    }
}

fn cbool(x: u8) -> bool {
    x != 0
}

#[derive(BinRead, Debug)]
pub struct LvdEntry {
    #[br(pad_before = 1, pad_size_to = 0x38)]
    pub name: NullString,
    #[br(pad_before = 1, pad_size_to = 0x40)]
    pub subname: NullString,
    #[br(pad_before = 1)]
    pub start_pos: Vector3,
    #[br(map = cbool)]
    pub use_start: bool,
    #[br(pad_before = 1)]
    pub unk: u32,
    #[br(pad_before = 1)]
    pub unk2: Vector3,
    pub unk3: u32,
    #[br(pad_before = 1, pad_size_to = 0x40)]
    pub bone_name: NullString,
}

#[derive(BinRead, Debug)]
pub struct ColFlags {
    #[br(map = cbool)]
    flag1: bool,
    #[br(map = cbool)]
    rig_col: bool,
    #[br(map = cbool)]
    flag3: bool,
    #[br(map = cbool)]
    drop_through: bool,
}

type Material = [u8; 0xC];

#[derive(BinRead, Debug)]
#[br(magic = b"\x02\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
pub struct UnknownEntry {
    pub entry: LvdEntry,
    pub unk: u32,
    #[br(pad_before = 1, pad_size_to = 0x40)]
    pub string: NullString,
    pub unk2: Vector2,
    pub unk3: Vector2,
    pub unk4: [u8; 8],
}

#[derive_binread]
#[derive(Debug)]
#[br(magic = b"\x04\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
pub struct Collision {
    pub entry: LvdEntry,
    pub col_flags: ColFlags,
    #[br(temp, pad_before = 1)]
    pub vert_count: u32,
    #[br(pad_before = 1, parse_with = Punctuated::separated, count = vert_count)]
    pub verts: Punctuated<Vector2, u8>,
    #[br(temp, pad_before = 1)]
    pub normal_count: u32,
    #[br(pad_before = 1, parse_with = Punctuated::separated, count = normal_count)]
    pub normals: Punctuated<Vector2, u8>,
    #[br(temp, pad_before = 1)]
    pub cliff_count: u32,
    #[br(count = cliff_count)]
    pub cliffs: Vec<CollisionCliff>,
    #[br(temp, pad_before = 1)]
    pub mat_count: u32,
    #[br(pad_before = 1, parse_with = Punctuated::separated, count = mat_count)]
    pub materials: Punctuated<Material, u8>,
    #[br(temp, pad_before = 1)]
    pub unk_count: u32,
    #[br(count = unk_count)]
    pub unknowns: Vec<UnknownEntry>,
}

#[derive(BinRead, Debug)]
#[br(magic = b"\x03\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
pub struct CollisionCliff {
    pub entry: LvdEntry,
    #[br(pad_before = 1)]
    pub pos: Vector2,
    pub angle: f32,
    pub line_index: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut f = std::fs::File::open("/home/jam/Downloads/forge.lvd").unwrap();

        let x: LvdFile = f.read_be().unwrap();
        dbg!(x);
        //dbg!(&x.collisions.collisions[0].verts.seperators);
    }
}
