use std::path::PathBuf;
use std::fs;

use lvd::LvdFile;
use clap::Parser;

#[derive(Parser)]
struct Args {
    in_file: PathBuf,
    out_file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let out_file = args.out_file.clone().unwrap_or_else(|| {
        let mut out_file = args.in_file.clone();
        match out_file.extension().map(|x| x.to_str()).flatten() {
            Some("lvd") => out_file.set_extension("yaml"),
            Some("yaml") | Some("yml") => out_file.set_extension("lvd"),
            _ => true
        };
        out_file
    });

    match LvdFile::open(&args.in_file) {
        Ok(lvd_file) => {
            fs::write(&out_file, serde_yaml::to_string(&lvd_file).unwrap()).unwrap();
        }
        Err(binrw::Error::BadMagic { pos: 0, .. }) => {
            // Magic doesn't match, is a yaml file
            let contents = fs::read_to_string(&args.in_file).unwrap();
            let lvd_file: LvdFile = serde_yaml::from_str(&contents).unwrap();

            lvd_file.save(&out_file).unwrap();
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
}
