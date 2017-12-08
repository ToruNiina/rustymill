extern crate rustymill as mill;

#[test]
fn read_pdb_line() {
    let atom = mill::pdb::Atom::from(
        "ATOM     45  N  BARG A   3      11.281  86.699  94.383  1.00 39.29           N N").unwrap();
    assert_eq!(atom.record_name(),        "ATOM");
    assert_eq!(atom.atom_number(),        45);
    assert_eq!(atom.atom_name(),          "N");
    assert_eq!(atom.alternate_location(), 'B');
    assert_eq!(atom.residue_name(),       "ARG");
    assert_eq!(atom.chain_id(),           'A');
    assert_eq!(atom.residue_number(),     3);
    assert_eq!(atom.insertion_code(),     ' ');
    assert_eq!(atom.x,                    11.281);
    assert_eq!(atom.y,                    86.699);
    assert_eq!(atom.z,                    94.383);
    assert_eq!(atom.occupancy(),          1.00);
    assert_eq!(atom.temperature_factor(), 39.29);
    assert_eq!(atom.element_symbol(),     "N");
    assert_eq!(atom.charge(),             "N");
}
#[test]
fn fail_to_read_pdb_line() {
    {
        let atom = mill::pdb::Atom::from(
            "ATOM     45  N  BARG A   3      ABCDEF  86.699  94.383  1.00 39.29           N N");
        assert!(atom.is_err());
    }

    {
        let atom = mill::pdb::Atom::from(
            "ATOM     45  N  BARG A   C      11.281  86.699  94.383  1.00 39.29           N N");
        assert!(atom.is_err());
    }
    {
        let atom = mill::pdb::Atom::from(
            "ATOM    ABC  N  BARG A   3      11.281  86.699  94.383  1.00 39.29           N N");
        assert!(atom.is_err());
    }
    {
        let atom = mill::pdb::Atom::from(
            "ATOM     45  N  BARG A   3      11.281  86.699  94.383  1.00 39.29");
        assert!(atom.is_err());
    }
}

#[test]
fn write_pdb_line() {
    let atom = mill::pdb::Atom::from(
        "ATOM     45  N  BARG A   3      11.281  86.699  94.383  1.00 39.29           N N").unwrap();
    let line = format!("{}", atom);
    println!("{}", atom);
    assert_eq!(line, "ATOM     45  N  BARG A   3      11.281  86.699  94.383  1.00 39.29           N N");
}

#[test]
fn build_pdb_atom() {
    let atom = mill::pdb::AtomBuilder::new()
               .pos(1.0, 2.0, 3.0)
               .atom_number(42)
               .residue_number(69)
               .record_name("ATOM")
               .atom_name("CA")
               .residue_name("GLY")
               .occupancy(3.14)
               .temperature_factor(2.71)
               .finalize();
    assert_eq!(atom.record_name(),        "ATOM");
    assert_eq!(atom.atom_number(),        42);
    assert_eq!(atom.atom_name(),          "CA");
    assert_eq!(atom.alternate_location(), ' ');
    assert_eq!(atom.residue_name(),       "GLY");
    assert_eq!(atom.chain_id(),           'A');
    assert_eq!(atom.residue_number(),     69);
    assert_eq!(atom.insertion_code(),     ' ');
    assert_eq!(atom.x,                    1.0);
    assert_eq!(atom.y,                    2.0);
    assert_eq!(atom.z,                    3.0);
    assert_eq!(atom.occupancy(),          3.14);
    assert_eq!(atom.temperature_factor(), 2.71);
    assert_eq!(atom.element_symbol(),     "");
    assert_eq!(atom.charge(),             "");
}
