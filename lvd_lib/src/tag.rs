//! An identifier for matching and filtering LVD objects.
//!
//! This module contains the [`Tag`] type and an error type that may result when converting from a string.
use std::{array, fmt, str::FromStr};

use binrw::binrw;
use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
/// When converting from a `Tag` to a string, the underlying binary data should follow these guidelines:
///
/// - Letter values must range from 0 to 26, inclusively.
/// - Number must range from 0 to 9999, inclusively.
///
/// Likewise, when converting from a string to a `Tag`, the string should follow these guidelines:
///
/// - Must have a length of seven bytes.
/// - Must begin with three capital letters, underscores, or any combination of the two.
/// - Must end with four digits.
///
/// If one or more of these guidelines are not met, the corresponding
/// conversion methods may return erroneous results.
#[binrw]
#[br(import(version: u8), pre_assert(version == 1))]
#[derive(Debug, Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Tag(u32);

impl Tag {
    /// The number of characters in a tag.
    const STRING_LEN: usize = 7;

    /// The number of letters in a tag.
    const LETTER_COUNT: usize = 3;

    /// The minimum supported letter character in a tag.
    const LETTER_CHAR_MIN: u8 = b'A';

    /// The maximum supported letter value in a tag.
    const LETTER_MAX: u8 = 26;

    /// The bitmasks for each letter in a tag.
    const LETTER_MASK: [u32; Self::LETTER_COUNT] = [
        0b00011111_00000000_00000000_00000000,
        0b00000000_11111000_00000000_00000000,
        0b00000000_00000111_11000000_00000000,
    ];

    /// The bit shift operands for each letter in a tag.
    const LETTER_SHIFT: [u32; Self::LETTER_COUNT] = [
        Self::LETTER_MASK[0].trailing_zeros(),
        Self::LETTER_MASK[1].trailing_zeros(),
        Self::LETTER_MASK[2].trailing_zeros(),
    ];

    /// The number of digits in a tag.
    const DIGIT_COUNT: usize = 4;

    /// The minimum supported digit character in a tag.
    const DIGIT_CHAR_MIN: u8 = b'0';

    /// The maximum supported digit value in a tag.
    const DIGIT_MAX: u8 = 10;

    /// The bitmask for the number in a tag.
    const NUMBER_MASK: u32 = 0b00000000_00000000_00111111_11111111;
}

impl FromStr for Tag {
    type Err = FromStrError;

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

        let word = (letters[0] as u32) << Self::LETTER_SHIFT[0]
            | (letters[1] as u32) << Self::LETTER_SHIFT[1]
            | (letters[2] as u32) << Self::LETTER_SHIFT[2];
        let number = (digits[0] as u32) * 1000
            + (digits[1] as u32) * 100
            + (digits[2] as u32) * 10
            + digits[3] as u32;

        Ok(Self(word | number))
    }
}

impl TryFrom<&String> for Tag {
    type Error = FromStrError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<&str> for Tag {
    type Error = FromStrError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for Tag {
    type Error = FromStrError;

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
        let number = self.0 & Self::NUMBER_MASK;

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
pub enum FromStrError {
    /// The string's length did not equal the expected length.
    #[error("expected string length {}, found length {0}", Tag::STRING_LEN)]
    InvalidStringLength(usize),

    /// An unexpected character was found in the alphabetical section of the string.
    #[error("expected uppercase letter or underscore, found {0}")]
    LetterNotFound(char),

    /// An unexpected character was found in the numeric section of the string.
    #[error("expected digit, found {0}")]
    DigitNotFound(char),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_to_id() {
        assert_eq!(Tag::from_str("IPP0001"), Ok(Tag(159645697)));
        assert_eq!(Tag::from_str("IPP0002"), Ok(Tag(159645698)));
        assert_eq!(Tag::from_str("FSP0010"), Ok(Tag(110886922)));
        assert_eq!(Tag::from_str("FSP0011"), Ok(Tag(110886923)));
        assert_eq!(Tag::from_str("PAL0101"), Ok(Tag(269156453)));
        assert_eq!(Tag::from_str("PAL0102"), Ok(Tag(269156454)));
        assert_eq!(Tag::from_str("SLD1001"), Ok(Tag(325125097)));
        assert_eq!(Tag::from_str("SLD1002"), Ok(Tag(325125098)));
        assert_eq!(Tag::from_str("AAA0000"), Ok(Tag(17317888)));
        assert_eq!(Tag::from_str("ZZZ9999"), Ok(Tag(450275087)));
        assert_eq!(Tag::from_str("C_Y0001"), Ok(Tag(50741249)));
        assert_eq!(Tag::from_str("SE_0001"), Ok(Tag(321388545)));
        assert_eq!(Tag::from_str("___0000"), Ok(Tag(0)));
        assert_eq!(Tag::from_str("___0001"), Ok(Tag(1)));
    }

    #[test]
    fn id_to_label() {
        assert_eq!(Tag(159645697).to_string(), "IPP0001");
        assert_eq!(Tag(159645698).to_string(), "IPP0002");
        assert_eq!(Tag(110886922).to_string(), "FSP0010");
        assert_eq!(Tag(110886923).to_string(), "FSP0011");
        assert_eq!(Tag(269156453).to_string(), "PAL0101");
        assert_eq!(Tag(269156454).to_string(), "PAL0102");
        assert_eq!(Tag(325125097).to_string(), "SLD1001");
        assert_eq!(Tag(325125098).to_string(), "SLD1002");
        assert_eq!(Tag(17317888).to_string(), "AAA0000");
        assert_eq!(Tag(450275087).to_string(), "ZZZ9999");
        assert_eq!(Tag(50741249).to_string(), "C_Y0001");
        assert_eq!(Tag(321388545).to_string(), "SE_0001");
        assert_eq!(Tag(0).to_string(), "___0000");
        assert_eq!(Tag(1).to_string(), "___0001");
    }

    #[test]
    fn use_invalid_string_length() {
        let s = "";
        assert_eq!(
            Tag::from_str(s),
            Err(FromStrError::InvalidStringLength(s.len()))
        );

        let s = "I";
        assert_eq!(
            Tag::from_str(s),
            Err(FromStrError::InvalidStringLength(s.len()))
        );

        let s = "IP";
        assert_eq!(
            Tag::from_str(s),
            Err(FromStrError::InvalidStringLength(s.len()))
        );

        let s = "IPP";
        assert_eq!(
            Tag::from_str(s),
            Err(FromStrError::InvalidStringLength(s.len()))
        );

        let s = "IPP0";
        assert_eq!(
            Tag::from_str(s),
            Err(FromStrError::InvalidStringLength(s.len()))
        );

        let s = "IPP00";
        assert_eq!(
            Tag::from_str(s),
            Err(FromStrError::InvalidStringLength(s.len()))
        );

        let s = "IPP000";
        assert_eq!(
            Tag::from_str(s),
            Err(FromStrError::InvalidStringLength(s.len()))
        );

        let s = "IPP00001";
        assert_eq!(
            Tag::from_str(s),
            Err(FromStrError::InvalidStringLength(s.len()))
        );
    }

    #[test]
    fn use_unsupported_character_letter() {
        // Test lowercase letters
        assert_eq!(
            Tag::from_str("bLK0001"),
            Err(FromStrError::LetterNotFound('b'))
        );
        assert_eq!(
            Tag::from_str("BlK0001"),
            Err(FromStrError::LetterNotFound('l'))
        );
        assert_eq!(
            Tag::from_str("BLk0001"),
            Err(FromStrError::LetterNotFound('k'))
        );
        // Test unsupported characters
        assert_eq!(
            Tag::from_str("@LK0000"),
            Err(FromStrError::LetterNotFound('@'))
        );
        assert_eq!(
            Tag::from_str("B[L0000"),
            Err(FromStrError::LetterNotFound('['))
        );
        // Test digits in alphabetical portion
        assert_eq!(
            Tag::from_str("0LK0000"),
            Err(FromStrError::LetterNotFound('0'))
        );
        assert_eq!(
            Tag::from_str("B1K0000"),
            Err(FromStrError::LetterNotFound('1'))
        );
        assert_eq!(
            Tag::from_str("BL20000"),
            Err(FromStrError::LetterNotFound('2'))
        );
    }

    #[test]
    fn use_unsupported_character_digit() {
        // Test unsupported characters
        assert_eq!(
            Tag::from_str("RNG/001"),
            Err(FromStrError::DigitNotFound('/'))
        );
        assert_eq!(
            Tag::from_str("RNG0:01"),
            Err(FromStrError::DigitNotFound(':'))
        );
        // Test letters in numeric portion
        assert_eq!(
            Tag::from_str("RNGA001"),
            Err(FromStrError::DigitNotFound('A'))
        );
        assert_eq!(
            Tag::from_str("RNG0B00"),
            Err(FromStrError::DigitNotFound('B'))
        );
        assert_eq!(
            Tag::from_str("RNG00C0"),
            Err(FromStrError::DigitNotFound('C'))
        );
        assert_eq!(
            Tag::from_str("RNG000D"),
            Err(FromStrError::DigitNotFound('D'))
        );
    }
}
