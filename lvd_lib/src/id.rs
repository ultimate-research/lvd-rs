use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Version;

/// A numeric identifier for matching and filtering LVD objects.
#[binrw]
#[br(import(_version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Id(pub u32);

impl Version for Id {
    fn version(&self) -> u8 {
        1
    }
}
