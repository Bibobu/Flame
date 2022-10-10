use nom;

// Parser related functions

fn not_whitespace(i: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::is_not(" \t")(i)
}

fn atom_parser(s: &str) -> IResult<&str, Vec<&str>> {
    many0(tag(" "))(s)
}
