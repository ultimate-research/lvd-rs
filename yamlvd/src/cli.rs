use clap::{Parser, ValueEnum};

/// Convert LVD files to and from YAML.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The input LVD or YAML file path.
    pub input: String,

    /// The output LVD or YAML file path.
    pub output: Option<String>,

    /// The endianness of the LVD file.
    #[arg(short, long, default_value_t, value_enum)]
    pub endian: Endian,
}

/// The endianness of the LVD file.
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum Endian {
    /// The most significant byte is stored first.
    #[default]
    Big,

    /// The least significant byte is stored first.
    Little,
}
