use flame::tools;


#[test]
fn test_line_parser() {
    let test:String = "LAMMPS comment

3 atoms
1 atom types".to_string();
    assert_eq!(
        Ok(("\n\n3 atoms\n1 atom types", "LAMMPS comment")),
        tools::parsers::line_parser(&test)
        )
}

#[test]
fn test_head_field_parser() {
    let test:&str = " 34 atoms # Un commentaire";
    assert_eq!(Ok((" atoms # Un commentaire", vec![34 as u32])),
    tools::parsers::head_field_parser(&test));
}

#[test]
fn test_box_field_parser() {
    let test:&str = "-0.003 1.000E+7 xlo xhi # Un commentaire";
    assert_eq!(Ok((" xlo xhi # Un commentaire", vec![-0.003 as f64, 1E+7 as f64])),
    tools::parsers::box_field_parser(&test));
}

