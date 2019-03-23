#[macro_use]
extern crate clap;

use std::fs;
use std::hash::Hasher;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;

use clap::{arg_enum, value_t, App, Arg};
use failure::Error;
use twox_hash::{XxHash, XxHash32};

arg_enum! {
    #[derive(PartialEq, Debug, Copy, Clone)]
    pub enum Algorithm {
        H64,
        H32,
    }
}

fn main() {
    let mut app = App::new("twoxhash")
        .version("1.0")
        .author("Jonathan Fok kan <jfokkan@gmail.com>")
        .about("Print xxhash checksums")
        .arg(
            Arg::with_name("FILE")
                .help("Sets the input file to use")
                .multiple(true),
        )
        .arg(
            Arg::with_name("ALG")
                .short("a")
                .long("algorithm")
                .help("Algorithm to be used")
                .possible_values(&Algorithm::variants())
                .default_value("H64")
                .takes_value(true),
        );
    let matches = app.clone().get_matches();

    if !matches.is_present("FILE") {
        app.print_long_help().expect("print help");
        process::exit(1)
    }
    let files: Vec<&str> = matches.values_of("FILE").expect("NO FILE").collect();
    let algorithm = value_t!(matches.value_of("ALG"), Algorithm).expect("NO ALG");

    match hash_files(files, algorithm) {
        Err(_) => process::exit(1),
        _ => process::exit(0),
    }
}

pub fn hash_files(files: Vec<&str>, alg: Algorithm) -> Result<(), Error> {
    for file in files {
        let checksum = hash_file_with_alg(file, alg)?;
        println!("{}  {}", checksum, file);
    }
    Ok(())
}

fn hash_file_with_alg<P: AsRef<Path>>(file_path: P, alg: Algorithm) -> Result<String, Error> {
    match alg {
        Algorithm::H64 => hash_file(file_path, &mut XxHash::default()),
        Algorithm::H32 => hash_file(file_path, &mut XxHash32::default()),
    }
}

pub fn hash_file<P: AsRef<Path>, H: Hasher>(file_path: P, hasher: &mut H) -> Result<String, Error> {
    let file = fs::File::open(file_path)?;
    let mut reader = BufReader::new(file);
    loop {
        let length = {
            let buf = reader.fill_buf()?;
            // do stuff with buffer here
            hasher.write(buf);
            buf.len()
        };
        if length == 0 {
            break;
        }
        reader.consume(length);
    }
    let hashed_value = hasher.finish();
    Ok(format!("{:x}", hashed_value))
}
