extern crate rustymill as mill;
extern crate nalgebra  as na;
use mill::Particle;
use mill::pdb::Atom;
use mill::pdb::AtomData;
use mill::pdb::ResidueData;
use mill::pdb::Residue;

#[test]
fn make_residue_iter() {
    {
    let resi = Residue::<Atom>::new();
    assert!(resi.atoms.is_empty());
    }
    {
    let mut atoms = Vec::<Atom>::new();
    atoms.push("ATOM     31  N   ARG A   2      11.281  86.699  94.383  0.50 35.88           N  ".parse::<mill::pdb::Atom>().unwrap());
    atoms.push("ATOM     32  CA  ARG A   2      12.353  85.696  94.456  0.50 36.67           C  ".parse::<mill::pdb::Atom>().unwrap());
    atoms.push("ATOM     33  C   ARG A   2      13.559  86.257  95.222  0.50 37.37           C  ".parse::<mill::pdb::Atom>().unwrap());
    atoms.push("ATOM     34  O   ARG A   2      13.753  87.471  95.270  0.50 37.74           O  ".parse::<mill::pdb::Atom>().unwrap());
    atoms.push("ATOM     35  N   GLY A   2      11.281  86.699  94.383  0.50 35.88           N  ".parse::<mill::pdb::Atom>().unwrap());
    atoms.push("ATOM     36  CA  GLY A   2      12.353  85.696  94.456  0.50 36.67           C  ".parse::<mill::pdb::Atom>().unwrap());
    atoms.push("ATOM     37  C   GLY A   2      13.559  86.257  95.222  0.50 37.37           C  ".parse::<mill::pdb::Atom>().unwrap());
    atoms.push("ATOM     38  O   GLY A   2      13.753  87.471  95.270  0.50 37.74           O  ".parse::<mill::pdb::Atom>().unwrap());

    let res = Residue::from(atoms).unwrap();
    assert_eq!(res.atoms.len(), 4);
    assert_eq!(res.atoms[0].atom_name(),      "N");
    assert_eq!(res.atoms[0].residue_name(),   "ARG");
    assert_eq!(res.atoms[0].residue_number(), 2);
    assert_eq!(res.atoms[0].chain_id(),       'A');
    assert_eq!(res.atoms[1].residue_name(),   "ARG");
    assert_eq!(res.atoms[1].residue_number(), 2);
    assert_eq!(res.atoms[1].chain_id(),       'A');
    assert_eq!(res.atoms[1].atom_name(),      "CA");
    assert_eq!(res.atoms[2].residue_name(),   "ARG");
    assert_eq!(res.atoms[2].residue_number(), 2);
    assert_eq!(res.atoms[2].atom_name(),      "C");
    assert_eq!(res.atoms[2].chain_id(),       'A');
    assert_eq!(res.atoms[3].residue_name(),   "ARG");
    assert_eq!(res.atoms[3].residue_number(), 2);
    assert_eq!(res.atoms[3].atom_name(),      "O");
    assert_eq!(res.atoms[3].chain_id(),       'A');
    }
}
