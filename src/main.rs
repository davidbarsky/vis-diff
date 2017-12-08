extern crate difference;
extern crate failure;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use difference::Changeset;
use failure::Error;
use structopt::StructOpt;

use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "vis-diff")]
struct Opts {
    /// The first file
    #[structopt]
    path_one: String,
    /// The second file
    #[structopt]
    path_two: String,
}

fn main() {
    if let Err(e) = run() {
        panic!("{}", e);
    }
}

fn run() -> Result<(), Error> {
    let args = Opts::from_args();
    let path_one = Path::new(&args.path_one);
    let path_two = Path::new(&args.path_two);

    let contents_one = read(path_one)?;
    let contents_two = read(path_two)?;

    let changes = Changeset::new(&contents_one, &contents_two, "\n");
    println!("{}", changes);

    Ok(())
}

fn read(path: &Path) -> Result<String, Error> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}
