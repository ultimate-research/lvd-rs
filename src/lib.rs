//! A crate for working with level data files from the Super Smash Brothers Series.
//!
//! ## Minimal Example
//!
//! ```no_run
//! let file = lvd::open("./pickel_world_00.lvd").unwrap();
//!
//! println!("{:?}", &file.collisions[0].vertices[0]);
//!
//! file.save("./pickel_world_00_edited.lvd").unwrap();
//! ```
use binrw::{binread, prelude::*, punctuated::Punctuated, NullString, VecArgs};
use core::fmt;
use std::path::Path;
use writer::c_bool;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

mod writer;

mod line_flags;
pub use line_flags::LineFlags;

fn read_punctuated<T: BinRead<Args = ()>, R: binrw::io::Read + binrw::io::Seek>(
    reader: &mut R,
    options: &binrw::ReadOptions,
    args: VecArgs<()>,
) -> BinResult<Vec<T>> {
    Punctuated::<T, u8>::separated(reader, options, args).map(Punctuated::into_values)
}

/// The top-level structure representing the LVD file containing all the sections included within
/// it.
///
/// ```no_run
/// let file = lvd::open("./pickel_world_00.lvd").unwrap();
///
/// println!("{:?}", &file.collisions[0].vertices[0]);
///
/// file.save("./pickel_world_00_edited.lvd").unwrap();
/// ```
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(BinRead, Debug)]
#[br(big, magic = b"\x00\x00\x00\x01\x0D\x01LVD\x31")]
pub struct LvdFile {
    /// Collisions for the various platforms of the stage.
    ///
    /// These can have arbitrary 2d shapes with ledges, various types of collisions, different
    /// properties, etc. See [`Collision`] for more info.
    pub collisions: Section<Collision>,

    /// The initial spawnpoints of characters when starting the match
    pub spawns: Section<Spawn>,

    /// The points in space where respawn platforms arrive
    pub respawns: Section<Spawn>,

    /// The furthest edges of the stage that the camera can pan to
    pub camera_boundary: Section<Bounds>,

    /// The bounds of how far fighters can go before they die from the blast zones
    pub blast_zone: Section<Bounds>,

    /// Locations where enemies can be spawned
    pub enemy_generators: UnsupportedSection,

    pub unk1: UnsupportedSection,
    pub unk2: UnsupportedSection,
    pub unk3: UnsupportedSection,

    /// Points where the final smash area camera is placed
    pub fs_area_cam: UnsupportedSection,

    /// Limit to panning of the final smash camera
    pub fs_cam_limit: UnsupportedSection,

    /// Places in the stage with hurtboxes attackable by players
    pub damage_shapes: Section<DamageShape>,

    /// Areas of the stage in which items can spawn
    pub item_spawners: Section<ItemSpawner>,

    /// Areas within the stage that pokemon trainers can move around on
    pub ptrainer_ranges: Section<PokemonTrainerRange>, // version 13 only

    /// Platforms where pokemon trainers hover
    pub ptrainer_platforms: Section<PokemonTrainerPlatform>, // version 13 only

    /// Generic shapes describing features of the stage
    pub general_shapes: Section<GeneralShape>,

    /// Generic points describing locations on the stage (used for final smash locations like Ike's, for
    /// example)
    pub general_points: Section<Point>,

    pub unk4: UnsupportedSection,
    pub unk5: UnsupportedSection,
    pub unk6: UnsupportedSection,
    pub unk7: UnsupportedSection,

    /// Camera boundary but after it has shrunken for sudden death
    pub shrunken_camera_boundary: Section<Bounds>, // version 13 only

    /// Blast zone boundary but after it has shrunken for sudden death
    pub shrunken_blast_zone: Section<Bounds>,      // version 13 only
}

/// The generic object data all entries in an LVD file have
#[derive(BinRead, Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct LvdEntry {
    /// The name of the entry
    #[br(pad_before = 1, pad_size_to = 0x38, map = NullString::into_string)]
    pub name: String,

    /// Additional name-like data that specifies certain properties about the entry. Tends to
    /// affect behavior of the object.
    #[br(pad_before = 1, pad_size_to = 0x40, map = NullString::into_string)]
    pub subname: String,

    /// The position the entry starts at, also the origin of the object. Any coordinates under the
    /// object (such as vertices) are **not** relative to this and are absolute world coordinates.
    #[br(pad_before = 1)]
    pub start_pos: Vector3,

    /// Whether or not the `start_pos` data is utilized in the positioning of the object
    #[br(map = cbool)]
    pub use_start: bool,

    #[br(pad_before = 1)]
    pub unk: u32,

    #[br(pad_before = 1)]
    pub unk2: Vector3,

    pub unk3: u32,

    /// The name of the bone the given object is bound to
    #[br(pad_before = 1, pad_size_to = 0x40, map = NullString::into_string)]
    pub bone_name: String,
}

/// A single collision in the level. Includes shape, ledges, collision material, flags about the
/// collision properties, and all other properties of the collision.
#[binread]
#[derive(Debug)]
#[br(magic = b"\x04\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Collision {
    /// The generic object data of the collision
    pub entry: LvdEntry,

    /// The flags specifying certain aspects of the collision behavior (whether the collision is
    /// rigged to an animated bone, whether the platform can be dropped through, etc.)
    pub col_flags: ColFlags,

    #[br(temp, pad_before = 1)]
    pub vertex_count: u32,

    /// The vertices describing the shape of the collision
    #[br(pad_before = 1, parse_with = read_punctuated, count = vertex_count)]
    pub vertices: Vec<Vector2>,

    #[br(temp, pad_before = 1)]
    pub normal_count: u32,

    /// The collision "normals", these are unit vectors describing the direction the collision is
    /// solid from. It effectively points "outward". So a normal of `Vector2(0.0, 1.0)` means the
    /// collision is solid from the top side.
    #[br(pad_before = 1, parse_with = read_punctuated, count = normal_count)]
    pub normals: Vec<Vector2>,

    #[br(temp, pad_before = 1)]
    pub cliff_count: u32,

    /// Data describing grabbable ledges, internally called "cliffs", present in this collision.
    /// This describes how far away the ledge can be grabbed from as well as what vertex/edge the
    /// cliff is a part of.
    #[br(count = cliff_count)]
    pub cliffs: Vec<CollisionCliff>,

    #[br(temp, pad_before = 1)]
    pub line_count: u32,

    /// For each segment in the collision, describe the "material". For example, a slippery stage
    /// may have a `line_material` of `GroundCollAttr::GROUND_COLL_ATTR_ICE`. This has no relation
    /// to graphical materials, only physics/sounds are affected by this.
    #[br(pad_before = 1, parse_with = read_punctuated, count = line_count)]
    pub materials: Vec<CollisionMaterial>,

    #[br(temp, pad_before = 1)]
    pub unk_count: u32,

    #[br(count = unk_count)]
    pub unknowns: Vec<UnknownEntry>,
}

/// The flags specifying certain aspects of the collision behavior (whether the collision is
/// rigged to an animated bone, whether the platform can be dropped through, etc.)
#[derive(BinRead, BinWrite, Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct ColFlags {
    #[br(map = cbool)]
    #[binwrite(map(c_bool))]
    pub flag1: bool,

    /// Whether the collision is rigged to an animated bone
    #[br(map = cbool)]
    #[binwrite(map(c_bool))]
    pub rig_col: bool,

    #[br(map = cbool)]
    #[binwrite(map(c_bool))]
    pub flag3: bool,

    /// Whether characters can press down in order to drop through the collision
    #[br(map = cbool)]
    #[binwrite(map(c_bool))]
    pub drop_through: bool,
}

/// Data describing grabbable ledges, internally called "cliffs", present in this collision.
///
/// This describes how far away the ledge can be grabbed from as well as what vertex/edge the
/// cliff is a part of.
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

/// A type representing the properties of a given collision segment which affects the physics,
/// gameplay effects and sounds used for the segment.
///
/// For example, a slippery stage may have a `line_material` of `GroundCollAttr::GROUND_COLL_ATTR_ICE`.
#[derive(BinRead, Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct CollisionMaterial {
    /// The type of a given ground collision (whether it is ice, wood, rock, wood, metal, etc)
    #[br(pad_after = 4)]
    pub line_material: GroundCollAttr,

    /// The various properties of a given segment that affect gameplay in non-physics manner, such
    /// as whether it is breakable or whether wall clings work on the given segment.
    pub line_flags: LineFlags,
}

/// The type of a given ground collision (whether it is ice, wood, rock, wood, metal, etc)
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

/// A hurtbox present as a part of the level itself. An example being Luigi's Mansion's pillar
/// hurtboxes that allow for parts of the stage to break.
#[derive(BinRead, Debug)]
#[br(magic = b"\x01\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct DamageShape {
    /// The generic object data (positioning, name, etc)
    pub entry: LvdEntry,

    #[br(pad_before = 1)]
    pub unk1: u32,

    #[br(pad_after = 1)]
    pub unk2: [f32; 8],
}

/// Shape data that can be used for various forms of "where is a shape located on the stage"
#[derive(BinRead, Debug)]
#[br(magic = b"\x01\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct GeneralShape {
    /// The generic object data (positioning, name, etc)
    pub entry: LvdEntry,

    #[br(pad_before = 1)]
    pub unk1: u32,

    pub shape: LvdShape,
}

/// Your guess is as good as mine. If you know what this is submit a PR
#[derive(BinRead, Debug)]
#[br(magic = b"\x02\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct UnknownEntry {
    /// The generic object data (positioning, name, etc)
    pub entry: LvdEntry,

    pub unk: u32,

    #[br(pad_before = 1, pad_size_to = 0x40, map = NullString::into_string)]
    pub string: String,

    pub unk2: Vector2,

    pub unk3: Vector2,

    pub unk4: [u8; 8],
}

/// A location for a spawn or respawn point
#[derive(BinRead, Debug)]
#[br(magic = b"\x02\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Spawn {
    /// The generic object data (name, subname, etc)
    pub entry: LvdEntry,

    /// The position of the spawnpoint
    #[br(pad_before = 1)]
    pub pos: Vector2,
}

/// The bounds of a given rectangular area. Used for deathzones (blastzones) and camera pan
/// boundaries.
#[derive(BinRead, Debug)]
#[br(magic = b"\x02\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Bounds {
    /// The generic object data (name, subname, etc)
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
    /// The generic object data (name, subname, etc)
    pub entry: LvdEntry,

    /// ID of the Spawner if needed to be recognized uniquely by the level's specialized code
    #[br(pad_before = 1)]
    pub id: u32,

    pub unk: u8,

    #[br(temp, pad_before = 1)]
    pub section_count: u32,

    /// The various physical areas the item spawner takes up (multiple are allowed)
    #[br(
        pad_before = if section_count > 0 { 
            1
        } else {
            0
        }, 
        parse_with = read_punctuated,
        count = section_count
    )]
    pub sections: Vec<LvdShape>,
}

/// The area of the stage where the Pokemon Trainer can run when attempting to follow the
/// player-controlled pokemon characters. Alternative to [`PokemonTrainerPlatform`], which instead
/// utilizes flying platforms due to not having a good location on the stage itself for them.
#[binread]
#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[br(magic = b"\x04\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
pub struct PokemonTrainerRange {
    /// The generic object data (name, subname, etc)
    pub entry: LvdEntry,

    /// The lower bounds of the pokemon trainer area (usually the left side)
    #[br(pad_before = 1)]
    pub boundary_min: Vector3,

    /// The upper bounds of the pokemon trainer area (usually the right side)
    #[br(pad_before = 1)]
    pub boundary_max: Vector3,

    #[br(temp, pad_before = 1)]
    pub trainer_count: u32,

    /// A list of where all the trainers start when the match begins
    #[br(
        pad_before = if trainer_count > 0 {
            1
        } else {
            0
        },
        parse_with = read_punctuated,
        count = trainer_count
    )]
    pub trainers: Vec<Vector3>,

    /// The name of the platform the pokemon trainer stands on. This is used to ensure the trainer
    /// continues to stay on that platform even if it rudely attempts to leave them.
    #[br(pad_before = 1, pad_size_to = 0x40, map = NullString::into_string)]
    pub platform_name: String,

    /// The subname of the platform the pokemon trainer stands on
    #[br(pad_before = 1, pad_size_to = 0x40, map = NullString::into_string)]
    pub sub_name: String,
}

/// The location on the stage where Pokemon Trainers can float on a cute little platform while
/// watching their pokemon battle to the death. Alternative to [`PokemonTrainerRange`], which
/// utilizes existing models to have the trainers run "on" the stage.
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(BinRead, Debug)]
#[br(magic = b"\x01\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
pub struct PokemonTrainerPlatform {
    /// The generic object data (name, subname, etc)
    pub entry: LvdEntry,

    /// The position of the platform
    #[br(pad_before = 1)]
    pub pos: Vector3,
}

/// A generic location in the level that can be accessed by the specialized level code. Also used
/// for things like points on the stage that character mechanics interact with (usually final
/// smashes).
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(BinRead, Debug)]
#[br(magic = b"\x01\x04\x01\x01\x77\x35\xBB\x75\x00\x00\x00\x02")]
pub struct Point {
    /// The generic object data (name, subname, etc)
    pub entry: LvdEntry,

    /// A unique ID for the point
    #[br(pad_before = 1)]
    pub id: u32,

    /// The type of generic point this is
    #[br(pad_before = 1)]
    pub ty: u32,

    /// The absolute position in the level
    #[br(pad_after = 0x10)]
    pub pos: Vector3,
}

/// A frequently-reused shape type for being able to describe the shape of various objects in LVD
/// files.
#[binread]
#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum LvdShape {
    /// A volume-less point in space
    #[br(magic = b"\x03\0\0\0\x01")]
    Point {
        x: f32,
        y: f32,

        #[br(temp, pad_before = 8)]
        unk: u8,

        /// The number of path points, should also be zero
        #[br(temp, pad_before = 1)]
        point_count: u32,
    },

    /// A 2d circle with no notion of the Z-axis
    #[br(magic = b"\x03\0\0\0\x02")]
    Circle {
        x: f32,
        y: f32,
        radius: f32,

        #[br(temp, pad_before = 4)]
        unk: u8,

        /// The number of path points, should also be zero
        #[br(temp, pad_after = 1)]
        point_count: u32,
    },

    /// A rectangle bound in space
    #[br(magic = b"\x03\0\0\0\x03")]
    Rectangle {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,

        #[br(temp)]
        unk: u8,

        /// The number of path points, should also be zero
        #[br(temp, pad_before = 1)]
        point_count: u32,
    },

    /// A set of points connected by segments describing a path
    #[br(magic = b"\x03\0\0\0\x04")]
    Path {
        #[br(temp, pad_before = 0x12)]
        point_count: u32,

        #[br(pad_before = 1, parse_with = read_punctuated, count = point_count)]
        points: Vec<Vector2>,
    },

    /// An unknown shape. If you know what this is, submit a PR
    Invalid {
        magic: u32,
    },
}

/// A generic type for a section which isn't supported. Submit a PR to add support for any missing
/// sections.
#[derive(BinRead, Debug)]
#[br(assert(count == 0))]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct UnsupportedSection {
    #[br(pad_before = 1)]
    pub count: u32,
}

/// A generic list of items of the same type grouped into a section. Sections in LVD come in a
/// specific order for which type is which.
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

/// A 2d point, size, or direction
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

/// A 3d point, size, or direction
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
    /// Read and parse an LVD from a file at a given path
    ///
    /// ```no_run
    /// use lvd::LvdFile;
    ///
    /// let file = LvdFile::open("path/to/file.lvd").unwrap();
    /// ```
    pub fn open<P: AsRef<Path>>(path: P) -> BinResult<Self> {
        // TODO: make this binrw::io::BufReader
        let mut f = std::io::BufReader::new(std::fs::File::open(path.as_ref())?);

        f.read_be()
    }
}

/// Read and parse an LVD from a file at a given path
///
/// ```no_run
/// let file = lvd::open("path/to/file.lvd").unwrap();
/// ```
pub fn open<P: AsRef<Path>>(path: P) -> BinResult<LvdFile> {
    // TODO: make this binrw::io::BufReader
    let mut f = std::io::BufReader::new(std::fs::File::open(path.as_ref())?);

    f.read_be()
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
