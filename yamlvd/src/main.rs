use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use lvd_lib::lvd::LvdFile;

mod cli;

use cli::{Args, Endian};

fn read_data_write_yaml<P: AsRef<Path> + ToString>(
    input_path: P,
    output_path: Option<String>,
    endian: Endian,
) {
    let result = match endian {
        Endian::Big => LvdFile::read_be_file(&input_path),
        Endian::Little => LvdFile::read_le_file(&input_path),
    };

    match result {
        Ok(lvd) => {
            let output_path = output_path
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(input_path.to_string() + ".yaml"));
            let yaml = serde_yaml::to_string(&lvd).unwrap();

            fs::write(output_path, yaml).expect("failed to write YAML file");
        }
        Err(error) => eprintln!("{error:?}"),
    }
}

fn read_yaml_write_data<P: AsRef<Path>>(
    input_path: P,
    output_path: Option<String>,
    endian: Endian,
) {
    let yaml = fs::read_to_string(&input_path).unwrap();

    match serde_yaml::from_str::<LvdFile>(&yaml) {
        Ok(lvd) => {
            let output_path = output_path
                .map(PathBuf::from)
                .unwrap_or_else(|| input_path.as_ref().with_extension("lvd"));
            let result = match endian {
                Endian::Big => lvd.write_be_file(output_path),
                Endian::Little => lvd.write_le_file(output_path),
            };

            result.expect("failed to write LVD file");
        }
        Err(error) => eprintln!("{error:?}"),
    }
}

fn main() {
    let args = Args::parse();

    match Path::new(&args.input)
        .extension()
        .expect("input file extension should exist")
        .to_str()
        .unwrap()
    {
        "yaml" | "yml" => read_yaml_write_data(args.input, args.output, args.endian),
        _ => read_data_write_yaml(args.input, args.output, args.endian),
    }
}
