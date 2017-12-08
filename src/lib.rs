extern crate arrayvec;
extern crate nalgebra;

pub trait Particle {
    fn x(&self)    -> f64;
    fn y(&self)    -> f64;
    fn z(&self)    -> f64;
    fn pos(&self)  -> nalgebra::Vector3<f64>;
    fn name(&self) -> Option<&str> {None}
}

pub mod pdb;
