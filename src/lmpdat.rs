#![allow(dead_code)]

use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::Enumerate;
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
        let file = fs::File::open(&filename).expect("File not found.");
        // let data = fs::read_to_string(filename).expect("Unable to read file");
        let buf = BufReader::new(file);

        // Initialising values
        let mut is_header = true;
        let mut natoms = 0 as u32;
        let mut nbonds = 0 as u32;
        let mut nangles = 0 as u32;
        let mut ndiheds = 0 as u32;
        let mut nimpros = 0 as u32; 
        let mut nattypes = 0 as u32;
        let mut nbotypes = 0 as u32;
        let mut nantypes = 0 as u32;
        let mut nditypes = 0 as u32;
        let mut nimtypes = 0 as u32;
        let mut xlo = 0. as f64;
        let mut xhi = 0. as f64;
        let mut ylo = 0. as f64;
        let mut yhi = 0. as f64;
        let mut zlo = 0. as f64;
        let mut zhi = 0. as f64;
        let mut xy = 0. as f64;
        let mut xz = 0. as f64;
        let mut yz = 0. as f64;

        for (i, line) in buf.lines().enumerate() {
            if i == 0 {
                continue;
            }

            let myline = line.expect("Could'not read data line.");
            if is_header {
                if myline.contains("atoms") {
                    natoms = parsers::head_field_parser(&myline.trim()).expect("Couldn't parse atom number").1[0];
                } else if myline.contains("atom types") {
                    nattypes = parsers::head_field_parser(&myline.trim()).expect("Couldn't parse atom types number").1[0];
                } else if myline.contains("xlo") {
                    let dims = parsers::box_field_parser(&myline.trim()).expect("Couldn't parse box xlo").1;
                    (xlo, xhi) = (dims[0], dims[1]);
                } else if myline.contains("ylo") {
                    let dims = parsers::box_field_parser(&myline.trim()).expect("Couldn't parse box xlo").1;
                    (ylo, yhi) = (dims[0], dims[1]);
                } else if myline.contains("zlo") {
                    let dims = parsers::box_field_parser(&myline.trim()).expect("Couldn't parse box xlo").1;
                    (zlo, zhi) = (dims[0], dims[1]);
                } else if myline.contains("xy") {
                    let dims = parsers::box_field_parser(&myline.trim()).expect("Couldn't parse box xlo").1;
                    (xy, xz, yz) = (dims[0], dims[1], dims[2]);
                } else if myline.chars().any(|x| x.is_uppercase()) {
                    is_header = false;
                }
            } else {
                if myline.contains("Atoms") {
                    let mut format = "full";
                    let mut atoms:Vec<lmpat::Atom> = Vec::new();
                    for i in 1..natoms {
                        let atline = buf.lines().next().unwrap().unwrap();
                        let atom = lmpat::Atom::read_atom(&atline, &format);
                        atoms.push(atom);
                    }
                }
            }
        }
        // Header starts here
        // let (body, header) = parsers::header_parser(&data).expect("Data parsing failed.");

        let databox = lmpbox::Lmpbox {xlo: xlo, xhi: xhi, ylo: ylo, yhi: yhi, zlo: zlo, zhi: zhi, xy: xy, xz: xz, yz: yz};
        let header = Header {natoms: natoms, nbonds: nbonds, nangles: nangles, ndiheds: ndiheds, nimpros: nimpros, nattypes: nattypes, nbotypes: nbotypes, nantypes: nantypes, nditypes: nditypes, nimtypes: nimtypes, lmpbox: databox};
        Data {file: filename, header: header, ..Default::default()}
    }
}

