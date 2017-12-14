extern crate rustymill as mill;
extern crate nalgebra  as na;
use mill::Particle;
use mill::pdb::Atom;
use mill::pdb::AtomData;
use mill::pdb::ResidueIterator;

#[test]
fn make_residue_iter() {
    {
    let atoms = Vec::<Atom>::new();
    let resi = ResidueIterator::new(&atoms);
    assert!(resi.is_none());
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

    let resi = ResidueIterator::new(&atoms);
    assert!(resi.is_some());

    let nxt  = resi.unwrap().next();
    assert!(nxt.is_some());

    let x = nxt.unwrap();
    assert_eq!(x.atoms[0].residue_name(), "ARG");

    }
}
