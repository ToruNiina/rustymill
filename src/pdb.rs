pub mod pdb {

extern crate approx;
extern crate nalgebra;
extern crate arrayvec;
use std::ascii::AsciiExt;
use std::string;
use std::str;


struct Atom {
    record    : arrayvec::ArrayString<[u8;6]>,
    serial    : i32,
    name      : arrayvec::ArrayString<[u8;5]>,
    altLoc    : u8,
    resName   : arrayvec::ArrayString<[u8;3]>,
    chainID   : u8,
    resSeq    : i32,
    iCode     : u8,
    position  : nalgebra::Vector3<f64>,
    occupancy : f64,
    tempFactor: f64,
    element   : arrayvec::ArrayString<[u8;2]>,
    charge    : arrayvec::ArrayString<[u8;2]>,
}

impl Atom {
    fn record_name(&self) -> &str {
        self.record.as_str()
    }
    fn atom_number(&self) -> i32 {
        self.serial
    }
    fn atom_name(&self) -> &str {
        self.name.as_str()
    }
    fn alternate_location(&self) -> u8 {
        self.altLoc
    }
    fn residue_name(&self) -> &str {
        self.resName.as_str()
    }
    fn chain_id(&self) -> u8 {
        self.chainID
    }
    fn residue_number(&self) -> i32 {
        self.resSeq
    }
    fn insertion_code(&self) -> u8 {
        self.iCode
    }
    fn x(&self) -> f64 {
        self.position.x
    }
    fn y(&self) -> f64 {
        self.position.y
    }
    fn z(&self) -> f64 {
        self.position.z
    }
    fn position(&self) -> &nalgebra::Vector3<f64> {
        &self.position
    }
    fn occupancy(&self) -> f64 {
        self.occupancy
    }
    fn temperature_factor(&self) -> f64 {
        self.tempFactor
    }
    fn element_symbol(&self) -> &str {
        self.element.as_str()
    }
    fn charge(&self) -> &str {
        self.charge.as_str()
    }

    fn new() -> Atom {
        Atom{
            record    : arrayvec::ArrayString::<[u8;6]>::from("ATOM").unwrap(),
            serial    : 1,
            name      : arrayvec::ArrayString::<[u8;5]>::new(),
            altLoc    : b' ',
            resName   : arrayvec::ArrayString::<[u8;3]>::new(),
            chainID   : b'A',
            resSeq    : 1,
            iCode     : b' ',
            position  : nalgebra::Vector3::<f64>::new(0.0,0.0,0.0),
            occupancy : 0.0,
            tempFactor: 99.9,
            element   : arrayvec::ArrayString::<[u8;2]>::new(),
            charge    : arrayvec::ArrayString::<[u8;2]>::new()
        }
    }

    fn from(line: &str) -> Atom {
        assert!(line.is_ascii());
        Atom{
            record    : arrayvec::ArrayString::<[u8;6]>::from(&line[0..6].trim()).unwrap(),
            serial    : (&line[6..11].trim()).parse::<i32>().unwrap(),
            name      : arrayvec::ArrayString::<[u8;5]>::from(&line[12..16].trim()).unwrap(),
            altLoc    : line.as_bytes()[16],
            resName   : arrayvec::ArrayString::<[u8;3]>::from(&line[17..20].trim()).unwrap(),
            chainID   : line.as_bytes()[21],
            resSeq    : (&line[22..26].trim()).parse::<i32>().unwrap(),
            iCode     : line.as_bytes()[26],
            position  : nalgebra::Vector3::<f64>::new(0.0,0.0,0.0),
            occupancy : (&line[54..60].trim()).parse::<f64>().unwrap(),
            tempFactor: (&line[60..66].trim()).parse::<f64>().unwrap(),
            element   : arrayvec::ArrayString::<[u8;2]>::from(&line[76..78].trim()).unwrap(),
            charge    : arrayvec::ArrayString::<[u8;2]>::from(&line[78..80].trim()).unwrap()
        }
    }
}
}
