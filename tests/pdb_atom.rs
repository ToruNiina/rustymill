extern crate rustymill as mill;
extern crate nalgebra  as na;
use mill::Particle;

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
    {
        let atom = mill::pdb::Atom::from(
            "ATOM     45  N  BARG A   3      11.281  86.699  94.383  1.00 39.29           N N").unwrap();
        let line = format!("{}", atom);
        println!("{}", atom);
        assert_eq!(line, "ATOM     45  N  BARG A   3      11.281  86.699  94.383  1.00 39.29           N N");
    }

    {
        let atom = mill::pdb::Atom::from(
            "ATOM   6025  CA  ALA A   1     106.912 133.801 -29.864  1.00 92.36           C  ").unwrap();
        let line = format!("{}", atom);
        println!("{}", atom);
        assert_eq!(line, "ATOM   6025  CA  ALA A   1     106.912 133.801 -29.864  1.00 92.36           C  ");
    }
}

#[test]
fn build_pdb_atom() {
    let atom = mill::pdb::AtomBuilder::new().finalize();
    assert_eq!(atom.record_name(),        "ATOM");
    assert_eq!(atom.atom_number(),        1);
    assert_eq!(atom.atom_name(),          "");
    assert_eq!(atom.alternate_location(), ' ');
    assert_eq!(atom.residue_name(),       "");
    assert_eq!(atom.chain_id(),           'A');
    assert_eq!(atom.residue_number(),     1);
    assert_eq!(atom.insertion_code(),     ' ');
    assert_eq!(atom.x,                    0.0);
    assert_eq!(atom.y,                    0.0);
    assert_eq!(atom.z,                    0.0);
    assert_eq!(atom.occupancy(),          0.0);
    assert_eq!(atom.temperature_factor(), 99.9);
    assert_eq!(atom.element_symbol(),     "");
    assert_eq!(atom.charge(),             "");


    let atom = mill::pdb::AtomBuilder::new()
               .pos(1.0, 2.0, 3.0)
               .atom_number(42)
               .residue_number(69)
               .record_name("ATOM")
               .atom_name("CA")
               .residue_name("GLY")
               .occupancy(3.14)
               .temperature_factor(2.71)
               .element("N")
               .charge("N")
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
    assert_eq!(atom.element_symbol(),     "N");
    assert_eq!(atom.charge(),             "N");

    let atom = mill::pdb::AtomBuilder::new()
               .pos(1.0, 2.0, 3.0)
               .atom_number(42)
               .residue_number(69)
               .atom_residue_chain("CA", "GLY", 'A')
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
    assert_eq!(atom.occupancy(),          0.00);
    assert_eq!(atom.temperature_factor(), 99.9);
    assert_eq!(atom.element_symbol(),     "");
    assert_eq!(atom.charge(),             "");
}

#[test]
fn particle_trait_pdb_atom() {
    let atom = mill::pdb::AtomBuilder::new().finalize();
    assert_eq!(atom.name().unwrap(), "");
    assert_eq!(atom.x(), 0.0);
    assert_eq!(atom.y(), 0.0);
    assert_eq!(atom.z(), 0.0);
    assert_eq!(atom.pos(),  na::Vector3::<f64>::new(0.0, 0.0, 0.0));

    let atom = mill::pdb::AtomBuilder::new()
               .pos(1.0, 2.0, 3.0)
               .atom_name("CA")
               .finalize();
    assert_eq!(atom.name().unwrap(), "CA");
    assert_eq!(atom.x(), 1.0);
    assert_eq!(atom.y(), 2.0);
    assert_eq!(atom.z(), 3.0);
    assert_eq!(atom.pos(),  na::Vector3::<f64>::new(1.0, 2.0, 3.0));
}
