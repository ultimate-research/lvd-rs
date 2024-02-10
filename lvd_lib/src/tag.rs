use std::{str::FromStr, string::ToString};

use binrw::binrw;
use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Version;

/// A unique integer identifier for an LVD object.
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
/// # Restrictions
///
/// When converting from a `Tag` to a string, the underlying binary data should follow these restrictions:
///
/// - Letter values must range from 0 to 26, inclusively.
/// - Number must range from 0 to 9999, inclusively.
///
/// Likewise, when converting from a string to a `Tag`'s native integer representation,
/// the string should follow these restrictions:
///
/// - Must have a length of seven characters.
/// - Must begin with three capital letters, underscores, or any combination of the two.
/// - Must end with four digits.
///
/// If one or more of these restrictions are not met on either end, the corresponding
/// conversion methods may return erroneous results.
#[binrw]
#[br(import(version: u8), pre_assert(version == 1))]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Tag(u32);

impl Tag {
    const STRING_LEN: usize = 7;
    const LETTER_COUNT: usize = 3;
    const LETTER_CHAR_MIN: u8 = b'A';
    const LETTER_MAX: u8 = 26;
    const DIGIT_COUNT: usize = 4;
    const DIGIT_CHAR_MIN: u8 = b'0';
    const DIGIT_MAX: u8 = 10;
}

impl FromStr for Tag {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();

        if bytes.len() != Self::STRING_LEN {
            return Err(FromStrError::BadStringLength(bytes.len()));
        }

        let (letters_str, digits_str) = bytes.split_at(Self::LETTER_COUNT);
        let mut letters = [0; Self::LETTER_COUNT];
        let mut digits = [0; Self::DIGIT_COUNT];

        for (index, letter) in letters_str.iter().enumerate() {
            let letter = *letter;

            if u8::wrapping_sub(letter, Self::LETTER_CHAR_MIN) < Self::LETTER_MAX {
                letters[index] = letter - (Self::LETTER_CHAR_MIN - 1);
            } else if letter == b'_' {
                letters[index] = 0;
            } else {
                return Err(FromStrError::LetterNotFound(letter as char));
            }
        }

        for (index, digit) in digits_str.iter().enumerate() {
            let digit = *digit;

            if u8::wrapping_sub(digit, Self::DIGIT_CHAR_MIN) < Self::DIGIT_MAX {
                digits[index] = digit - Self::DIGIT_CHAR_MIN;
            } else {
                return Err(FromStrError::DigitNotFound(digit as char));
            }
        }

        let word =
            (letters[0] as u32) << 24 | (letters[1] as u32) << 19 | (letters[2] as u32) << 14;
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

impl ToString for Tag {
    fn to_string(&self) -> String {
        let letter1 = match self.0 & 0b00011111_00000000_00000000_00000000 {
            // SAFETY: The given bit manipulations guarantee a valid char.
            c if c != 0 => unsafe {
                char::from_u32_unchecked((c >> 24) + Self::LETTER_CHAR_MIN as u32 - 1)
            },
            _ => '_',
        };
        let letter2 = match self.0 & 0b00000000_11111000_00000000_00000000 {
            // SAFETY: The given bit manipulations guarantee a valid char.
            c if c != 0 => unsafe {
                char::from_u32_unchecked((c >> 19) + Self::LETTER_CHAR_MIN as u32 - 1)
            },
            _ => '_',
        };
        let letter3 = match self.0 & 0b00000000_00000111_11000000_00000000 {
            // SAFETY: The given bit manipulations guarantee a valid char.
            c if c != 0 => unsafe {
                char::from_u32_unchecked((c >> 14) + Self::LETTER_CHAR_MIN as u32 - 1)
            },
            _ => '_',
        };
        let number = self.0 & 0b00000000_00000000_00111111_11111111;

        format!("{}{}{}{:0>4}", letter1, letter2, letter3, number)
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

/// The error type used when converting a string into the integer representation of a `Tag`.
#[derive(Debug, PartialEq, Error)]
pub enum FromStrError {
    /// The string's length did not equate to the expected length.
    #[error("expected string length {}, found length {0}", Tag::STRING_LEN)]
    BadStringLength(usize),

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
    }

    #[test]
    fn use_bad_string_length() {
        assert_eq!(
            Tag::from_str(""),
            Err(FromStrError::BadStringLength("".len()))
        );
        assert_eq!(
            Tag::from_str("I"),
            Err(FromStrError::BadStringLength("I".len()))
        );
        assert_eq!(
            Tag::from_str("IP"),
            Err(FromStrError::BadStringLength("IP".len()))
        );
        assert_eq!(
            Tag::from_str("IPP"),
            Err(FromStrError::BadStringLength("IPP".len()))
        );
        assert_eq!(
            Tag::from_str("IPP0"),
            Err(FromStrError::BadStringLength("IPP0".len()))
        );
        assert_eq!(
            Tag::from_str("IPP00"),
            Err(FromStrError::BadStringLength("IPP00".len()))
        );
        assert_eq!(
            Tag::from_str("IPP000"),
            Err(FromStrError::BadStringLength("IPP000".len()))
        );
        assert_eq!(
            Tag::from_str("IPP00001"),
            Err(FromStrError::BadStringLength("IPP00001".len()))
        );
    }

    #[test]
    fn use_unsupported_character() {
        // Test lowercase letters
        assert_eq!(
            Tag::from_str("iPP0001"),
            Err(FromStrError::LetterNotFound('i'))
        );
        assert_eq!(
            Tag::from_str("IpP0001"),
            Err(FromStrError::LetterNotFound('p'))
        );
        assert_eq!(
            Tag::from_str("IPp0001"),
            Err(FromStrError::LetterNotFound('p'))
        );
        // Test unsupported characters
        assert_eq!(
            Tag::from_str("@AA0000"),
            Err(FromStrError::LetterNotFound('@'))
        );
        assert_eq!(
            Tag::from_str("A[A0000"),
            Err(FromStrError::LetterNotFound('['))
        );
        // Test digits in alphabetical portion
        assert_eq!(
            Tag::from_str("0000000"),
            Err(FromStrError::LetterNotFound('0'))
        );
        assert_eq!(
            Tag::from_str("SE00000"),
            Err(FromStrError::LetterNotFound('0'))
        );
    }

    #[test]
    fn use_unsupported_digit() {
        // Test unsupported characters
        assert_eq!(
            Tag::from_str("IPP/001"),
            Err(FromStrError::DigitNotFound('/'))
        );
        assert_eq!(
            Tag::from_str("IPP0:01"),
            Err(FromStrError::DigitNotFound(':'))
        );
        // Test letters in numeric portion
        assert_eq!(
            Tag::from_str("IPPA001"),
            Err(FromStrError::DigitNotFound('A'))
        );
        assert_eq!(
            Tag::from_str("IPP000B"),
            Err(FromStrError::DigitNotFound('B'))
        );
        assert_eq!(
            Tag::from_str("CCCCCCC"),
            Err(FromStrError::DigitNotFound('C'))
        );
    }
}
