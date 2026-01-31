//! Essential file format utilities.

use std::{fs, io::Cursor, path::Path};

use binrw::{BinReaderExt, BinResult, BinWrite, binrw};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    array::Array,
    objects::{
        AreaHint, AreaLight, Collision, DamageShape, EnemyGenerator, FsAreaCam, FsAreaLock,
        FsCamLimit, FsItem, FsStartPoint, FsUnknown, GeneralShape2, GeneralShape3, ItemPopup,
        PTrainerFloatingFloor, PTrainerRange, Point, Region, SplitArea,
    },
    version::{Version, Versioned},
};

/// The container type for the various LVD file format versions.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug)]
pub struct LvdFile {
    #[br(temp)]
    #[bw(calc = 1u32)]
    _unk: u32,

    /// The associated data for each LVD file format version.
    pub data: Versioned<Lvd>,
}

impl LvdFile {
    /// Reads the data from the given file path in big-endian.
    pub fn read_be_file<P: AsRef<Path>>(path: P) -> BinResult<Self> {
        let mut cursor = Cursor::new(fs::read(path)?);

        cursor.read_be()
    }

    /// Reads the data from the given file path in little-endian.
    pub fn read_le_file<P: AsRef<Path>>(path: P) -> BinResult<Self> {
        let mut cursor = Cursor::new(fs::read(path)?);

        cursor.read_le()
    }

    /// Writes the data to the given file path in big-endian.
    pub fn write_be_file<P: AsRef<Path>>(&self, path: P) -> BinResult<()> {
        let mut cursor = Cursor::new(Vec::new());

        self.write_be(&mut cursor)?;
        fs::write(path, cursor.get_mut())?;

        Ok(())
    }

    /// Writes the data to the given file path in little-endian.
    pub fn write_le_file<P: AsRef<Path>>(&self, path: P) -> BinResult<()> {
        let mut cursor = Cursor::new(Vec::new());

        self.write_le(&mut cursor)?;
        fs::write(path, cursor.get_mut())?;

        Ok(())
    }
}

/// A signature for serializing an LVD file.
#[binrw]
#[br(import(version: u8), assert(self == Self::LVD1))]
#[derive(Debug, PartialEq)]
enum LvdFileSignature {
    /// The first version of the `LvdFileSignature` type.
    #[br(pre_assert(version == 1))]
    V1 { magic: [u8; 4] },
}

impl LvdFileSignature {
    /// The magic number identifying LVD files.
    const LVD1: Self = Self::V1 { magic: *b"LVD1" };
}

impl Version for LvdFileSignature {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
        }
    }
}

/// The associated data for each LVD file format version.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum Lvd {
    /// The first version of the `Lvd` type.
    ///
    /// This version is not known to be used.
    #[br(pre_assert(version == 1))]
    V1 {
        #[br(temp)]
        #[bw(calc = Versioned::new(LvdFileSignature::LVD1))]
        _signature: Versioned<LvdFileSignature>,

        collisions: Versioned<Array<Collision>>,
        start_positions: Versioned<Array<Point>>,
        restart_positions: Versioned<Array<Point>>,
        camera_regions: Versioned<Array<Region>>,
        death_regions: Versioned<Array<Region>>,
        enemy_generators: Versioned<Array<EnemyGenerator>>,
    },

    /// The second version of the `Lvd` type.
    ///
    /// Adds [`fs_items`](#variant.V2.field.fs_items).
    /// This version is not known to be used.
    #[br(pre_assert(version == 2))]
    V2 {
        #[br(temp)]
        #[bw(calc = Versioned::new(LvdFileSignature::LVD1))]
        _signature: Versioned<LvdFileSignature>,

        collisions: Versioned<Array<Collision>>,
        start_positions: Versioned<Array<Point>>,
        restart_positions: Versioned<Array<Point>>,
        camera_regions: Versioned<Array<Region>>,
        death_regions: Versioned<Array<Region>>,
        enemy_generators: Versioned<Array<EnemyGenerator>>,
        fs_items: Versioned<Array<FsItem>>,
    },

    /// The third version of the `Lvd` type.
    ///
    /// Adds [`fs_unknown`](#variant.V3.field.fs_unknown), [`fs_area_cams`](#variant.V3.field.fs_area_cams), [`fs_area_locks`](#variant.V3.field.fs_area_locks), and [`fs_cam_limits`](#variant.V3.field.fs_cam_limits).
    /// This version is not known to be used.
    #[br(pre_assert(version == 3))]
    V3 {
        #[br(temp)]
        #[bw(calc = Versioned::new(LvdFileSignature::LVD1))]
        _signature: Versioned<LvdFileSignature>,

        collisions: Versioned<Array<Collision>>,
        start_positions: Versioned<Array<Point>>,
        restart_positions: Versioned<Array<Point>>,
        camera_regions: Versioned<Array<Region>>,
        death_regions: Versioned<Array<Region>>,
        enemy_generators: Versioned<Array<EnemyGenerator>>,
        fs_items: Versioned<Array<FsItem>>,
        fs_unknown: Versioned<Array<FsUnknown>>,
        fs_area_cams: Versioned<Array<FsAreaCam>>,
        fs_area_locks: Versioned<Array<FsAreaLock>>,
        fs_cam_limits: Versioned<Array<FsCamLimit>>,
    },

    /// The fourth version of the `Lvd` type.
    ///
    /// Adds [`damage_shapes`](#variant.V4.field.damage_shapes).
    #[br(pre_assert(version == 4))]
    V4 {
        #[br(temp)]
        #[bw(calc = Versioned::new(LvdFileSignature::LVD1))]
        _signature: Versioned<LvdFileSignature>,

        collisions: Versioned<Array<Collision>>,
        start_positions: Versioned<Array<Point>>,
        restart_positions: Versioned<Array<Point>>,
        camera_regions: Versioned<Array<Region>>,
        death_regions: Versioned<Array<Region>>,
        enemy_generators: Versioned<Array<EnemyGenerator>>,
        fs_items: Versioned<Array<FsItem>>,
        fs_unknown: Versioned<Array<FsUnknown>>,
        fs_area_cams: Versioned<Array<FsAreaCam>>,
        fs_area_locks: Versioned<Array<FsAreaLock>>,
        fs_cam_limits: Versioned<Array<FsCamLimit>>,
        damage_shapes: Versioned<Array<DamageShape>>,
    },

    /// The fifth version of the `Lvd` type.
    ///
    /// Adds [`item_popups`](#variant.V5.field.item_popups).
    /// This version is not known to be used.
    #[br(pre_assert(version == 5))]
    V5 {
        #[br(temp)]
        #[bw(calc = Versioned::new(LvdFileSignature::LVD1))]
        _signature: Versioned<LvdFileSignature>,

        collisions: Versioned<Array<Collision>>,
        start_positions: Versioned<Array<Point>>,
        restart_positions: Versioned<Array<Point>>,
        camera_regions: Versioned<Array<Region>>,
        death_regions: Versioned<Array<Region>>,
        enemy_generators: Versioned<Array<EnemyGenerator>>,
        fs_items: Versioned<Array<FsItem>>,
        fs_unknown: Versioned<Array<FsUnknown>>,
        fs_area_cams: Versioned<Array<FsAreaCam>>,
        fs_area_locks: Versioned<Array<FsAreaLock>>,
        fs_cam_limits: Versioned<Array<FsCamLimit>>,
        damage_shapes: Versioned<Array<DamageShape>>,
        item_popups: Versioned<Array<ItemPopup>>,
    },

    /// The sixth version of the `Lvd` type.
    ///
    /// Adds [`general_shapes2`](#variant.V6.field.general_shapes2) and [`general_shapes3`](#variant.V6.field.general_shapes3).
    #[br(pre_assert(version == 6))]
    V6 {
        #[br(temp)]
        #[bw(calc = Versioned::new(LvdFileSignature::LVD1))]
        _signature: Versioned<LvdFileSignature>,

        collisions: Versioned<Array<Collision>>,
        start_positions: Versioned<Array<Point>>,
        restart_positions: Versioned<Array<Point>>,
        camera_regions: Versioned<Array<Region>>,
        death_regions: Versioned<Array<Region>>,
        enemy_generators: Versioned<Array<EnemyGenerator>>,
        fs_items: Versioned<Array<FsItem>>,
        fs_unknown: Versioned<Array<FsUnknown>>,
        fs_area_cams: Versioned<Array<FsAreaCam>>,
        fs_area_locks: Versioned<Array<FsAreaLock>>,
        fs_cam_limits: Versioned<Array<FsCamLimit>>,
        damage_shapes: Versioned<Array<DamageShape>>,
        item_popups: Versioned<Array<ItemPopup>>,
        general_shapes2: Versioned<Array<GeneralShape2>>,
        general_shapes3: Versioned<Array<GeneralShape3>>,
    },

    /// The seventh version of the `Lvd` type.
    ///
    /// Adds [`area_lights`](#variant.V7.field.area_lights).
    /// This version is not known to be used.
    #[br(pre_assert(version == 7))]
    V7 {
        #[br(temp)]
        #[bw(calc = Versioned::new(LvdFileSignature::LVD1))]
        _signature: Versioned<LvdFileSignature>,

        collisions: Versioned<Array<Collision>>,
        start_positions: Versioned<Array<Point>>,
        restart_positions: Versioned<Array<Point>>,
        camera_regions: Versioned<Array<Region>>,
        death_regions: Versioned<Array<Region>>,
        enemy_generators: Versioned<Array<EnemyGenerator>>,
        fs_items: Versioned<Array<FsItem>>,
        fs_unknown: Versioned<Array<FsUnknown>>,
        fs_area_cams: Versioned<Array<FsAreaCam>>,
        fs_area_locks: Versioned<Array<FsAreaLock>>,
        fs_cam_limits: Versioned<Array<FsCamLimit>>,
        damage_shapes: Versioned<Array<DamageShape>>,
        item_popups: Versioned<Array<ItemPopup>>,
        general_shapes2: Versioned<Array<GeneralShape2>>,
        general_shapes3: Versioned<Array<GeneralShape3>>,
        area_lights: Versioned<Array<AreaLight>>,
    },

    /// The eighth version of the `Lvd` type.
    ///
    /// Adds [`fs_start_points`](#variant.V8.field.fs_start_points).
    #[br(pre_assert(version == 8))]
    V8 {
        #[br(temp)]
        #[bw(calc = Versioned::new(LvdFileSignature::LVD1))]
        _signature: Versioned<LvdFileSignature>,

        collisions: Versioned<Array<Collision>>,
        start_positions: Versioned<Array<Point>>,
        restart_positions: Versioned<Array<Point>>,
        camera_regions: Versioned<Array<Region>>,
        death_regions: Versioned<Array<Region>>,
        enemy_generators: Versioned<Array<EnemyGenerator>>,
        fs_items: Versioned<Array<FsItem>>,
        fs_unknown: Versioned<Array<FsUnknown>>,
        fs_area_cams: Versioned<Array<FsAreaCam>>,
        fs_area_locks: Versioned<Array<FsAreaLock>>,
        fs_cam_limits: Versioned<Array<FsCamLimit>>,
        damage_shapes: Versioned<Array<DamageShape>>,
        item_popups: Versioned<Array<ItemPopup>>,
        general_shapes2: Versioned<Array<GeneralShape2>>,
        general_shapes3: Versioned<Array<GeneralShape3>>,
        area_lights: Versioned<Array<AreaLight>>,
        fs_start_points: Versioned<Array<FsStartPoint>>,
    },

    /// The ninth version of the `Lvd` type.
    ///
    /// Adds [`area_hints`](#variant.V9.field.area_hints).
    /// This version is not known to be used.
    #[br(pre_assert(version == 9))]
    V9 {
        #[br(temp)]
        #[bw(calc = Versioned::new(LvdFileSignature::LVD1))]
        _signature: Versioned<LvdFileSignature>,

        collisions: Versioned<Array<Collision>>,
        start_positions: Versioned<Array<Point>>,
        restart_positions: Versioned<Array<Point>>,
        camera_regions: Versioned<Array<Region>>,
        death_regions: Versioned<Array<Region>>,
        enemy_generators: Versioned<Array<EnemyGenerator>>,
        fs_items: Versioned<Array<FsItem>>,
        fs_unknown: Versioned<Array<FsUnknown>>,
        fs_area_cams: Versioned<Array<FsAreaCam>>,
        fs_area_locks: Versioned<Array<FsAreaLock>>,
        fs_cam_limits: Versioned<Array<FsCamLimit>>,
        damage_shapes: Versioned<Array<DamageShape>>,
        item_popups: Versioned<Array<ItemPopup>>,
        general_shapes2: Versioned<Array<GeneralShape2>>,
        general_shapes3: Versioned<Array<GeneralShape3>>,
        area_lights: Versioned<Array<AreaLight>>,
        fs_start_points: Versioned<Array<FsStartPoint>>,
        area_hints: Versioned<Array<AreaHint>>,
    },

    /// The tenth version of the `Lvd` type.
    ///
    /// Adds [`split_areas`](#variant.V10.field.split_areas).
    #[br(pre_assert(version == 10))]
    V10 {
        #[br(temp)]
        #[bw(calc = Versioned::new(LvdFileSignature::LVD1))]
        _signature: Versioned<LvdFileSignature>,

        collisions: Versioned<Array<Collision>>,
        start_positions: Versioned<Array<Point>>,
        restart_positions: Versioned<Array<Point>>,
        camera_regions: Versioned<Array<Region>>,
        death_regions: Versioned<Array<Region>>,
        enemy_generators: Versioned<Array<EnemyGenerator>>,
        fs_items: Versioned<Array<FsItem>>,
        fs_unknown: Versioned<Array<FsUnknown>>,
        fs_area_cams: Versioned<Array<FsAreaCam>>,
        fs_area_locks: Versioned<Array<FsAreaLock>>,
        fs_cam_limits: Versioned<Array<FsCamLimit>>,
        damage_shapes: Versioned<Array<DamageShape>>,
        item_popups: Versioned<Array<ItemPopup>>,
        general_shapes2: Versioned<Array<GeneralShape2>>,
        general_shapes3: Versioned<Array<GeneralShape3>>,
        area_lights: Versioned<Array<AreaLight>>,
        fs_start_points: Versioned<Array<FsStartPoint>>,
        area_hints: Versioned<Array<AreaHint>>,
        split_areas: Versioned<Array<SplitArea>>,
    },

    /// The eleventh version of the `Lvd` type.
    ///
    /// Adds [`shrinked_camera_regions`](#variant.V11.field.shrinked_camera_regions) and [`shrinked_death_regions`](#variant.V11.field.shrinked_death_regions).
    #[br(pre_assert(version == 11))]
    V11 {
        #[br(temp)]
        #[bw(calc = Versioned::new(LvdFileSignature::LVD1))]
        _signature: Versioned<LvdFileSignature>,

        collisions: Versioned<Array<Collision>>,
        start_positions: Versioned<Array<Point>>,
        restart_positions: Versioned<Array<Point>>,
        camera_regions: Versioned<Array<Region>>,
        death_regions: Versioned<Array<Region>>,
        enemy_generators: Versioned<Array<EnemyGenerator>>,
        fs_items: Versioned<Array<FsItem>>,
        fs_unknown: Versioned<Array<FsUnknown>>,
        fs_area_cams: Versioned<Array<FsAreaCam>>,
        fs_area_locks: Versioned<Array<FsAreaLock>>,
        fs_cam_limits: Versioned<Array<FsCamLimit>>,
        damage_shapes: Versioned<Array<DamageShape>>,
        item_popups: Versioned<Array<ItemPopup>>,
        general_shapes2: Versioned<Array<GeneralShape2>>,
        general_shapes3: Versioned<Array<GeneralShape3>>,
        area_lights: Versioned<Array<AreaLight>>,
        fs_start_points: Versioned<Array<FsStartPoint>>,
        area_hints: Versioned<Array<AreaHint>>,
        split_areas: Versioned<Array<SplitArea>>,
        shrinked_camera_regions: Versioned<Array<Region>>,
        shrinked_death_regions: Versioned<Array<Region>>,
    },

    /// The twelfth version of the `Lvd` type.
    ///
    /// Adds [`ptrainer_ranges`](#variant.V12.field.ptrainer_ranges).
    #[br(pre_assert(version == 12))]
    V12 {
        #[br(temp)]
        #[bw(calc = Versioned::new(LvdFileSignature::LVD1))]
        _signature: Versioned<LvdFileSignature>,

        collisions: Versioned<Array<Collision>>,
        start_positions: Versioned<Array<Point>>,
        restart_positions: Versioned<Array<Point>>,
        camera_regions: Versioned<Array<Region>>,
        death_regions: Versioned<Array<Region>>,
        enemy_generators: Versioned<Array<EnemyGenerator>>,
        fs_items: Versioned<Array<FsItem>>,
        fs_unknown: Versioned<Array<FsUnknown>>,
        fs_area_cams: Versioned<Array<FsAreaCam>>,
        fs_area_locks: Versioned<Array<FsAreaLock>>,
        fs_cam_limits: Versioned<Array<FsCamLimit>>,
        damage_shapes: Versioned<Array<DamageShape>>,
        item_popups: Versioned<Array<ItemPopup>>,
        ptrainer_ranges: Versioned<Array<PTrainerRange>>,
        general_shapes2: Versioned<Array<GeneralShape2>>,
        general_shapes3: Versioned<Array<GeneralShape3>>,
        area_lights: Versioned<Array<AreaLight>>,
        fs_start_points: Versioned<Array<FsStartPoint>>,
        area_hints: Versioned<Array<AreaHint>>,
        split_areas: Versioned<Array<SplitArea>>,
        shrinked_camera_regions: Versioned<Array<Region>>,
        shrinked_death_regions: Versioned<Array<Region>>,
    },

    /// The thirteenth version of the `Lvd` type.
    ///
    /// Adds [`ptrainer_floating_floors`](#variant.V13.field.ptrainer_floating_floors).
    #[br(pre_assert(version == 13))]
    V13 {
        #[br(temp)]
        #[bw(calc = Versioned::new(LvdFileSignature::LVD1))]
        _signature: Versioned<LvdFileSignature>,

        collisions: Versioned<Array<Collision>>,
        start_positions: Versioned<Array<Point>>,
        restart_positions: Versioned<Array<Point>>,
        camera_regions: Versioned<Array<Region>>,
        death_regions: Versioned<Array<Region>>,
        enemy_generators: Versioned<Array<EnemyGenerator>>,
        fs_items: Versioned<Array<FsItem>>,
        fs_unknown: Versioned<Array<FsUnknown>>,
        fs_area_cams: Versioned<Array<FsAreaCam>>,
        fs_area_locks: Versioned<Array<FsAreaLock>>,
        fs_cam_limits: Versioned<Array<FsCamLimit>>,
        damage_shapes: Versioned<Array<DamageShape>>,
        item_popups: Versioned<Array<ItemPopup>>,
        ptrainer_ranges: Versioned<Array<PTrainerRange>>,
        ptrainer_floating_floors: Versioned<Array<PTrainerFloatingFloor>>,
        general_shapes2: Versioned<Array<GeneralShape2>>,
        general_shapes3: Versioned<Array<GeneralShape3>>,
        area_lights: Versioned<Array<AreaLight>>,
        fs_start_points: Versioned<Array<FsStartPoint>>,
        area_hints: Versioned<Array<AreaHint>>,
        split_areas: Versioned<Array<SplitArea>>,
        shrinked_camera_regions: Versioned<Array<Region>>,
        shrinked_death_regions: Versioned<Array<Region>>,
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
