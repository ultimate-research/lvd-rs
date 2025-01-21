//! A nul-terminated string with a fixed capacity.
//!
//! This module contains the [`FixedString`] type, several type aliases for common
//! capacities, and an error type that may result when converting from a string.

use std::str::{self, FromStr, Utf8Error};

use binrw::{binrw, BinRead, BinResult};
use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
pub struct FixedString<const N: usize> {
    #[br(parse_with = read_bytes)]
    inner: [u8; N],
}

impl<const N: usize> FixedString<N> {
    /// The number of bytes the buffer can hold, excluding the nul byte.
    pub const CAPACITY: usize = N - 1;

    /// The number of bytes the buffer can hold, including the nul byte.
    pub const CAPACITY_WITH_NUL: usize = N;

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
    /// let s = FixedString::<64>::try_from("COL_00_Floor01").unwrap();
    /// assert_eq!(s.len(), 14);
    /// ```
    pub const fn len(&self) -> usize {
        let mut len = 0;

        while len != Self::CAPACITY_WITH_NUL {
            if self.inner[len] == 0 {
                break;
            }

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
    /// let s = FixedString::<64>::try_from("curve1").unwrap();
    /// assert!(!s.is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.inner[0] == 0
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
    /// let s = FixedString::<64>::try_from("curve2").unwrap();
    /// assert_eq!(s.to_str().unwrap(), "curve2");
    /// ```
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(&self.inner[..self.len()])
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
    /// let s = FixedString::<64>::try_from("curve3").unwrap();
    /// assert_eq!(s.to_string().unwrap(), "curve3".to_string());
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
        if s.len() >= N {
            return Err(Self::Err::BufferOverflow);
        }

        let mut buffer = [0; N];

        for (index, byte) in s.as_bytes().iter().copied().enumerate() {
            buffer[index] = byte;
        }

        Ok(Self { inner: buffer })
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
        self.inner[..self.len()] == other.inner[..other.len()]
    }
}

impl<const N: usize> PartialEq<&String> for FixedString<N> {
    fn eq(&self, other: &&String) -> bool {
        &self.inner[..self.len()] == other.as_bytes()
    }
}

impl<const N: usize> PartialEq<&str> for FixedString<N> {
    fn eq(&self, other: &&str) -> bool {
        &self.inner[..self.len()] == other.as_bytes()
    }
}

impl<const N: usize> PartialEq<String> for FixedString<N> {
    fn eq(&self, other: &String) -> bool {
        &self.inner[..self.len()] == other.as_bytes()
    }
}

#[binrw::parser(reader)]
fn read_bytes<const N: usize>() -> BinResult<[u8; N]> {
    use std::io::SeekFrom;

    let pos = reader.stream_position()?;
    let mut buffer = [0; N];
    let mut index = 0;

    while index != N {
        let b = u8::read(reader)?;

        if b == 0 {
            reader.seek(SeekFrom::Start(pos + N as u64))?;

            return Ok(buffer);
        }

        buffer[index] = b;
        index += 1;
    }

    Err(binrw::Error::AssertFail {
        pos: reader.stream_position()?,
        message: "unable to read beyond the end of the buffer".to_string(),
    })
}

#[cfg(feature = "serde")]
impl<const N: usize> Serialize for FixedString<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_str().unwrap())
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

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::{io::Cursor, BinReaderExt, BinWriterExt};

    #[test]
    fn read_fixed_string() {
        // Test initialized string buffer.
        let mut reader = Cursor::new(b"COL_00_Floor01\0\0");
        let value = reader.read_be_args::<FixedString<16>>((1,)).unwrap();

        assert_eq!(value.to_string().unwrap(), "COL_00_Floor01");

        // Test uninitialized string buffer.
        let mut reader = Cursor::new(b"START_00_P01\0\xFF\xFF\xFF");
        let value = reader.read_be_args::<FixedString<16>>((1,)).unwrap();

        assert_eq!(value.to_string().unwrap(), "START_00_P01");
    }

    #[test]
    fn read_fixed_string_empty() {
        // Test initialized string buffer.
        let mut reader = Cursor::new(b"\0\0\0\0\0\0\0\0");
        let value = reader.read_be_args::<FixedString<8>>((1,)).unwrap();

        assert_eq!(value.to_string().unwrap(), "");

        // Test uninitialized string buffer.
        let mut reader = Cursor::new(b"\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF");
        let value = reader.read_be_args::<FixedString<8>>((1,)).unwrap();

        assert_eq!(value.to_string().unwrap(), "");
    }

    #[test]
    fn read_fixed_string_missing_nul() {
        let mut reader = Cursor::new(b"DEATH_00");
        let result = reader.read_be_args::<FixedString<8>>((1,));

        assert!(result.is_err());
    }

    #[test]
    fn fixed_string_from_str() {
        // Test empty string.
        let s = "";
        let value = FixedString::<8>::from_str(s).unwrap();
        assert_eq!(value.to_string().unwrap(), s);

        // Test in-bounds string.
        let s = "COL_curve1";
        let value = FixedString::<16>::from_str(s).unwrap();
        assert_eq!(value.to_string().unwrap(), s);

        // Test out-of-bounds string.
        let s = "GeneralPoint3D__tag____0000_Kir";
        let value = FixedString::<24>::from_str(s);
        assert_eq!(value, Err(ParseFixedStringError::<24>::BufferOverflow));
    }

    #[test]
    fn write_fixed_string() {
        let value = FixedString::<8>::from_str("curve1").unwrap();
        let mut writer = Cursor::new(Vec::new());

        writer.write_be(&value).unwrap();

        assert_eq!(writer.into_inner(), b"curve1\0\0");
    }

    #[test]
    fn write_fixed_string_empty() {
        let value = FixedString::<8>::new();
        let mut writer = Cursor::new(Vec::new());

        writer.write_be(&value).unwrap();

        assert_eq!(writer.into_inner(), b"\0\0\0\0\0\0\0\0");
    }
}
