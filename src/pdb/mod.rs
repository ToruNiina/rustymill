/// module pdb: read pdb file
use super::Particle;
// use super::{Particle, Snapshot, Trajectory};

/// pdb::Atom
pub mod atom;
pub use self::atom::Atom;
pub use self::atom::AtomBuilder;

// /// pdb::HetAtm
// pub mod hetatm;
// pub use self::hetatm::HetAtm;
// pub use self::hetatm::HetAtmBuilder;
//
// /// pdb::Chain
// /// pdb::Ter
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
