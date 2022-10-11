use flame::tools;

#[test]
fn test_header_parser() {
    let test:String = "LAMMPS data file via write_data, version 2 Jun 2022, timestep = 0

3 atoms
1 atom types

0 10 xlo xhi
0 10 ylo yhi
0 10 zlo zhi

Masses

1 1

Atoms # atomic

1 1 8.608283632718159 9.42301509409352 2.6146864297868158 0 0 0
2 1 5.034825427008246 0.31095172758724154 6.165685558768774 0 0 0
3 1 6.677186226787598 3.468913619159215 2.0311972089257075 0 0 0

Velocities
1 0 0 0
2 0 0 0
3 0 0 0
".to_string();
    let result1:String = "

Masses

1 1

Atoms # atomic

1 1 8.608283632718159 9.42301509409352 2.6146864297868158 0 0 0
2 1 5.034825427008246 0.31095172758724154 6.165685558768774 0 0 0
3 1 6.677186226787598 3.468913619159215 2.0311972089257075 0 0 0

Velocities
1 0 0 0
2 0 0 0
3 0 0 0
".to_string();
    let result2:String = "LAMMPS data file via write_data, version 2 Jun 2022, timestep = 0

3 atoms
1 atom types

0 10 xlo xhi
0 10 ylo yhi
0 10 zlo zhi
".to_string();
    // let parsed = tools::parsers::header_parser(&test);
    println!("{:?}", tools::parsers::header_parser("Toto Masses Titi"));
    // println!("{}", parsed.0);
    // assert_eq!(
    //     Ok((&result1, &result2)),
    //     tools::parsers::header_parser(&test)
    //     )
}
