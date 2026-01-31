//! A fixed-size collection of contiguous versioned elements.

use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::version::{Version, Versioned};

/// A fixed-size collection of contiguous versioned elements.
#[binrw]
#[br(import(version: u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum Array<T: Version + 'static> {
    /// The first version of the `Array` type.
    #[br(pre_assert(version == 1))]
    V1 {
        /// The number of elements.
        #[br(temp)]
        #[bw(calc = elements.len() as u32)]
        count: u32,

        /// The collection of contiguous versioned elements.
        #[br(count = count)]
        elements: Vec<Versioned<T>>,
    },
}

impl<T: Version> Array<T> {
    /// Creates a new empty `Array`.
    pub const fn new() -> Self {
        Self::V1 {
            elements: Vec::new(),
        }
    }

    /// Returns the number of elements in the array.
    pub const fn len(&self) -> usize {
        self.as_slice().len()
    }

    /// Returns `true` if the array contains no elements.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Extracts a slice containing the entire array.
    pub const fn as_slice(&self) -> &[Versioned<T>] {
        match self {
            Self::V1 { elements } => elements.as_slice(),
        }
    }

    /// Extracts a mutable slice of the entire array.
    pub const fn as_mut_slice(&mut self) -> &mut [Versioned<T>] {
        match self {
            Self::V1 { elements } => elements.as_mut_slice(),
        }
    }

    /// Returns a mutable reference to the contents of the array.
    pub const fn as_mut_vec(&mut self) -> &mut Vec<Versioned<T>> {
        match self {
            Self::V1 { elements } => elements,
        }
    }
}

impl<T: Version> Default for Array<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Version> From<Vec<Versioned<T>>> for Array<T> {
    fn from(vec: Vec<Versioned<T>>) -> Self {
        Self::V1 {
            elements: vec,
        }
    }
}

impl<T: Version> Version for Array<T> {
    fn version(&self) -> u8 {
        match self {
            Self::V1 { .. } => 1,
        }
    }
}
