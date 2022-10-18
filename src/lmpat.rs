#![allow(dead_code)]

use crate::tools::parsers;

#[derive(Debug, Default)]
pub struct Atom {
    pub format: String,
    pub atid: u32,
    pub attype: u32,
    pub molid: u32,
    pub q: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub fx: f64,
    pub fy: f64,
    pub fz: f64,
    pub xflag: u32,
    pub yflag: u32,
    pub zflag: u32,
}

impl Atom {
    fn new() -> Atom {
        Atom{ ..Default::default() }
    }

    pub fn read_atom(input: &str, format:&str) -> Atom {
        let parsed = parsers::atom_parser(input).expect("Could not parse atom.").1;
        let mut atom = Atom::new();
        match format {
            "atomic" => {
                atom.atid = parsed[0].parse::<u32>().expect("Could not parse atid.");
                atom.attype = parsed[1].parse::<u32>().expect("Could not parse attype.");
                atom.x = parsed[2].parse::<f64>().expect("Could not parse x.");
                atom.y = parsed[3].parse::<f64>().expect("Could not parse y.");
                atom.z = parsed[4].parse::<f64>().expect("Could not parse z.");
                atom.format = format.to_string();
            }
            "full" => {
                atom.atid = parsed[0].parse::<u32>().expect("Could not parse atid.");
                atom.molid = parsed[1].parse::<u32>().expect("Could not parse molid.");
                atom.attype = parsed[2].parse::<u32>().expect("Could not parse attype.");
                atom.q = parsed[3].parse::<f64>().expect("Could not parse q.");
                atom.x = parsed[4].parse::<f64>().expect("Could not parse x.");
                atom.y = parsed[5].parse::<f64>().expect("Could not parse y.");
                atom.z = parsed[6].parse::<f64>().expect("Could not parse z.");
                atom.format = format.to_string();
            }
            _ => panic!("Could not understand format for atom parsing!")
        }
    atom
    }
}
