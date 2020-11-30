use std::path::PathBuf;
use std::fs;

use lvd::LvdFile;
use clap::Clap;

#[derive(Clap)]
struct Args {
    in_file: PathBuf,
    out_file: PathBuf,
}

fn main() {
    let args = Args::parse();

    match LvdFile::open(&args.in_file) {
        Ok(lvd_file) => {
            fs::write(&args.out_file, serde_yaml::to_string(&lvd_file).unwrap()).unwrap();
        }
        Err(binrw::Error::BadMagic { .. }) => {
            // Magic doesn't match, is a yaml file
            let contents = fs::read_to_string(&args.in_file).unwrap();
            let lvd_file: LvdFile = serde_yaml::from_str(&contents).unwrap();

            lvd_file.save(&args.out_file).unwrap();
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
}
