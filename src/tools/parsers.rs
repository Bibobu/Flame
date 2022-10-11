use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;

// Parser related functions

// Just parses the line for atoms using whitespaces.
// Format spec should be dealt with in lmpat.rs
pub fn atom_parser(s: &str) -> IResult<&str, Vec<&str>> {
    nom::multi::many0(nom::bytes::complete::tag(" "))(s)
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
