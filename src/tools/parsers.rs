use std::str::FromStr;
use nom::IResult;
// use nom::branch::alt;
use nom::character::complete::{not_line_ending, digit1};
use nom::number::complete::double;
use nom::bytes::complete::{tag, take_until};
use nom::multi::many_m_n;
use nom::combinator::map_res;

// Parser related functions

// Just parses the line for atoms using whitespaces.
// Format spec should be dealt with in lmpat.rs
pub fn atom_parser(s: &str) -> IResult<&str, Vec<&str>> {
    nom::multi::many0(nom::bytes::complete::tag(" "))(s)
}

fn parse_integer(input: &str) -> IResult<&str, u32> {
    map_res(digit1, u32::from_str)(input.trim_start())
}

fn parse_float(input: &str) -> IResult<&str, f64> {
    double(input.trim_start())
}

// Parses header num + field lines
pub fn head_field_parser(s: &str) -> IResult<&str, Vec<u32>> {
    many_m_n(1, 2, parse_integer)(s)
}

pub fn box_field_parser(s: &str) -> IResult<&str, Vec<f64>> {
    many_m_n(2, 3, parse_float)(s)
}

// Parses each line of the input
pub fn line_parser(s: &str) -> IResult<&str, &str> {
    not_line_ending(s)
}

// Should split the file at the first body keyword encountered
// TODO: add more keywords when POC works
pub fn header_parser(data_content: &str) -> IResult<&str, &str> {
    // alt((
    //     tag("Masses"),
    //     tag("Atoms"),
    //     tag("Velocities"),
    // ))(data_content)
    tag("Masses")(data_content)
}
