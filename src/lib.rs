use core::fmt;
use binread::{prelude::*, punctuated::Punctuated, NullString};

#[derive(BinRead, Debug)]
#[br(big, magic = b"\x00\x00\x00\x01\x0D\x01\x4C\x56\x44\x31")]
struct LvdFile {
    collisions: Section<Collision>,
    spawns: Section<Spawn>,
    respawns: Section<Spawn>,
    camera: Section<Bounds>,
    blastzones: Section<Bounds>,
    enemy_generators: UnsupportedSection,
    unk1: UnsupportedSection,
    unk2: UnsupportedSection,
    unk3: UnsupportedSection,
    fs_area_cam: UnsupportedSection,
    fs_cam_limit: UnsupportedSection,
    damage_shapes: UnsupportedSection,
    item_spawners: Section<ItemSpawner>,
    ptrainers: Section<PokemonTrainer>,
    ptrainer_platform: Section<PokemonTrainerPlatform>,
    general_shapes: UnsupportedSection,
    general_points: Section<Point>,
    unk4: UnsupportedSection,
    unk5: UnsupportedSection,
    unk6: UnsupportedSection,
    unk7: UnsupportedSection,
    shrunk_cameras: Section<Bounds>,
    shrunk_blastzones: Section<Bounds>,
}

#[derive(BinRead, Debug)]
#[br(magic = b"\x01\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
struct Point {
    entry: LvdEntry,
    #[br(pad_before = 1)]
    id: u32,
    #[br(pad_before = 1)]
    ty: u32,
    #[br(pad_after = 0x10)]
    pos: Vector3,
}

#[derive(BinRead, Debug)]
#[br(magic = b"\x01\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
struct PokemonTrainerPlatform {
    entry: LvdEntry,
    #[br(pad_before = 1)]
    pos: Vector3,
}

#[derive(BinRead, Debug)]
#[br(magic = b"\x04\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
struct PokemonTrainer {
    entry: LvdEntry,
    #[br(pad_before = 1)]
    boundary_min: Vector3,
    #[br(pad_before = 1)]
    boundary_max: Vector3,
    #[br(pad_before = 1)]
    trainer_count: u32,
    #[br(pad_before = 1, parse_with = Punctuated::separated, count = trainer_count)]
    trainers: Punctuated<Vector3, u8>,
    #[br(pad_before = 1, pad_size_to = 0x40)]
    platform_name: NullString,
    #[br(pad_before = 1, pad_size_to = 0x40)]
    sub_name: NullString,
}

#[derive(BinRead, Debug)]
#[br(magic = b"\x01\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
struct ItemSpawner {
    entry: LvdEntry,
    #[br(pad_before = 1)]
    id: u32,
    unk: u8,
    #[br(pad_before = 1)]
    section_count: u32,
    #[br(pad_before = 1, parse_with = Punctuated::separated, count = section_count)]
    sections: Punctuated<LvdShape, u8>,
}

#[derive(BinRead, Debug)]
enum LvdShape {
    #[br(magic = b"\x03\0\0\0\x01")]
    Point {
        x: f32,
        y: f32,
        #[br(pad_before = 8)]
        unk: u8,
        point_count: u32,
    },
    #[br(magic = b"\x03\0\0\0\x02")]
    Circle {
        x: f32,
        y: f32,
        r: f32,
        #[br(pad_before = 4)]
        unk: u8,
        point_count: u32,
    },
    #[br(magic = b"\x03\0\0\0\x03")]
    Rectangle {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        unk: u8,
        point_count: u32,
    },
    #[br(magic = b"\x03\0\0\0\x04")]
    Path {
        #[br(pad_before = 0x12)]
        point_count: u32,
        #[br(pad_before = 1, parse_with = Punctuated::separated, count = point_count)]
        points: Punctuated<Vector2, u8>
    },
    Invalid {
        magic: u32,
    }
}

#[derive(BinRead, Debug)]
#[br(assert(count == 0))]
struct UnsupportedSection {
    #[br(pad_before = 1)]
    count: u32,
}

#[derive(BinRead, Debug)]
struct Section<T: BinRead<Args=()>> {
    #[br(pad_before = 1)]
    count: u32,
    #[br(count = count)]
    data: Vec<T>,
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
struct Spawn {
    entry: LvdEntry,
    #[br(pad_before = 1)]
    pos: Vector2
}

#[derive(BinRead, Debug)]
#[br(magic = b"\x02\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
struct Bounds {
    entry: LvdEntry,
    #[br(pad_before = 1)]
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

#[derive(BinRead)]
struct Vector3 {
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
struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Vec")
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

fn cbool(x: u8) -> bool {
    x != 0
}

#[derive(BinRead, Debug)]
struct LvdEntry {
    #[br(pad_before = 1, pad_size_to = 0x38)]
    name: NullString,
    #[br(pad_before = 1, pad_size_to = 0x40)]
    subname: NullString,
    #[br(pad_before = 1)]
    start_pos: Vector3,
    #[br(map = cbool)]
    use_start: bool,
    #[br(pad_before = 1)]
    unk: u32,
    #[br(pad_before = 1)]
    unk2: Vector3,
    unk3: u32,
    #[br(pad_before = 1, pad_size_to = 0x40)]
    bone_name: NullString,
}

#[derive(BinRead, Debug)]
struct ColFlags {
    #[br(map = cbool)] flag1: bool,
    #[br(map = cbool)] rig_col: bool,
    #[br(map = cbool)] flag3: bool,
    #[br(map = cbool)] drop_through: bool,
}

type Material = [u8; 0xC];

#[derive(BinRead, Debug)]
#[br(magic = b"\x02\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
struct UnknownEntry {
    entry: LvdEntry,
    unk: u32,
    #[br(pad_before = 1, pad_size_to = 0x40)]
    string: NullString,
    unk2: Vector2,
    unk3: Vector2,
    unk4: [u8; 8],
}

#[derive(BinRead, Debug)]
#[br(magic = b"\x04\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
struct Collision {
    entry: LvdEntry,
    col_flags: ColFlags,
    #[br(pad_before = 1)]
    vert_count: u32,
    #[br(pad_before = 1, parse_with = Punctuated::separated, count = vert_count)]
    verts: Punctuated<Vector2, u8>,
    #[br(pad_before = 1)]
    normal_count: u32,
    #[br(pad_before = 1, parse_with = Punctuated::separated, count = normal_count)]
    normals: Punctuated<Vector2, u8>,
    #[br(pad_before = 1)]
    cliff_count: u32,
    #[br(count = cliff_count)]
    cliffs: Vec<CollisionCliff>,
    #[br(pad_before = 1)]
    mat_count: u32,
    #[br(pad_before = 1, parse_with = Punctuated::separated, count = mat_count)]
    materials: Punctuated<Material, u8>,
    #[br(pad_before = 1)]
    unk_count: u32,
    #[br(count = unk_count)]
    unknowns: Vec<UnknownEntry>,
}

#[derive(BinRead, Debug)]
#[br(magic = b"\x03\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
struct CollisionCliff {
    entry: LvdEntry,
    #[br(pad_before = 1)]
    pos: Vector2,
    angle: f32,
    line_index: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut f = std::fs::File::open("/home/jam/Downloads/village2_00.lvd").unwrap();

        let x: LvdFile = f.read_be().unwrap();
        dbg!(x);
        //dbg!(&x.collisions.collisions[0].verts.seperators);
    }
}
