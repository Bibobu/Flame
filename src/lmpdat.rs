#![allow(dead_code)]

use std::fs;
// use std::io::BufReader;
use crate::tools::parsers;
use crate::{lmpat, lmpbox};

#[derive(Debug, Default)]
struct Header {
    natoms: u32,
    nbonds: u32,
    nangles: u32,
    ndiheds: u32,
    nimpros: u32,

    nattypes: u32,
    nbotypes: u32,
    nantypes: u32,
    nditypes: u32,
    nimtypes: u32,

    lmpbox: lmpbox::Lmpbox,
}

#[derive(Debug, Default)]
struct Body {
    atoms: Vec<lmpat::Atom>,
}

#[derive(Debug, Default)]
struct Data {
    file: String,
    header: Header,
    body: Body,
}

impl Data {
    fn read_file(datafile:&str) -> Data {
        let filename = String::from(datafile);
        // let mut file = File::open(filename).expect("File not found.");
        let data = fs::read_to_string(filename).expect("Unable to read file");

        // Header starts here
        let (body, header) = parsers::header_parser(&data).expect("Data parsing failed.");

        Data {..Default::default()}
    }
}

