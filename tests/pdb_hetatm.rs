extern crate rustymill as mill;
extern crate nalgebra  as na;
use mill::Particle;
use mill::pdb::AtomData;

#[test]
fn read_pdb_hetatm() {
    {
        let het =
            "HETATM12345  O   HOH B 101      72.565  60.018  -6.320  1.00 21.96           O  "
            .parse::<mill::pdb::Hetatm>().unwrap();
        assert_eq!(het.record_name(),        "HETATM");
        assert_eq!(het.atom_number(),        12345);
        assert_eq!(het.atom_name(),          "O");
        assert_eq!(het.alternate_location(), ' ');
        assert_eq!(het.residue_name(),       "HOH");
        assert_eq!(het.chain_id(),           'B');
        assert_eq!(het.residue_number(),     101);
        assert_eq!(het.insertion_code(),     ' ');
        assert_eq!(het.x,                    72.565);
        assert_eq!(het.y,                    60.018);
        assert_eq!(het.z,                    -6.320);
        assert_eq!(het.occupancy(),          1.00);
        assert_eq!(het.temperature_factor(), 21.96);
        assert_eq!(het.element_symbol(),     "O");
        assert_eq!(het.charge(),             "");
    }

    {
        let het =
            "HETATM12345  O   HOH B 101      72.565  60.018  -6.320"
            .parse::<mill::pdb::Hetatm>().unwrap();
        assert_eq!(het.record_name(),         "HETATM");
        assert_eq!(het.atom_number(),         12345);
        assert_eq!(het.atom_name(),           "O");
        assert_eq!(het.alternate_location(),  ' ');
        assert_eq!(het.residue_name(),        "HOH");
        assert_eq!(het.chain_id(),            'B');
        assert_eq!(het.residue_number(),      101);
        assert_eq!(het.insertion_code(),      ' ');
        assert_eq!(het.x,                     72.565);
        assert_eq!(het.y,                     60.018);
        assert_eq!(het.z,                     -6.320);
        assert_eq!(het.occupancy(),            0.00);
        assert_eq!(het.temperature_factor(), 999.99);
        assert_eq!(het.element_symbol(),     "O");
        assert_eq!(het.charge(),             "");
    }

}

#[test]
fn write_pdb_hetatm() {
    {
        let het =
            "HETATM12345  O   HOH B 101      72.565  60.018  -6.320  1.00 21.96           O  "
            .parse::<mill::pdb::Hetatm>().unwrap();
        let line = format!("{}", het);
        assert_eq!(line, "HETATM12345  O   HOH B 101      72.565  60.018  -6.320  1.00 21.96           O  ");
    }
}

#[test]
fn build_pdb_hetatm() {
    {
        let atom = mill::pdb::HetatmBuilder::new().finalize();
        assert_eq!(atom.record_name(),        "HETATM");
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
    }


    {
        let atom = mill::pdb::HetatmBuilder::new()
                   .pos(1.0, 2.0, 3.0)
                   .atom_number(42)
                   .residue_number(69)
                   .atom_residue_chain("O", "HOH", 'A')
                   .occupancy(3.14)
                   .temperature_factor(2.71)
                   .element("O")
                   .finalize();
        assert_eq!(atom.record_name(),        "HETATM");
        assert_eq!(atom.atom_number(),        42);
        assert_eq!(atom.atom_name(),          "O");
        assert_eq!(atom.alternate_location(), ' ');
        assert_eq!(atom.residue_name(),       "HOH");
        assert_eq!(atom.chain_id(),           'A');
        assert_eq!(atom.residue_number(),     69);
        assert_eq!(atom.insertion_code(),     ' ');
        assert_eq!(atom.x,                    1.0);
        assert_eq!(atom.y,                    2.0);
        assert_eq!(atom.z,                    3.0);
        assert_eq!(atom.occupancy(),          3.14);
        assert_eq!(atom.temperature_factor(), 2.71);
        assert_eq!(atom.element_symbol(),     "O");
        assert_eq!(atom.charge(),             "");
    }
}

#[test]
fn particle_trait_pdb_hetatm() {
    let atom = mill::pdb::HetatmBuilder::new().finalize();
    assert_eq!(atom.name().unwrap(), "");
    assert_eq!(atom.x(), 0.0);
    assert_eq!(atom.y(), 0.0);
    assert_eq!(atom.z(), 0.0);
    assert_eq!(atom.vec(),  na::Vector3::<f64>::new(0.0, 0.0, 0.0));

    let atom = mill::pdb::HetatmBuilder::new()
               .pos(1.0, 2.0, 3.0)
               .atom_name("O")
               .finalize();
    assert_eq!(atom.name().unwrap(), "O");
    assert_eq!(atom.x(), 1.0);
    assert_eq!(atom.y(), 2.0);
    assert_eq!(atom.z(), 3.0);
    assert_eq!(atom.vec(),  na::Vector3::<f64>::new(1.0, 2.0, 3.0));
}
