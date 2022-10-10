#![allow(dead_code)]

#[derive(Debug)]
struct Atom {
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
    zflag: u32
}
