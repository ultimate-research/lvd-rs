use std::str::{self, FromStr};

use binrw::{binrw, BinRead, BinResult};
use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Version;

/// A nul-terminated string with a fixed capacity of 32 bytes.
pub type LvdFixedString32 = LvdFixedString<32>;

/// A nul-terminated string with a fixed capacity of 56 bytes.
pub type LvdFixedString56 = LvdFixedString<56>;

/// A nul-terminated string with a fixed capacity of 64 bytes.
pub type LvdFixedString64 = LvdFixedString<64>;

/// A nul-terminated string with a fixed capacity.
#[binrw]
#[br(import(version: u8), pre_assert(version == 1))]
#[derive(Debug)]
pub struct LvdFixedString<const N: usize>(#[br(parse_with = read_bytes)] [u8; N]);

impl<const N: usize> LvdFixedString<N> {
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
    /// use lvd_lib::LvdFixedString;
    ///
    /// let s = LvdFixedString::<64>::try_from("COL_00_Floor01").unwrap();
    /// assert_eq!(s.len(), 14);
    /// ```
    pub const fn len(&self) -> usize {
        let mut len = 0;

        while len != self.capacity() {
            if self.0[len] == 0 {
                break;
            }

            len += 1;
        }

        len
    }

    /// Returns this `LvdFixedString`’s capacity, in bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use lvd_lib::LvdFixedString;
    ///
    /// let s = LvdFixedString::<64>::try_from("COL_00_Platform01_through").unwrap();
    /// assert_eq!(s.capacity(), 64);
    /// ```
    pub const fn capacity(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the contained string has a length of zero, and `false` otherwise.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use lvd_lib::LvdFixedString;
    ///
    /// let s = LvdFixedString::<64>::try_from("").unwrap();
    /// assert!(s.is_empty());
    ///
    /// let s = LvdFixedString::<64>::try_from("curve1").unwrap();
    /// assert!(!s.is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.0[0] == 0
    }

    /// Converts the underlying buffer to a string slice if it contains valid UTF-8.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use lvd_lib::LvdFixedString;
    ///
    /// let s = LvdFixedString::<64>::try_from("curve2").unwrap();
    /// assert_eq!(s.to_str().unwrap(), "curve2");
    /// ```
    pub fn to_str(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(&self.0[..self.len()])
    }

    /// Converts the underlying buffer to a [`String`] if it contains valid UTF-8.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use lvd_lib::LvdFixedString;
    ///
    /// let s = LvdFixedString::<64>::try_from("curve3").unwrap();
    /// assert_eq!(s.to_string().unwrap(), "curve3".to_string());
    /// ```
    pub fn to_string(&self) -> Result<String, str::Utf8Error> {
        self.to_str().map(|s| s.to_string())
    }
}

impl<const N: usize> FromStr for LvdFixedString<N> {
    type Err = ToLvdFixedStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() >= N {
            return Err(ToLvdFixedStringError::BufferOverflow(N));
        }

        let bytes = s.as_bytes();
        let mut buffer = [0; N];
        let mut index = 0;

        while index != bytes.len() {
            buffer[index] = bytes[index];
            index += 1;
        }

        Ok(Self(buffer))
    }
}

impl<const N: usize> TryFrom<&str> for LvdFixedString<N> {
    type Error = ToLvdFixedStringError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl<const N: usize> TryFrom<String> for LvdFixedString<N> {
    type Error = ToLvdFixedStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl<const N: usize> TryFrom<&String> for LvdFixedString<N> {
    type Error = ToLvdFixedStringError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl<const N: usize> PartialEq<&str> for LvdFixedString<N> {
    fn eq(&self, other: &&str) -> bool {
        &self.0[..self.len()] == other.as_bytes()
    }
}

impl<const N: usize> PartialEq<String> for LvdFixedString<N> {
    fn eq(&self, other: &String) -> bool {
        &self.0[..self.len()] == other.as_bytes()
    }
}

impl<const N: usize> PartialEq<&String> for LvdFixedString<N> {
    fn eq(&self, other: &&String) -> bool {
        &self.0[..self.len()] == other.as_bytes()
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
impl<const N: usize> Serialize for LvdFixedString<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_str().unwrap())
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize> Deserialize<'de> for LvdFixedString<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;

        Self::from_str(&string).map_err(serde::de::Error::custom)
    }
}

impl<const N: usize> Version for LvdFixedString<N> {
    fn version(&self) -> u8 {
        1
    }
}

/// The error type used when converting a string into a fixed-capacity byte buffer.
#[derive(Debug, Error)]
pub enum ToLvdFixedStringError {
    /// The nul-terminated string exceeds the buffer's capacity.
    #[error("nul-terminated string exceeds buffer capacity of {0} bytes")]
    BufferOverflow(usize),
}

#[cfg(test)]
mod tests {
    use super::*;
    use binrw::{io::Cursor, BinReaderExt, BinWriterExt};

    #[test]
    fn read_lvd_fixed_string() {
        // Test initialized string buffer.
        let mut reader = Cursor::new(b"COL_00_Floor01\0\0");
        let value = reader.read_be_args::<LvdFixedString<16>>((1,)).unwrap();

        assert_eq!(value.to_string().unwrap(), "COL_00_Floor01");

        // Test uninitialized string buffer.
        let mut reader = Cursor::new(b"START_00_P01\0\xFF\xFF\xFF");
        let value = reader.read_be_args::<LvdFixedString<16>>((1,)).unwrap();

        assert_eq!(value.to_string().unwrap(), "START_00_P01");
    }

    #[test]
    fn read_lvd_fixed_string_empty() {
        // Test initialized string buffer.
        let mut reader = Cursor::new(b"\0\0\0\0\0\0\0\0");
        let value = reader.read_be_args::<LvdFixedString<8>>((1,)).unwrap();

        assert_eq!(value.to_string().unwrap(), "");

        // Test uninitialized string buffer.
        let mut reader = Cursor::new(b"\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF");
        let value = reader.read_be_args::<LvdFixedString<8>>((1,)).unwrap();

        assert_eq!(value.to_string().unwrap(), "");
    }

    #[test]
    fn read_lvd_fixed_string_missing_nul() {
        let mut reader = Cursor::new(b"DEATH_00");
        let result = reader.read_be_args::<LvdFixedString<8>>((1,));

        assert!(result.is_err());
    }

    #[test]
    fn lvd_fixed_string_from_str() {
        // Test empty string.
        let value = LvdFixedString::<8>::from_str("").unwrap();
        assert_eq!(value.to_string().unwrap(), "");

        // Test in-bounds string.
        let value = LvdFixedString::<16>::from_str("COL_curve1").unwrap();
        assert_eq!(value.to_string().unwrap(), "COL_curve1");

        // Test out-of-bounds string.
        let value = LvdFixedString::<24>::from_str("GeneralPoint3D__tag____0000_Kir");
        assert!(value.is_err());
    }

    #[test]
    fn write_lvd_fixed_string() {
        let value = LvdFixedString::<8>::from_str("curve1").unwrap();
        let mut writer = Cursor::new(Vec::new());

        writer.write_be(&value).unwrap();

        assert_eq!(writer.into_inner(), b"curve1\0\0");
    }

    #[test]
    fn write_lvd_fixed_string_empty() {
        let value = LvdFixedString::<8>::from_str("").unwrap();
        let mut writer = Cursor::new(Vec::new());

        writer.write_be(&value).unwrap();

        assert_eq!(writer.into_inner(), b"\0\0\0\0\0\0\0\0");
    }
}
