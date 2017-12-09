extern crate arrayvec;
extern crate nalgebra;

pub trait Particle {
    fn x(&self)    -> f64;
    fn y(&self)    -> f64;
    fn z(&self)    -> f64;
    fn vec(&self)  -> nalgebra::Vector3<f64>;
    fn name(&self) -> Option<&str> {None}
}

pub mod pdb;
pub use pdb::atom::Atom;
pub use pdb::atom::AtomBuilder;
pub use pdb::hetatm::Hetatm;
pub use pdb::hetatm::HetatmBuilder;
