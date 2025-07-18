//! An identifier for matching and filtering LVD objects.

use std::{array, fmt, str::FromStr};

use binrw::binrw;
use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(test)]
mod tests;

use crate::version::Version;

/// An identifier for matching and filtering LVD objects.
///
/// An example of a `Tag` represented as a string is as follows: `"IPP0001"`
///
/// # Format
///
/// The bit layout of a `Tag` under a big-endian byte order is as follows:
///
/// | **Bit Length**  | 3      | 5      | 5      | 5      | 14     |
/// |-----------------|--------|--------|--------|--------|--------|
/// | **Designation** | Unused | Letter | Letter | Letter | Number |
///
/// # Guidelines
///
/// When converting from a `Tag` to a string, the underlying binary data should
/// follow these guidelines:
///
/// - Letter values must range from 0 to 26, inclusively.
/// - Number value must range from 0 to 9999, inclusively.
///
/// Likewise, when converting from a string to a `Tag`, the string should
/// follow these guidelines:
///
/// - It must have a length of seven bytes.
/// - It must begin with three capital letters, underscores, or any combination
///   of the two.
/// - It must end with four digits.
///
/// If one or more of these guidelines are not met, the corresponding
/// conversion methods will return an error.
#[binrw]
#[br(import(version: u8), pre_assert(version == 1))]
#[derive(Debug, Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Tag(u32);

impl Tag {
    /// The number of characters in the display string.
    const STRING_LEN: usize = 7;

    /// The number of letters in the display string.
    const LETTER_COUNT: usize = 3;

    /// The minimum supported letter character in the display string.
    const LETTER_CHAR_MIN: u8 = b'A';

    /// The maximum supported letter value.
    const LETTER_MAX: u8 = 26;

    /// The bitmasks for each letter.
    const LETTER_MASK: [u32; Self::LETTER_COUNT] = [
        0b00011111_00000000_00000000_00000000,
        0b00000000_11111000_00000000_00000000,
        0b00000000_00000111_11000000_00000000,
    ];

    /// The bit shift operands for each letter.
    const LETTER_SHIFT: [u32; Self::LETTER_COUNT] = [
        Self::LETTER_MASK[0].trailing_zeros(),
        Self::LETTER_MASK[1].trailing_zeros(),
        Self::LETTER_MASK[2].trailing_zeros(),
    ];

    /// The number of digits in the display string.
    const DIGIT_COUNT: usize = 4;

    /// The minimum supported digit character in the display string.
    const DIGIT_CHAR_MIN: u8 = b'0';

    /// The maximum supported digit value.
    const DIGIT_MAX: u8 = 10;

    /// The bitmask for the number.
    const NUMBER_MASK: u32 = 0b00000000_00000000_00111111_11111111;

    /// The maximum supported number value.
    const NUMBER_MAX: u32 = 10000;
}

impl FromStr for Tag {
    type Err = ParseTagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != Self::STRING_LEN {
            return Err(Self::Err::InvalidStringLength(s.len()));
        }

        let (letters, digits) = s.as_bytes().split_at(Self::LETTER_COUNT);

        for letter in letters.iter().copied() {
            if letter == b'_' {
                continue;
            }

            if u8::wrapping_sub(letter, Self::LETTER_CHAR_MIN) < Self::LETTER_MAX {
                continue;
            }

            return Err(Self::Err::LetterNotFound(letter as char));
        }

        for digit in digits.iter().copied() {
            if u8::wrapping_sub(digit, Self::DIGIT_CHAR_MIN) < Self::DIGIT_MAX {
                continue;
            }

            return Err(Self::Err::DigitNotFound(digit as char));
        }

        let letters: [_; Self::LETTER_COUNT] = array::from_fn(|i| match letters[i] {
            b'_' => 0,
            c => c - (Self::LETTER_CHAR_MIN - 1),
        });
        let digits: [_; Self::DIGIT_COUNT] = array::from_fn(|i| digits[i] - Self::DIGIT_CHAR_MIN);

        let word = ((letters[0] as u32) << Self::LETTER_SHIFT[0])
            | ((letters[1] as u32) << Self::LETTER_SHIFT[1])
            | ((letters[2] as u32) << Self::LETTER_SHIFT[2]);
        let number = (digits[0] as u32) * 1000
            + (digits[1] as u32) * 100
            + (digits[2] as u32) * 10
            + digits[3] as u32;

        Ok(Self(word | number))
    }
}

impl TryFrom<&String> for Tag {
    type Error = ParseTagError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<&str> for Tag {
    type Error = ParseTagError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for Tag {
    type Error = ParseTagError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let letters: [_; Self::LETTER_COUNT] =
            array::from_fn(|i| match self.0 & Self::LETTER_MASK[i] {
                0 => '_',
                c => ((c >> Self::LETTER_SHIFT[i]) as u8 + Self::LETTER_CHAR_MIN - 1) as char,
            });
        let number = (self.0 & Self::NUMBER_MASK) % Self::NUMBER_MAX;

        write!(f, "{}{}{}{:04}", letters[0], letters[1], letters[2], number)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Tag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Tag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;

        Self::from_str(&string).map_err(serde::de::Error::custom)
    }
}

impl Version for Tag {
    fn version(&self) -> u8 {
        1
    }
}

/// The error type used when converting a string into a [`Tag`].
#[derive(Debug, PartialEq, Error)]
pub enum ParseTagError {
    /// The string's length did not equal the expected length.
    #[error("expected string length {expected}, found length {0}", expected = Tag::STRING_LEN)]
    InvalidStringLength(usize),

    /// An unexpected character was found in the alphabetical section of the string.
    #[error("expected uppercase letter or underscore, found {0}")]
    LetterNotFound(char),

    /// An unexpected character was found in the numeric section of the string.
    #[error("expected digit, found {0}")]
    DigitNotFound(char),
}
