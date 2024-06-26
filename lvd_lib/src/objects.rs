//! The supported LVD objects for reading and writing.

pub mod base;
pub mod collision;
pub mod damage_shape;
pub mod enemy_generator;
pub mod field_smash;
pub mod general_shape;
pub mod item_popup;
pub mod point;
pub mod ptrainer;
pub mod region;

pub use collision::Collision;
pub use damage_shape::DamageShape;
pub use enemy_generator::EnemyGenerator;
pub use field_smash::{
    AreaHint, AreaLight, FsAreaCam, FsAreaLock, FsCamLimit, FsItem, FsStartPoint, FsUnknown,
    SplitArea,
};
pub use general_shape::{GeneralShape2, GeneralShape3};
pub use item_popup::ItemPopup;
pub use point::Point;
pub use ptrainer::{PTrainerFloatingFloor, PTrainerRange};
pub use region::Region;
