extern crate approx;
extern crate nalgebra;
extern crate arrayvec;
use std::ascii::AsciiExt;
use std::str;
use std::fmt;

#[derive(Debug)]
pub struct Atom {
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
    pub fn record_name(&self) -> &str {
        self.record.as_str()
    }
    pub fn atom_number(&self) -> i32 {
        self.serial
    }
    pub fn atom_name(&self) -> &str {
        self.name.as_str()
    }
    pub fn alternate_location(&self) -> u8 {
        self.altLoc
    }
    pub fn residue_name(&self) -> &str {
        self.resName.as_str()
    }
    pub fn chain_id(&self) -> u8 {
        self.chainID
    }
    pub fn residue_number(&self) -> i32 {
        self.resSeq
    }
    pub fn insertion_code(&self) -> u8 {
        self.iCode
    }
    pub fn x(&self) -> f64 {
        self.position.x
    }
    pub fn y(&self) -> f64 {
        self.position.y
    }
    pub fn z(&self) -> f64 {
        self.position.z
    }
    pub fn position(&self) -> &nalgebra::Vector3<f64> {
        &self.position
    }
    pub fn occupancy(&self) -> f64 {
        self.occupancy
    }
    pub fn temperature_factor(&self) -> f64 {
        self.tempFactor
    }
    pub fn element_symbol(&self) -> &str {
        self.element.as_str()
    }
    pub fn charge(&self) -> &str {
        self.charge.as_str()
    }

    pub fn new() -> Atom {
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

    pub fn from(line: &str) -> Atom {
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
            position  : nalgebra::Vector3::<f64>::new(
                            (&line[30..38].trim()).parse::<f64>().unwrap(),
                            (&line[38..46].trim()).parse::<f64>().unwrap(),
                            (&line[46..54].trim()).parse::<f64>().unwrap()
                        ),
            occupancy : (&line[54..60].trim()).parse::<f64>().unwrap(),
            tempFactor: (&line[60..66].trim()).parse::<f64>().unwrap(),
            element   : arrayvec::ArrayString::<[u8;2]>::from(&line[76..78].trim()).unwrap(),
            charge    : arrayvec::ArrayString::<[u8;2]>::from(&line[78..80].trim()).unwrap()
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.name.len() == 1 {
            write!(f, "{:<6}{:>5}  {:<3}{}{:<3} {}{:>4}{}   {:8.3}{:8.3}{:8.3}{:6.2}{:6.2}          {:>2}{:>2}",
                   self.record, self.serial, self.name, self.altLoc as char,
                   self.resName, self.chainID as char, self.resSeq, self.iCode as char,
                   self.position.x, self.position.y, self.position.z,
                   self.occupancy, self.tempFactor, self.element, self.charge)
        } else {
            write!(f, "{:<6}{:>5} {:<4}{}{:<3} {}{:>4}{}   {:8.3}{:8.3}{:8.3}{:6.2}{:6.2}          {:>2}{:>2}",
                   self.record, self.serial, self.name, self.altLoc as char,
                   self.resName, self.chainID as char, self.resSeq, self.iCode as char,
                   self.position.x, self.position.y, self.position.z,
                   self.occupancy, self.tempFactor, self.element, self.charge)
        }
    }
}
