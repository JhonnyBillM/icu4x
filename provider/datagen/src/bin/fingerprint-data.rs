// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use clap::{App, Arg};
use icu_provider_blob::BlobDataProvider;
use sha2::{Digest, Sha256};
use simple_logger::SimpleLogger;
use std::fs;
use std::io::Write;
use walkdir::WalkDir;

fn main() -> eyre::Result<()> {
    let matches = App::new("ICU4X Data fingerprinter")
        .version("0.0.1")
        .author("The ICU4X Project Developers")
        .about("Generate a fingerprint file from a data folder containing hashes of the data")
        .arg(
            Arg::with_name("VERBOSE")
                .short("v")
                .long("verbose")
                .help("Requests verbose output"),
        )
        .arg(
            Arg::with_name("DATA")
                .short("d")
                .long("data")
                .takes_value(true)
                .required(true)
                .help(
                    "Path to folder containing data generated by icu4x-datagen --format=dir, 
                       or file containing data generated by icu4x-datagen --format=blob.",
                ),
        )
        .arg(
            Arg::with_name("FORMAT")
                .long("format")
                .takes_value(true)
                .possible_value("dir")
                .possible_value("blob")
                .help("Read from an fs-format directory on the filesystem or a blob input.")
                .default_value("dir"),
        )
        .arg(
            Arg::with_name("FINGERPRINT_FILE")
                .short("o")
                .long("out")
                .takes_value(true)
                .required(true)
                .help("Path to output file to put fingerprint in"),
        )
        .get_matches();

    if matches.is_present("VERBOSE") {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Trace)
            .init()
            .unwrap()
    } else {
        SimpleLogger::new()
            .env()
            .with_level(log::LevelFilter::Info)
            .init()
            .unwrap()
    }

    let base = matches.value_of("DATA").unwrap();
    let out_path = matches.value_of("FINGERPRINT_FILE").unwrap();
    let mut out = fs::File::create(out_path)?;

    let format = matches.value_of("FORMAT").unwrap();

    log::info!("Writing hashes for {format}-type data in {base} to {out_path}");

    if format == "dir" {
        let walker = WalkDir::new(base).sort_by_file_name().into_iter();

        for entry in walker {
            let mut hasher = Sha256::new();
            let entry = entry?;
            if entry.file_type().is_dir() || entry.file_name() == "manifest.json" {
                continue;
            }
            let bytes = fs::read(entry.path())?;
            hasher.update(&bytes);
            let result = hasher.finalize();
            let path =
                pathdiff::diff_paths(entry.path(), base).expect("Paths ought to diff correctly");

            let parent = path.parent().expect("File must be in a folder");
            let parent = parent.display();
            let file = path
                .file_stem()
                .expect("We have already checked that this is a file")
                .to_str()
                .expect("Keys should be ASCII");

            log::trace!("Hash for {parent}/{file} is {result:x}");
            writeln!(out, "{parent}/{file}: {result:x}")?;
        }
    } else if format == "blob" {
        let blob = fs::read(base)?;
        let provider = BlobDataProvider::new_from_blob(blob)?;
        let mut all_keys = icu_datagen::get_all_keys();
        all_keys.sort_by_key(|k| k.get_path());
        let map = provider.get_map();
        for key in all_keys {
            let hash = key.get_hash();
            if let Some(cursor) = map.get0(&hash) {
                for (locale, data) in cursor.iter1() {
                    let mut hasher = Sha256::new();
                    hasher.update(&data);
                    let result = hasher.finalize();
                    let slash = if locale.is_empty() { "" } else { "/" };
                    let path = key.get_path();
                    let locale_str = std::str::from_utf8(locale)
                        .expect("Locales in the data provider should be valid strings");

                    log::trace!("Hash for {path}{slash}{locale_str} is {result:x}");
                    writeln!(out, "{path}{slash}{locale_str}: {result:x}")?;
                }
            }
        }
    } else {
        eyre::bail!("--format must be `blob` or `dir`",)
    }

    Ok(())
}
