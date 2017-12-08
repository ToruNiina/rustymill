use arrayvec::ArrayString;
use std::ascii::AsciiExt;
use std::str;
use std::fmt;

// ------------------------------------ ATOM -----------------------------------

/// The ATOM record in PDB format. The names of the fields are based on
/// PDB 3.30(Nov. 21, 2012).
#[derive(Debug)]
pub struct Atom {
    record    : ArrayString<[u8;6]>,
    serial    : i32,
    name      : ArrayString<[u8;5]>,
    altloc    : u8,
    resname   : ArrayString<[u8;3]>,
    chainid   : u8,
    resseq    : i32,
    icode     : u8,
    pub x     : f64,
    pub y     : f64,
    pub z     : f64,
    occupancy : f64,
    tempfactor: f64,
    element   : ArrayString<[u8;2]>,
    charge    : ArrayString<[u8;2]>,
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
    pub fn alternate_location(&self) -> char {
        self.altloc as char
    }
    pub fn residue_name(&self) -> &str {
        self.resname.as_str()
    }
    pub fn chain_id(&self) -> char {
        self.chainid as char
    }
    pub fn residue_number(&self) -> i32 {
        self.resseq
    }
    pub fn insertion_code(&self) -> char {
        self.icode as char
    }
    pub fn occupancy(&self) -> f64 {
        self.occupancy
    }
    pub fn temperature_factor(&self) -> f64 {
        self.tempfactor
    }
    pub fn element_symbol(&self) -> &str {
        self.element.as_str()
    }
    pub fn charge(&self) -> &str {
        self.charge.as_str()
    }

    pub fn new() -> Atom {
        Atom{
            record    : ArrayString::<[u8;6]>::from("ATOM").unwrap(),
            serial    : 1,
            name      : ArrayString::<[u8;5]>::new(),
            altloc    : b' ',
            resname   : ArrayString::<[u8;3]>::new(),
            chainid   : b'A',
            resseq    : 1,
            icode     : b' ',
            x         : 0.0,
            y         : 0.0,
            z         : 0.0,
            occupancy : 0.0,
            tempfactor: 99.9,
            element   : ArrayString::<[u8;2]>::new(),
            charge    : ArrayString::<[u8;2]>::new()
        }
    }

    pub fn from(line: &str) -> Atom {
        assert!(line.is_ascii());
        Atom{
            record    : ArrayString::<[u8;6]>::from(&line[0..6].trim()).unwrap(),
            serial    : (&line[6..11].trim()).parse::<i32>().unwrap(),
            name      : ArrayString::<[u8;5]>::from(&line[12..16].trim()).unwrap(),
            altloc    : line.as_bytes()[16],
            resname   : ArrayString::<[u8;3]>::from(&line[17..20].trim()).unwrap(),
            chainid   : line.as_bytes()[21],
            resseq    : (&line[22..26].trim()).parse::<i32>().unwrap(),
            icode     : line.as_bytes()[26],
            x         : (&line[30..38].trim()).parse::<f64>().unwrap(),
            y         : (&line[38..46].trim()).parse::<f64>().unwrap(),
            z         : (&line[46..54].trim()).parse::<f64>().unwrap(),
            occupancy : (&line[54..60].trim()).parse::<f64>().unwrap(),
            tempfactor: (&line[60..66].trim()).parse::<f64>().unwrap(),
            element   : ArrayString::<[u8;2]>::from(&line[76..78].trim()).unwrap(),
            charge    : ArrayString::<[u8;2]>::from(&line[78..80].trim()).unwrap()
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.name.len() == 1 {
            write!(f, "{:<6}{:>5}  {:<3}{}{:<3} {}{:>4}{}   {:8.3}{:8.3}{:8.3}{:6.2}{:6.2}          {:>2}{:>2}",
                   self.record, self.serial, self.name, self.altloc as char,
                   self.resname, self.chainid as char, self.resseq,
                   self.icode as char, self.x, self.y, self.z,
                   self.occupancy, self.tempfactor, self.element, self.charge)
        } else {
            write!(f, "{:<6}{:>5} {:<4}{}{:<3} {}{:>4}{}   {:8.3}{:8.3}{:8.3}{:6.2}{:6.2}          {:>2}{:>2}",
                   self.record, self.serial, self.name, self.altloc as char,
                   self.resname, self.chainid as char, self.resseq,
                   self.icode as char, self.x, self.y, self.z,
                   self.occupancy, self.tempfactor, self.element, self.charge)
        }
    }
}


