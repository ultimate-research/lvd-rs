//! # lvd_lib
//!
//! lvd_lib is a library for reading and writing LVD files from Super Smash Bros. for Nintendo 3DS / Wii U and Super Smash Bros. Ultimate.
use std::{
    fs,
    io::{Cursor, Read, Seek, Write},
    path::Path,
};

use binrw::{binrw, BinReaderExt, BinResult, BinWrite};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod array;
pub mod id;
pub mod objects;
pub mod shape;
pub mod string;
pub mod tag;
pub mod vector;
pub mod version;

use array::LvdArray;
use objects::*;
use version::{Version, Versioned};

/// The container type for the various LVD file format versions.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug)]
pub struct LvdFile {
    #[br(temp)]
    #[bw(calc = 1u32)]
    _unk: u32,

    /// Associated data for each LVD file format version.
    pub data: Versioned<Lvd>,
}

impl LvdFile {
    /// Reads the data from the given file path.
    pub fn from_file<P: AsRef<Path>>(path: P) -> BinResult<Self> {
        let mut file = Cursor::new(fs::read(path)?);
        let lvd = file.read_be::<Self>()?;

        Ok(lvd)
    }

    /// Reads the data from the given reader.
    pub fn read<R: Read + Seek>(reader: &mut R) -> BinResult<Self> {
        let lvd = reader.read_be::<Self>()?;

        Ok(lvd)
    }

    /// Writes the data to the given writer.
    pub fn write<W: Write + Seek>(&self, writer: &mut W) -> BinResult<()> {
        self.write_be(writer)
    }

    /// Writes the data to the given file path.
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> BinResult<()> {
        let mut cursor = Cursor::new(Vec::new());

        self.write_be(&mut cursor)?;
        fs::write(path, cursor.get_mut())?;

        Ok(())
    }
}

/// The associated data for each LVD file format version.
#[binrw]
#[br(import(version: u8))]
#[brw(magic = b"\x01LVD1")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum Lvd {
    /// `Lvd` version 1.
    /// This version is not known to be used.
    #[br(pre_assert(version == 1))]
    V1 {
        collisions: Versioned<LvdArray<Collision>>,
        start_positions: Versioned<LvdArray<Point>>,
        restart_positions: Versioned<LvdArray<Point>>,
        camera_regions: Versioned<LvdArray<Region>>,
        death_regions: Versioned<LvdArray<Region>>,
        enemy_generators: Versioned<LvdArray<EnemyGenerator>>,
    },

    /// `Lvd` version 2.
    /// Adds [fs_items](#variant.V2.field.fs_items).
    /// This version is not known to be used.
    #[br(pre_assert(version == 2))]
    V2 {
        collisions: Versioned<LvdArray<Collision>>,
        start_positions: Versioned<LvdArray<Point>>,
        restart_positions: Versioned<LvdArray<Point>>,
        camera_regions: Versioned<LvdArray<Region>>,
        death_regions: Versioned<LvdArray<Region>>,
        enemy_generators: Versioned<LvdArray<EnemyGenerator>>,
        fs_items: Versioned<LvdArray<FsItem>>,
    },

    /// `Lvd` version 3.
    /// Adds [fs_unknown](#variant.V3.field.fs_unknown), [fs_area_cams](#variant.V3.field.fs_area_cams), [fs_area_locks](#variant.V3.field.fs_area_locks), and [fs_cam_limits](#variant.V3.field.fs_cam_limits).
    /// This version is not known to be used.
    #[br(pre_assert(version == 3))]
    V3 {
        collisions: Versioned<LvdArray<Collision>>,
        start_positions: Versioned<LvdArray<Point>>,
        restart_positions: Versioned<LvdArray<Point>>,
        camera_regions: Versioned<LvdArray<Region>>,
        death_regions: Versioned<LvdArray<Region>>,
        enemy_generators: Versioned<LvdArray<EnemyGenerator>>,
        fs_items: Versioned<LvdArray<FsItem>>,
        fs_unknown: Versioned<LvdArray<FsUnknown>>,
        fs_area_cams: Versioned<LvdArray<FsAreaCam>>,
        fs_area_locks: Versioned<LvdArray<FsAreaLock>>,
        fs_cam_limits: Versioned<LvdArray<FsCamLimit>>,
    },

    /// `Lvd` version 4.
    /// Adds [damage_shapes](#variant.V4.field.damage_shapes).
    #[br(pre_assert(version == 4))]
    V4 {
        collisions: Versioned<LvdArray<Collision>>,
        start_positions: Versioned<LvdArray<Point>>,
        restart_positions: Versioned<LvdArray<Point>>,
        camera_regions: Versioned<LvdArray<Region>>,
        death_regions: Versioned<LvdArray<Region>>,
        enemy_generators: Versioned<LvdArray<EnemyGenerator>>,
        fs_items: Versioned<LvdArray<FsItem>>,
        fs_unknown: Versioned<LvdArray<FsUnknown>>,
        fs_area_cams: Versioned<LvdArray<FsAreaCam>>,
        fs_area_locks: Versioned<LvdArray<FsAreaLock>>,
        fs_cam_limits: Versioned<LvdArray<FsCamLimit>>,
        damage_shapes: Versioned<LvdArray<DamageShape>>,
    },

    /// `Lvd` version 5.
    /// Adds [item_popups](#variant.V5.field.item_popups).
    /// This version is not known to be used.
    #[br(pre_assert(version == 5))]
    V5 {
        collisions: Versioned<LvdArray<Collision>>,
        start_positions: Versioned<LvdArray<Point>>,
        restart_positions: Versioned<LvdArray<Point>>,
        camera_regions: Versioned<LvdArray<Region>>,
        death_regions: Versioned<LvdArray<Region>>,
        enemy_generators: Versioned<LvdArray<EnemyGenerator>>,
        fs_items: Versioned<LvdArray<FsItem>>,
        fs_unknown: Versioned<LvdArray<FsUnknown>>,
        fs_area_cams: Versioned<LvdArray<FsAreaCam>>,
        fs_area_locks: Versioned<LvdArray<FsAreaLock>>,
        fs_cam_limits: Versioned<LvdArray<FsCamLimit>>,
        damage_shapes: Versioned<LvdArray<DamageShape>>,
        item_popups: Versioned<LvdArray<ItemPopup>>,
    },

    /// `Lvd` version 6.
    /// Adds [general_shapes2](#variant.V6.field.general_shapes2) and [general_shapes3](#variant.V6.field.general_shapes3).
    #[br(pre_assert(version == 6))]
    V6 {
        collisions: Versioned<LvdArray<Collision>>,
        start_positions: Versioned<LvdArray<Point>>,
        restart_positions: Versioned<LvdArray<Point>>,
        camera_regions: Versioned<LvdArray<Region>>,
        death_regions: Versioned<LvdArray<Region>>,
        enemy_generators: Versioned<LvdArray<EnemyGenerator>>,
        fs_items: Versioned<LvdArray<FsItem>>,
        fs_unknown: Versioned<LvdArray<FsUnknown>>,
        fs_area_cams: Versioned<LvdArray<FsAreaCam>>,
        fs_area_locks: Versioned<LvdArray<FsAreaLock>>,
        fs_cam_limits: Versioned<LvdArray<FsCamLimit>>,
        damage_shapes: Versioned<LvdArray<DamageShape>>,
        item_popups: Versioned<LvdArray<ItemPopup>>,
        general_shapes2: Versioned<LvdArray<GeneralShape2>>,
        general_shapes3: Versioned<LvdArray<GeneralShape3>>,
    },

    /// `Lvd` version 7.
    /// Adds [area_lights](#variant.V7.field.area_lights).
    /// This version is not known to be used.
    #[br(pre_assert(version == 7))]
    V7 {
        collisions: Versioned<LvdArray<Collision>>,
        start_positions: Versioned<LvdArray<Point>>,
        restart_positions: Versioned<LvdArray<Point>>,
        camera_regions: Versioned<LvdArray<Region>>,
        death_regions: Versioned<LvdArray<Region>>,
        enemy_generators: Versioned<LvdArray<EnemyGenerator>>,
        fs_items: Versioned<LvdArray<FsItem>>,
        fs_unknown: Versioned<LvdArray<FsUnknown>>,
        fs_area_cams: Versioned<LvdArray<FsAreaCam>>,
        fs_area_locks: Versioned<LvdArray<FsAreaLock>>,
        fs_cam_limits: Versioned<LvdArray<FsCamLimit>>,
        damage_shapes: Versioned<LvdArray<DamageShape>>,
        item_popups: Versioned<LvdArray<ItemPopup>>,
        general_shapes2: Versioned<LvdArray<GeneralShape2>>,
        general_shapes3: Versioned<LvdArray<GeneralShape3>>,
        area_lights: Versioned<LvdArray<AreaLight>>,
    },

    /// `Lvd` version 8.
    /// Adds [fs_start_points](#variant.V8.field.fs_start_points).
    #[br(pre_assert(version == 8))]
    V8 {
        collisions: Versioned<LvdArray<Collision>>,
        start_positions: Versioned<LvdArray<Point>>,
        restart_positions: Versioned<LvdArray<Point>>,
        camera_regions: Versioned<LvdArray<Region>>,
        death_regions: Versioned<LvdArray<Region>>,
        enemy_generators: Versioned<LvdArray<EnemyGenerator>>,
        fs_items: Versioned<LvdArray<FsItem>>,
        fs_unknown: Versioned<LvdArray<FsUnknown>>,
        fs_area_cams: Versioned<LvdArray<FsAreaCam>>,
        fs_area_locks: Versioned<LvdArray<FsAreaLock>>,
        fs_cam_limits: Versioned<LvdArray<FsCamLimit>>,
        damage_shapes: Versioned<LvdArray<DamageShape>>,
        item_popups: Versioned<LvdArray<ItemPopup>>,
        general_shapes2: Versioned<LvdArray<GeneralShape2>>,
        general_shapes3: Versioned<LvdArray<GeneralShape3>>,
        area_lights: Versioned<LvdArray<AreaLight>>,
        fs_start_points: Versioned<LvdArray<FsStartPoint>>,
    },

    /// `Lvd` version 9.
    /// Adds [area_hints](#variant.V9.field.area_hints).
    /// This version is not known to be used.
    #[br(pre_assert(version == 9))]
    V9 {
        collisions: Versioned<LvdArray<Collision>>,
        start_positions: Versioned<LvdArray<Point>>,
        restart_positions: Versioned<LvdArray<Point>>,
        camera_regions: Versioned<LvdArray<Region>>,
        death_regions: Versioned<LvdArray<Region>>,
        enemy_generators: Versioned<LvdArray<EnemyGenerator>>,
        fs_items: Versioned<LvdArray<FsItem>>,
        fs_unknown: Versioned<LvdArray<FsUnknown>>,
        fs_area_cams: Versioned<LvdArray<FsAreaCam>>,
        fs_area_locks: Versioned<LvdArray<FsAreaLock>>,
        fs_cam_limits: Versioned<LvdArray<FsCamLimit>>,
        damage_shapes: Versioned<LvdArray<DamageShape>>,
        item_popups: Versioned<LvdArray<ItemPopup>>,
        general_shapes2: Versioned<LvdArray<GeneralShape2>>,
        general_shapes3: Versioned<LvdArray<GeneralShape3>>,
        area_lights: Versioned<LvdArray<AreaLight>>,
        fs_start_points: Versioned<LvdArray<FsStartPoint>>,
        area_hints: Versioned<LvdArray<AreaHint>>,
    },

    /// `Lvd` version 10.
    /// Adds [split_areas](#variant.V10.field.split_areas).
    #[br(pre_assert(version == 10))]
    V10 {
        collisions: Versioned<LvdArray<Collision>>,
        start_positions: Versioned<LvdArray<Point>>,
        restart_positions: Versioned<LvdArray<Point>>,
        camera_regions: Versioned<LvdArray<Region>>,
        death_regions: Versioned<LvdArray<Region>>,
        enemy_generators: Versioned<LvdArray<EnemyGenerator>>,
        fs_items: Versioned<LvdArray<FsItem>>,
        fs_unknown: Versioned<LvdArray<FsUnknown>>,
        fs_area_cams: Versioned<LvdArray<FsAreaCam>>,
        fs_area_locks: Versioned<LvdArray<FsAreaLock>>,
        fs_cam_limits: Versioned<LvdArray<FsCamLimit>>,
        damage_shapes: Versioned<LvdArray<DamageShape>>,
        item_popups: Versioned<LvdArray<ItemPopup>>,
        general_shapes2: Versioned<LvdArray<GeneralShape2>>,
        general_shapes3: Versioned<LvdArray<GeneralShape3>>,
        area_lights: Versioned<LvdArray<AreaLight>>,
        fs_start_points: Versioned<LvdArray<FsStartPoint>>,
        area_hints: Versioned<LvdArray<AreaHint>>,
        split_areas: Versioned<LvdArray<SplitArea>>,
    },

    /// `Lvd` version 11.
    /// Adds [shrinked_camera_regions](#variant.V11.field.shrinked_camera_regions) and [shrinked_death_regions](#variant.V11.field.shrinked_death_regions).
    #[br(pre_assert(version == 11))]
    V11 {
        collisions: Versioned<LvdArray<Collision>>,
        start_positions: Versioned<LvdArray<Point>>,
        restart_positions: Versioned<LvdArray<Point>>,
        camera_regions: Versioned<LvdArray<Region>>,
        death_regions: Versioned<LvdArray<Region>>,
        enemy_generators: Versioned<LvdArray<EnemyGenerator>>,
        fs_items: Versioned<LvdArray<FsItem>>,
        fs_unknown: Versioned<LvdArray<FsUnknown>>,
        fs_area_cams: Versioned<LvdArray<FsAreaCam>>,
        fs_area_locks: Versioned<LvdArray<FsAreaLock>>,
        fs_cam_limits: Versioned<LvdArray<FsCamLimit>>,
        damage_shapes: Versioned<LvdArray<DamageShape>>,
        item_popups: Versioned<LvdArray<ItemPopup>>,
        general_shapes2: Versioned<LvdArray<GeneralShape2>>,
        general_shapes3: Versioned<LvdArray<GeneralShape3>>,
        area_lights: Versioned<LvdArray<AreaLight>>,
        fs_start_points: Versioned<LvdArray<FsStartPoint>>,
        area_hints: Versioned<LvdArray<AreaHint>>,
        split_areas: Versioned<LvdArray<SplitArea>>,
        shrinked_camera_regions: Versioned<LvdArray<Region>>,
        shrinked_death_regions: Versioned<LvdArray<Region>>,
    },

    /// `Lvd` version 12.
    /// Adds [ptrainer_ranges](#variant.V12.field.ptrainer_ranges).
    #[br(pre_assert(version == 12))]
    V12 {
        collisions: Versioned<LvdArray<Collision>>,
        start_positions: Versioned<LvdArray<Point>>,
        restart_positions: Versioned<LvdArray<Point>>,
        camera_regions: Versioned<LvdArray<Region>>,
        death_regions: Versioned<LvdArray<Region>>,
        enemy_generators: Versioned<LvdArray<EnemyGenerator>>,
        fs_items: Versioned<LvdArray<FsItem>>,
        fs_unknown: Versioned<LvdArray<FsUnknown>>,
        fs_area_cams: Versioned<LvdArray<FsAreaCam>>,
        fs_area_locks: Versioned<LvdArray<FsAreaLock>>,
        fs_cam_limits: Versioned<LvdArray<FsCamLimit>>,
        damage_shapes: Versioned<LvdArray<DamageShape>>,
        item_popups: Versioned<LvdArray<ItemPopup>>,
        ptrainer_ranges: Versioned<LvdArray<PTrainerRange>>,
        general_shapes2: Versioned<LvdArray<GeneralShape2>>,
        general_shapes3: Versioned<LvdArray<GeneralShape3>>,
        area_lights: Versioned<LvdArray<AreaLight>>,
        fs_start_points: Versioned<LvdArray<FsStartPoint>>,
        area_hints: Versioned<LvdArray<AreaHint>>,
        split_areas: Versioned<LvdArray<SplitArea>>,
        shrinked_camera_regions: Versioned<LvdArray<Region>>,
        shrinked_death_regions: Versioned<LvdArray<Region>>,
    },

    /// `Lvd` version 13.
    /// Adds [ptrainer_floating_floors](#variant.V13.field.ptrainer_floating_floors).
    #[br(pre_assert(version == 13))]
    V13 {
        collisions: Versioned<LvdArray<Collision>>,
        start_positions: Versioned<LvdArray<Point>>,
        restart_positions: Versioned<LvdArray<Point>>,
        camera_regions: Versioned<LvdArray<Region>>,
        death_regions: Versioned<LvdArray<Region>>,
        enemy_generators: Versioned<LvdArray<EnemyGenerator>>,
        fs_items: Versioned<LvdArray<FsItem>>,
        fs_unknown: Versioned<LvdArray<FsUnknown>>,
        fs_area_cams: Versioned<LvdArray<FsAreaCam>>,
        fs_area_locks: Versioned<LvdArray<FsAreaLock>>,
        fs_cam_limits: Versioned<LvdArray<FsCamLimit>>,
        damage_shapes: Versioned<LvdArray<DamageShape>>,
        item_popups: Versioned<LvdArray<ItemPopup>>,
        ptrainer_ranges: Versioned<LvdArray<PTrainerRange>>,
        ptrainer_floating_floors: Versioned<LvdArray<PTrainerFloatingFloor>>,
        general_shapes2: Versioned<LvdArray<GeneralShape2>>,
        general_shapes3: Versioned<LvdArray<GeneralShape3>>,
        area_lights: Versioned<LvdArray<AreaLight>>,
        fs_start_points: Versioned<LvdArray<FsStartPoint>>,
        area_hints: Versioned<LvdArray<AreaHint>>,
        split_areas: Versioned<LvdArray<SplitArea>>,
        shrinked_camera_regions: Versioned<LvdArray<Region>>,
        shrinked_death_regions: Versioned<LvdArray<Region>>,
    },
}

impl Version for Lvd {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
            Self::V2 { .. } => 2,
            Self::V3 { .. } => 3,
            Self::V4 { .. } => 4,
            Self::V5 { .. } => 5,
            Self::V6 { .. } => 6,
            Self::V7 { .. } => 7,
            Self::V8 { .. } => 8,
            Self::V9 { .. } => 9,
            Self::V10 { .. } => 10,
            Self::V11 { .. } => 11,
            Self::V12 { .. } => 12,
            Self::V13 { .. } => 13,
        }
    }
}
