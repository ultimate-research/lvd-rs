use super::*;

#[test]
fn label_to_id() {
    assert_eq!(Tag::from_str("IPP0001"), Ok(Tag::from_raw(159645697)));
    assert_eq!(Tag::from_str("IPP0002"), Ok(Tag::from_raw(159645698)));
    assert_eq!(Tag::from_str("FSP0010"), Ok(Tag::from_raw(110886922)));
    assert_eq!(Tag::from_str("FSP0011"), Ok(Tag::from_raw(110886923)));
    assert_eq!(Tag::from_str("PAL0101"), Ok(Tag::from_raw(269156453)));
    assert_eq!(Tag::from_str("PAL0102"), Ok(Tag::from_raw(269156454)));
    assert_eq!(Tag::from_str("SLD1001"), Ok(Tag::from_raw(325125097)));
    assert_eq!(Tag::from_str("SLD1002"), Ok(Tag::from_raw(325125098)));
    assert_eq!(Tag::from_str("AAA0000"), Ok(Tag::from_raw(17317888)));
    assert_eq!(Tag::from_str("ZZZ9999"), Ok(Tag::from_raw(450275087)));
    assert_eq!(Tag::from_str("C_Y0001"), Ok(Tag::from_raw(50741249)));
    assert_eq!(Tag::from_str("SE_0001"), Ok(Tag::from_raw(321388545)));
    assert_eq!(Tag::from_str("___0000"), Ok(Tag::from_raw(0)));
    assert_eq!(Tag::from_str("___0001"), Ok(Tag::from_raw(1)));
}

#[test]
fn id_to_label() {
    assert_eq!(Tag::from_raw(159645697).to_string(), "IPP0001");
    assert_eq!(Tag::from_raw(159645698).to_string(), "IPP0002");
    assert_eq!(Tag::from_raw(110886922).to_string(), "FSP0010");
    assert_eq!(Tag::from_raw(110886923).to_string(), "FSP0011");
    assert_eq!(Tag::from_raw(269156453).to_string(), "PAL0101");
    assert_eq!(Tag::from_raw(269156454).to_string(), "PAL0102");
    assert_eq!(Tag::from_raw(325125097).to_string(), "SLD1001");
    assert_eq!(Tag::from_raw(325125098).to_string(), "SLD1002");
    assert_eq!(Tag::from_raw(17317888).to_string(), "AAA0000");
    assert_eq!(Tag::from_raw(450275087).to_string(), "ZZZ9999");
    assert_eq!(Tag::from_raw(50741249).to_string(), "C_Y0001");
    assert_eq!(Tag::from_raw(321388545).to_string(), "SE_0001");
    assert_eq!(Tag::from_raw(0).to_string(), "___0000");
    assert_eq!(Tag::from_raw(1).to_string(), "___0001");
}

#[test]
fn use_invalid_string_length() {
    let s = "";
    assert_eq!(
        Tag::from_str(s),
        Err(ParseTagError::InvalidStringLength(s.len()))
    );

    let s = "I";
    assert_eq!(
        Tag::from_str(s),
        Err(ParseTagError::InvalidStringLength(s.len()))
    );

    let s = "IP";
    assert_eq!(
        Tag::from_str(s),
        Err(ParseTagError::InvalidStringLength(s.len()))
    );

    let s = "IPP";
    assert_eq!(
        Tag::from_str(s),
        Err(ParseTagError::InvalidStringLength(s.len()))
    );

    let s = "IPP0";
    assert_eq!(
        Tag::from_str(s),
        Err(ParseTagError::InvalidStringLength(s.len()))
    );

    let s = "IPP00";
    assert_eq!(
        Tag::from_str(s),
        Err(ParseTagError::InvalidStringLength(s.len()))
    );

    let s = "IPP000";
    assert_eq!(
        Tag::from_str(s),
        Err(ParseTagError::InvalidStringLength(s.len()))
    );

    let s = "IPP00001";
    assert_eq!(
        Tag::from_str(s),
        Err(ParseTagError::InvalidStringLength(s.len()))
    );
}

#[test]
fn use_unsupported_character_letter() {
    // Test lowercase letters
    assert_eq!(
        Tag::from_str("bLK0001"),
        Err(ParseTagError::LetterNotFound('b'))
    );
    assert_eq!(
        Tag::from_str("BlK0001"),
        Err(ParseTagError::LetterNotFound('l'))
    );
    assert_eq!(
        Tag::from_str("BLk0001"),
        Err(ParseTagError::LetterNotFound('k'))
    );
    // Test unsupported characters
    assert_eq!(
        Tag::from_str("@LK0000"),
        Err(ParseTagError::LetterNotFound('@'))
    );
    assert_eq!(
        Tag::from_str("B[L0000"),
        Err(ParseTagError::LetterNotFound('['))
    );
    // Test digits in alphabetical portion
    assert_eq!(
        Tag::from_str("0LK0000"),
        Err(ParseTagError::LetterNotFound('0'))
    );
    assert_eq!(
        Tag::from_str("B1K0000"),
        Err(ParseTagError::LetterNotFound('1'))
    );
    assert_eq!(
        Tag::from_str("BL20000"),
        Err(ParseTagError::LetterNotFound('2'))
    );
}

#[test]
fn use_unsupported_character_digit() {
    // Test unsupported characters
    assert_eq!(
        Tag::from_str("RNG/001"),
        Err(ParseTagError::DigitNotFound('/'))
    );
    assert_eq!(
        Tag::from_str("RNG0:01"),
        Err(ParseTagError::DigitNotFound(':'))
    );
    // Test letters in numeric portion
    assert_eq!(
        Tag::from_str("RNGA001"),
        Err(ParseTagError::DigitNotFound('A'))
    );
    assert_eq!(
        Tag::from_str("RNG0B00"),
        Err(ParseTagError::DigitNotFound('B'))
    );
    assert_eq!(
        Tag::from_str("RNG00C0"),
        Err(ParseTagError::DigitNotFound('C'))
    );
    assert_eq!(
        Tag::from_str("RNG000D"),
        Err(ParseTagError::DigitNotFound('D'))
    );
}

#[test]
fn wrap_number() {
    assert_eq!(Tag::from_raw(9999).to_string(), "___9999");
    assert_eq!(Tag::from_raw(10000).to_string(), "___0000");
}
