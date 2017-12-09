extern crate rustymill as mill;
use mill::pdb::AtomData;

fn main() {
    let atom =
        "ATOM     45  N  BARG A   3      11.281  86.699  94.383  1.00 39.29           N  "
        .parse::<mill::pdb::Atom>().unwrap();

    println!("{}-th atom : name = {}, residue = {}, chain = {}",
             atom.atom_number(), atom.atom_name(), atom.residue_name(),
             atom.chain_id());
}

