//! A nul-terminated string with a fixed capacity.

use std::str::{self, FromStr, Utf8Error};

use binrw::{BinRead, BinResult, binrw};
use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(test)]
mod tests;

use crate::version::Version;

/// A nul-terminated string with a fixed capacity of 32 bytes.
pub type FixedString32 = FixedString<32>;

/// A nul-terminated string with a fixed capacity of 56 bytes.
pub type FixedString56 = FixedString<56>;

/// A nul-terminated string with a fixed capacity of 64 bytes.
pub type FixedString64 = FixedString<64>;

/// A nul-terminated string with a fixed capacity.
#[binrw]
#[br(import(version: u8), pre_assert(version == 1))]
#[derive(Debug)]
#[repr(transparent)]
pub struct FixedString<const N: usize> {
    #[br(parse_with = read_bytes_until_nul)]
    inner: [u8; N],
}

impl<const N: usize> FixedString<N> {
    /// The number of bytes the buffer can hold, including the nul byte.
    pub const CAPACITY: usize = N;

    /// The number of bytes the buffer can hold, excluding the nul byte.
    pub const CAPACITY_WITHOUT_NUL: usize = N - 1;

    /// Creates a new empty `FixedString`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lvd_lib::string::FixedString;
    ///
    /// let s = FixedString::<64>::new();
    /// ```
    pub const fn new() -> Self {
        Self { inner: [0; N] }
    }

    /// Returns the length of the contained string.
    ///
    /// This length is in bytes, not [`char`]s or graphemes. In other words,
    /// it might not be what a human considers the length of the string.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use lvd_lib::string::FixedString;
    ///
    /// let s = FixedString::<64>::try_from("curve1").unwrap();
    /// assert_eq!(s.len(), 6);
    /// ```
    pub const fn len(&self) -> usize {
        let mut len = 0;

        while self.inner[len] != 0 {
            len += 1;
        }

        len
    }

    /// Returns `true` if the contained string has a length of zero, and `false` otherwise.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use lvd_lib::string::FixedString;
    ///
    /// let s = FixedString::<64>::new();
    /// assert!(s.is_empty());
    ///
    /// let s = FixedString::<64>::try_from("transform1").unwrap();
    /// assert!(!s.is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.inner[0] == 0
    }

    /// Converts the underlying buffer to a byte slice.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use lvd_lib::string::FixedString;
    ///
    /// let s = FixedString::<64>::try_from("pPlane1").unwrap();
    /// assert_eq!(s.as_bytes(), b"pPlane1");
    /// ```
    pub fn as_bytes(&self) -> &[u8] {
        &self.inner[..self.len()]
    }

    /// Converts the underlying buffer to a string slice if it contains valid UTF-8.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use lvd_lib::string::FixedString;
    ///
    /// let s = FixedString::<64>::try_from("locator1").unwrap();
    /// assert_eq!(s.to_str().unwrap(), "locator1");
    /// ```
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(self.as_bytes())
    }

    /// Converts the underlying buffer to a [`String`] if it contains valid UTF-8.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use lvd_lib::string::FixedString;
    ///
    /// let s = FixedString::<64>::try_from("locator2").unwrap();
    /// assert_eq!(s.to_string().unwrap(), "locator2".to_string());
    /// ```
    pub fn to_string(&self) -> Result<String, Utf8Error> {
        self.to_str().map(|s| s.to_string())
    }
}

impl<const N: usize> Default for FixedString<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> FromStr for FixedString<N> {
    type Err = ParseFixedStringError<N>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > Self::CAPACITY_WITHOUT_NUL {
            return Err(Self::Err::BufferOverflow);
        }

        let mut buf = [0; N];

        for (index, byte) in s.as_bytes().iter().copied().enumerate() {
            buf[index] = byte;
        }

        Ok(Self { inner: buf })
    }
}

impl<const N: usize> TryFrom<&String> for FixedString<N> {
    type Error = ParseFixedStringError<N>;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl<const N: usize> TryFrom<&str> for FixedString<N> {
    type Error = ParseFixedStringError<N>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl<const N: usize> TryFrom<String> for FixedString<N> {
    type Error = ParseFixedStringError<N>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl<const N: usize> PartialEq for FixedString<N> {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl<const N: usize> PartialEq<&String> for FixedString<N> {
    fn eq(&self, other: &&String) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl<const N: usize> PartialEq<&str> for FixedString<N> {
    fn eq(&self, other: &&str) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl<const N: usize> PartialEq<String> for FixedString<N> {
    fn eq(&self, other: &String) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

#[cfg(feature = "serde")]
impl<const N: usize> Serialize for FixedString<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string = self.to_str().map_err(serde::ser::Error::custom)?;

        serializer.serialize_str(string)
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize> Deserialize<'de> for FixedString<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;

        Self::from_str(&string).map_err(serde::de::Error::custom)
    }
}

impl<const N: usize> Version for FixedString<N> {
    fn version(&self) -> u8 {
        1
    }
}

/// The error type used when converting a string into a [`FixedString`].
#[derive(Debug, PartialEq, Error)]
pub enum ParseFixedStringError<const N: usize> {
    /// The nul-terminated string exceeds the buffer's capacity.
    #[error("nul-terminated string exceeds buffer capacity of {} bytes", N)]
    BufferOverflow,
}

#[binrw::parser(reader)]
fn read_bytes_until_nul<const N: usize>() -> BinResult<[u8; N]> {
    use std::io::SeekFrom;

    let pos = reader.stream_position()?;
    let mut buf = [0; N];
    let mut index = 0;

    while index != N {
        let b = u8::read(reader)?;

        if b == 0 {
            reader.seek(SeekFrom::Start(pos + N as u64))?;

            return Ok(buf);
        }

        buf[index] = b;
        index += 1;
    }

    Err(binrw::Error::AssertFail {
        pos: reader.stream_position()?,
        message: "unable to read beyond the end of the buffer".to_string(),
    })
}
