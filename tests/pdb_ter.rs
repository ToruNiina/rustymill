extern crate rustymill as mill;
extern crate nalgebra  as na;

#[test]
fn read_pdb_ter_line() {
    {
        let ter = mill::pdb::Ter::from(
            "TER   12345      ALA A   2                                                      "
            ).unwrap();
        assert_eq!(ter.record_name(),        "TER");
        assert_eq!(ter.atom_number(),        12345);
        assert_eq!(ter.residue_name(),       "ALA");
        assert_eq!(ter.chain_id(),           'A');
        assert_eq!(ter.residue_number(),     2);
        assert_eq!(ter.insertion_code(),     ' ');
    }

    {
        let ter = mill::pdb::Ter::from("TER   12345      ALA A   2").unwrap();
        assert_eq!(ter.record_name(),        "TER");
        assert_eq!(ter.atom_number(),        12345);
        assert_eq!(ter.residue_name(),       "ALA");
        assert_eq!(ter.chain_id(),           'A');
        assert_eq!(ter.residue_number(),     2);
        assert_eq!(ter.insertion_code(),     ' ');
    }

    {
        let ter = mill::pdb::Ter::from("TER").unwrap();
        assert_eq!(ter.record_name(),        "TER");
        assert_eq!(ter.atom_number(),        1);
        assert_eq!(ter.residue_name(),       "");
        assert_eq!(ter.chain_id(),           'A');
        assert_eq!(ter.residue_number(),     1);
        assert_eq!(ter.insertion_code(),     ' ');
    }
}

#[test]
fn make_pdb_ter_line() {
    {
        let atom = mill::pdb::AtomBuilder::new()
               .atom_number(42)
               .residue_number(69)
               .residue_name("GLY")
               .chain_id('B')
               .finalize();
        let ter = mill::pdb::Ter::new(&atom);
        assert_eq!(ter.record_name(),        "TER");
        assert_eq!(ter.atom_number(),        42);
        assert_eq!(ter.residue_name(),       "GLY");
        assert_eq!(ter.chain_id(),           'B');
        assert_eq!(ter.residue_number(),     69);
        assert_eq!(ter.insertion_code(),     ' ');
    }
    {
        let hetatm = mill::pdb::HetatmBuilder::new()
               .atom_number(42)
               .residue_number(69)
               .residue_name("HOH")
               .chain_id('B')
               .finalize();
        let ter = mill::pdb::Ter::new(&hetatm);
        assert_eq!(ter.record_name(),        "TER");
        assert_eq!(ter.atom_number(),        42);
        assert_eq!(ter.residue_name(),       "HOH");
        assert_eq!(ter.chain_id(),           'B');
        assert_eq!(ter.residue_number(),     69);
        assert_eq!(ter.insertion_code(),     ' ');
    }
}

#[test]
fn write_pdb_ter_line() {
    {
        let ter = mill::pdb::Ter::from(
            "TER   12345      ALA A   2                                                      "
            ).unwrap();
        let line = format!("{}", ter);
        assert_eq!(line, "TER   12345      ALA A   2                                                      ");
    }

    {
        let atom = mill::pdb::AtomBuilder::new()
               .atom_number(42)
               .residue_number(69)
               .residue_name("GLY")
               .chain_id('B')
               .finalize();
        let ter = mill::pdb::Ter::new(&atom);
        let line = format!("{}", ter);
        assert_eq!(line, "TER      42      GLY B  69                                                      ");
    }
}
