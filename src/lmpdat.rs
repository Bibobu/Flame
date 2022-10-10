#![allow(dead_code)]

use std::fs::File;
use std::io::BufReader;
use lmpat::Atom;
// use bond;
// use angle;
// use dihed;
// use impro;

#[derive(Debug)]
struct Data {
    file: String,
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

    xlo: f64,
    xhi: f64,
    ylo: f64,
    yhi: f64,
    zlo: f64,
    zhi: f64,
    xy: f64,
    xz: f64,
    yz: f64,

    atoms: Vec<Atom>,
//     bonds: Vec<bond>,
//     angles: Vec<angle>,
//     diheds: Vec<dihed>,
//     impros: Vec<impro>
}

impl Data {
    fn read_file(datafile:&str) -> Data {
        let file = String::from(datafile);
        let mut file = File::open(file).expect("File not found.");
        let mut line = String::new(file);

        let header_keywords = vec!["atoms".to_string()];
        let body_keywords = vec!["Atoms".to_string()];
        // There is a first garbage commentary line in data files.
        line = file.read_line();

        // Header starts here
        let mut is_header = true;
        while is_header {
            line = file.read_line().expect("Read line failed");
            let myline = line.split(" ").collect::<Vec<&str>>();
            match myline.len() {
                2 => match line[1] {
                    "atoms" => let natoms = line[0]
                        .parse::<u32>()
                        .expect("Could not read number of atoms."),
                    "atom" => let nattypes = line[0]
                        .parse::<u32>()
                        .expect("Could not read number of atom types."),
                    _ => panic!("Error while reading file header."),
                    },
                4 => match (line[2], line[3]) {
                    ("xlo", "xhi") => {
                        let xlo = line[0].parse::<f64>().expect("Could not read xlo");
                        let xhi = line[1].parse::<f64>().expect("Could not read xhi");
                    },
                    ("ylo", "yhi") => {
                        let ylo = line[0].parse::<f64>().expect("Could not read ylo");
                        let yhi = line[1].parse::<f64>().expect("Could not read yhi");
                    },
                    ("zlo", "zhi") => {
                        let zlo = line[0].parse::<f64>().expect("Could not read zlo");
                        let zhi = line[1].parse::<f64>().expect("Could not read zhi");
                    },
                    _ => panic!("Error while reading file header."),
                }
                6 => match(line[3], line[4], line[5]) {
                    ("xy", "xz", "yz") => {
                        let xy = line[0].parse::<f64>().expect("Could not read xlo");
                        let xz = line[1].parse::<f64>().expect("Could not read xlo");
                        let yz = line[1].parse::<f64>().expect("Could not read xlo");
                    },
                    _ => panic!("Error while reading file header.")
                }
                1 => is_header = false,
                _ => panic!("Error while reading file header.")
            }
        }
    }
}

