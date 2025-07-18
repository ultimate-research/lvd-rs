use binrw::{BinReaderExt, BinWriterExt, io::Cursor};

use super::*;

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
