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
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Tag {
    inner: u32,
}

impl Tag {
    /// The number of characters in the display string.
    const STRING_LEN: usize = Self::LETTER_COUNT + Self::DIGIT_COUNT;

    /// The number of letters in the display string.
    const LETTER_COUNT: usize = 3;

    /// The minimum supported letter character in the display string.
    const LETTER_CHAR_MIN: u8 = b'A';

    /// The maximum supported letter character in the display string.
    const LETTER_CHAR_MAX: u8 = b'Z';

    /// The maximum exclusive letter value.
    const LETTER_MAX: u8 = (Self::LETTER_CHAR_MAX - Self::LETTER_CHAR_MIN) + 1;

    /// The bitmask for a letter value.
    const LETTER_MASK: u32 = u32::MAX >> (u32::BITS - Self::LETTER_WIDTH);

    /// The number of bits required to represent each letter value.
    const LETTER_WIDTH: u32 = u32::BITS - (Self::LETTER_MAX as u32).leading_zeros();

    /// The bitmasks for each letter value, in descending order.
    const LETTER_MASKS: [u32; Self::LETTER_COUNT] = [
        Self::LETTER_MASK << Self::LETTER_SHIFTS[0],
        Self::LETTER_MASK << Self::LETTER_SHIFTS[1],
        Self::LETTER_MASK << Self::LETTER_SHIFTS[2],
    ];

    /// The bit shift operands for each letter value, in descending order.
    const LETTER_SHIFTS: [u32; Self::LETTER_COUNT] = [
        Self::NUMBER_WIDTH + Self::LETTER_WIDTH + Self::LETTER_WIDTH,
        Self::NUMBER_WIDTH + Self::LETTER_WIDTH,
        Self::NUMBER_WIDTH,
    ];

    /// The number of digits in the display string.
    const DIGIT_COUNT: usize = 4;

    /// The minimum supported digit character in the display string.
    const DIGIT_CHAR_MIN: u8 = b'0';

    /// The maximum supported digit character in the display string.
    const DIGIT_CHAR_MAX: u8 = b'9';

    /// The maximum exclusive digit value.
    const DIGIT_MAX: u8 = (Self::DIGIT_CHAR_MAX - Self::DIGIT_CHAR_MIN) + 1;

    /// The place value for each digit value, in descending order.
    const DIGIT_PLACE_VALUES: [u32; Self::DIGIT_COUNT] =
        [10u32.pow(3), 10u32.pow(2), 10u32.pow(1), 10u32.pow(0)];

    /// The maximum exclusive number value.
    const NUMBER_MAX: u32 = 10000;

    /// The bitmask for the number value.
    const NUMBER_MASK: u32 = u32::MAX >> (u32::BITS - Self::NUMBER_WIDTH);

    /// The number of bits required to represent the number value.
    const NUMBER_WIDTH: u32 = u32::BITS - Self::NUMBER_MAX.leading_zeros();

    /// Creates a new `Tag` from a raw tag.
    const fn from_raw(tag: u32) -> Self {
        Self { inner: tag }
    }
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

        let word = letters
            .iter()
            .zip(Self::LETTER_SHIFTS.iter())
            .fold(0, |acc, (lhs, rhs)| acc | (*lhs as u32) << rhs);
        let number = digits
            .iter()
            .zip(Self::DIGIT_PLACE_VALUES.iter())
            .map(|(lhs, rhs)| (*lhs as u32) * rhs)
            .sum::<u32>();

        Ok(Self::from_raw(word | number))
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
            array::from_fn(|i| match self.inner & Self::LETTER_MASKS[i] {
                0 => '_',
                c => ((c >> Self::LETTER_SHIFTS[i]) as u8 + Self::LETTER_CHAR_MIN - 1) as char,
            });
        let number = (self.inner & Self::NUMBER_MASK) % Self::NUMBER_MAX;

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
