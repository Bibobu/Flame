#![allow(dead_code)]

use crate::tools::parsers;

#[derive(Debug, Default)]
pub struct Atom {
    format: String,
    id: u32,
    attype: u32,
    mol: u32,
    q: f64,
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    fx: f64,
    fy: f64,
    fz: f64,
    xflag: u32,
    yflag: u32,
    zflag: u32,
}

impl Atom {
    fn read_atom(input: &str, format:&str) -> Atom {
        if format == "atomic" {
            let parsed = parsers::atom_parser(input).expect("Could not parse atom.").1;
            let id = parsed[0].parse::<u32>().expect("Could not parse id.");
            let attype = parsed[1].parse::<u32>().expect("Could not parse attype.");
            let x = parsed[2].parse::<f64>().expect("Could not parse x.");
            let y = parsed[3].parse::<f64>().expect("Could not parse y.");
            let z = parsed[4].parse::<f64>().expect("Could not parse z.");
            Atom {
                format: format.to_string(),
                id: id, attype: attype,
                x: x, y: y, z: z, ..Default::default()
            }
        } else {
            panic!("Unrecognized format.")
        }
    }
}
