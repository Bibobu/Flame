#![allow(dead_code)]

#[derive(Default, Debug)]
pub struct Lmpbox {
    pub xlo: f64,
    pub xhi: f64,
    pub ylo: f64,
    pub yhi: f64,
    pub zlo: f64,
    pub zhi: f64,
    pub xy: f64,
    pub xz: f64,
    pub yz: f64,
}
