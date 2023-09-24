use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use lvd_lib::LvdFile;

/// Convert LVD files to and from YAML
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The input LVD or YAML file path
    pub input: String,

    /// The output LVD or YAML file path
    pub output: Option<String>,
}

fn read_data_write_yaml<P: AsRef<Path> + ToString>(input_path: P, output_path: Option<String>) {
    let output_path = output_path
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(&(input_path.to_string() + ".yaml")));

    match LvdFile::from_file(input_path) {
        Ok(lvd) => {
            let yaml = serde_yaml::to_string(&lvd).unwrap();

            fs::write(output_path, yaml).expect("failed to write YAML file");
        }
        Err(error) => eprintln!("{error:?}"),
    }
}

fn read_yaml_write_data<P: AsRef<Path>>(input_path: P, output_path: Option<String>) {
    let yaml = fs::read_to_string(&input_path).unwrap();

    match serde_yaml::from_str::<LvdFile>(&yaml) {
        Ok(lvd) => {
            let output_path = output_path
                .map(PathBuf::from)
                .unwrap_or_else(|| input_path.as_ref().with_extension("lvd"));

            lvd.write_to_file(output_path)
                .expect("failed to write LVD file");
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
        "yaml" | "yml" => read_yaml_write_data(args.input, args.output),
        _ => read_data_write_yaml(args.input, args.output),
    }
}
