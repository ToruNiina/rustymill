extern crate rustymill;

#[test]
fn read_pdb_line() {
    let atom = PDBAtom::from("ATOM     45  N  BARG A   3      11.281  86.699  94.383  1.00 39.29           N N");
    assert_eq!(atom.record_name(),        "ATOM");
    assert_eq!(atom.atom_number(),        45);
    assert_eq!(atom.atom_name(),          "N");
    assert_eq!(atom.alternate_location(), b'B');
    assert_eq!(atom.residue_name(),       "ARG");
    assert_eq!(atom.chain_id(),           b'A');
    assert_eq!(atom.residue_number(),     3);
    assert_eq!(atom.insertion_code(),     b' ');
    assert_eq!(atom.x(),                  11.281);
    assert_eq!(atom.y(),                  86.699);
    assert_eq!(atom.z(),                  94.383);
    assert_eq!(atom.occupancy(),          1.00);
    assert_eq!(atom.temperature_factor(), 39.29);
    assert_eq!(atom.element_symbol(),     "N");
    assert_eq!(atom.charge(),             "N");
}
