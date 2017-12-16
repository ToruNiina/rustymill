/// module pdb: read pdb file
use super::Particle;
// use super::{Particle, Snapshot, Trajectory};

pub trait AtomData {
    fn record_name(&self)        -> &str;
    fn atom_number(&self)        -> i32;
    fn atom_name(&self)          -> &str;
    fn alternate_location(&self) -> char;
    fn residue_name(&self)       -> &str;
    fn chain_id(&self)           -> char;
    fn residue_number(&self)     -> i32;
    fn insertion_code(&self)     -> char;
    fn occupancy(&self)          -> f64;
    fn temperature_factor(&self) -> f64;
    fn element_symbol(&self)     -> &str;
    fn charge(&self)             -> &str;
}

pub trait ResidueData {
    fn residue_name(&self)   -> &str;
    fn residue_number(&self) -> i32;
    fn chain_id(&self)       -> char;
}

pub trait ChainData {
    fn chain_id(&self)       -> char;
}


/// pdb::Atom
pub mod atom;
pub use self::atom::Atom;
pub use self::atom::AtomBuilder;

/// pdb::HetAtm
pub mod hetatm;
pub use self::hetatm::Hetatm;
pub use self::hetatm::HetatmBuilder;

/// pdb::Ter
pub mod ter;
pub use self::ter::Ter;

pub mod reader;
pub use self::reader::Record;
pub use self::reader::Reader;

pub mod residue;
pub use self::residue::Residue;
pub use self::residue::ResidueSlice;
pub use self::residue::ResidueIterator;
// pub mod chain;
// pub use self::chain::Chain;

// /// pdb::Residue
// /// pdb::Chain
// /// pdb::Model
// /// pdb::Record
// pub mod chain;
// pub use self::chain::Ter;
// pub use self::chain::Chain;
//
// /// pdb::Model
// /// pdb::EndMdl
// pub mod model;
// pub use self::model::Model;
// pub use self::model::EndMdl;
//
// /// pdb::System is a complete set of chains in a system at some time-point.
// /// while pdb::Model represents MODEL line in pdb format,
// /// pdb::System represents `model` in PDB file.
// pub struct System {
//     serial : i32,
//     chains : std::Vec<Chain>,
// }
//
// impl Snapshot for System {
//
// }
//
// pub struct Data {
//     snapshots : Vec<System>,
// }
//
// impl Trajectory for Data {
//
// }
