use arrayvec::ArrayString;
use nalgebra::Vector3;
use std::ascii::AsciiExt;
use std::string::String;
use std::str;
use std::fmt;
use super::Particle;

// ------------------------------------ ATOM -----------------------------------

/// The ATOM record in PDB 3.30 format.
#[derive(Debug)]
pub struct Atom {
    record    : ArrayString<[u8;6]>,
    serial    : i32,
    name      : ArrayString<[u8;4]>,
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
    pub fn record_name(&self)        -> &str {self.record.as_str()}
    pub fn atom_number(&self)        -> i32  {self.serial}
    pub fn atom_name(&self)          -> &str {self.name.as_str()}
    pub fn alternate_location(&self) -> char {self.altloc as char}
    pub fn residue_name(&self)       -> &str {self.resname.as_str()}
    pub fn chain_id(&self)           -> char {self.chainid as char}
    pub fn residue_number(&self)     -> i32  {self.resseq}
    pub fn insertion_code(&self)     -> char {self.icode as char}
    pub fn occupancy(&self)          -> f64  {self.occupancy}
    pub fn temperature_factor(&self) -> f64  {self.tempfactor}
    pub fn element_symbol(&self)     -> &str {self.element.as_str()}
    pub fn charge(&self)             -> &str {self.charge.as_str()}

    pub fn new() -> Atom {
        Atom{
            record    : ArrayString::<[u8;6]>::from("ATOM").unwrap(),
            serial    : 1,
            name      : ArrayString::<[u8;4]>::new(),
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

    pub fn from(line: &str) -> Result<Atom, String> {
        if !line.is_ascii() {
            return Err(format!("the line is not encoded as ASCII. \n{}", line).to_owned())
        }
        if &line[0..6] != "ATOM  " {
            return Err(format!("the line is not ATOM line. \n{}", line).to_owned())
        }

        let srl  = try!((&line)[ 6..11].trim().parse::<i32>().map_err(
                        |e| [e.to_string(), format!("at\n{}", line)].concat()));
        let resn = try!((&line)[22..26].trim().parse::<i32>().map_err(
                        |e| [e.to_string(), format!("at\n{}", line)].concat()));
        let x_   = try!((&line)[30..38].trim().parse::<f64>().map_err(
                        |e| [e.to_string(), format!("at\n{}", line)].concat()));
        let y_   = try!((&line)[38..46].trim().parse::<f64>().map_err(
                        |e| [e.to_string(), format!("at\n{}", line)].concat()));
        let z_   = try!((&line)[46..54].trim().parse::<f64>().map_err(
                        |e| [e.to_string(), format!("at\n{}", line)].concat()));

        let occ: f64 = if line.len() < 60 {   0.0  } else {
            try!((&line)[54..60].trim().parse::<f64>().map_err(
                 |e| [e.to_string(), format!("at\n{}", line)].concat()))
        };

        let tmp: f64 = if line.len() < 66 { 999.99 } else {
            try!((&line)[60..66].trim().parse::<f64>().map_err(
                 |e| [e.to_string(), format!("at\n{}", line)].concat()))
        };

        let elem: ArrayString<[u8;2]> = if line.len() >= 78 {
            ArrayString::from(&line[76..78].trim()).unwrap()
        } else {
            if line.as_bytes()[12] == b' ' {
                ArrayString::from(&line[13..14].trim()).unwrap()
            } else {
                ArrayString::from(&line[12..14].trim()).unwrap()
            }
        };

        let chg: ArrayString<[u8;2]> = if line.len() >= 80 {
            ArrayString::from(&line[78..80].trim()).unwrap()
        } else {
            ArrayString::new()
        };

        Ok(Atom{
            record    : ArrayString::<[u8;6]>::from(&line[0..6].trim()).unwrap(),
            serial    : srl,
            name      : ArrayString::<[u8;4]>::from(&line[12..16].trim()).unwrap(),
            altloc    : line.as_bytes()[16],
            resname   : ArrayString::<[u8;3]>::from(&line[17..20].trim()).unwrap(),
            chainid   : line.as_bytes()[21],
            resseq    : resn,
            icode     : line.as_bytes()[26],
            x         : x_,
            y         : y_,
            z         : z_,
            occupancy : occ,
            tempfactor: tmp,
            element   : elem,
            charge    : chg,
        })
    }
}

pub struct AtomBuilder {
    record    : ArrayString<[u8;6]>,
    serial    : i32,
    name      : ArrayString<[u8;4]>,
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

impl AtomBuilder {
    pub fn new() -> AtomBuilder {
        AtomBuilder{
            record    : ArrayString::<[u8;6]>::from("ATOM").unwrap(),
            serial    : 1,
            name      : ArrayString::<[u8;4]>::new(),
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
    pub fn atom_number(&mut self, an: i32) -> &mut AtomBuilder {
        self.serial = an;
        self
    }
    pub fn residue_number(&mut self, rn: i32) -> &mut AtomBuilder {
        self.resseq = rn;
        self
    }
    pub fn chain_id(&mut self, ch: i32) -> &mut AtomBuilder {
        self.chainid = ch as u8;
        self
    }
    pub fn x(&mut self, crd: f64) -> &mut AtomBuilder {self.x = crd; self}
    pub fn y(&mut self, crd: f64) -> &mut AtomBuilder {self.y = crd; self}
    pub fn z(&mut self, crd: f64) -> &mut AtomBuilder {self.z = crd; self}
    pub fn pos(&mut self, px: f64, py: f64, pz: f64) -> &mut AtomBuilder {
        self.x = px;
        self.y = py;
        self.z = pz;
        self
    }
    pub fn record_name(&mut self, rec: &str) -> &mut AtomBuilder {
        self.record = ArrayString::from(rec).unwrap();
        self
    }
    pub fn atom_name(&mut self, atm: &str) -> &mut AtomBuilder {
        self.name = ArrayString::from(atm).unwrap();
        self
    }
    pub fn residue_name(&mut self, res: &str) -> &mut AtomBuilder {
        self.resname = ArrayString::from(res).unwrap();
        self
    }
    pub fn atom_residue_chain(&mut self, atm: &str, res: &str, chn: char)
        -> &mut AtomBuilder {
        self.name    = ArrayString::from(atm).unwrap();
        self.resname = ArrayString::from(res).unwrap();
        self.chainid = chn as u8;
        self
    }

    pub fn occupancy(&mut self, occ: f64) -> &mut AtomBuilder {
        self.occupancy = occ;
        self
    }
    pub fn temperature_factor(&mut self, tfc: f64) -> &mut AtomBuilder {
        self.tempfactor = tfc;
        self
    }
    pub fn element(&mut self, elm: &str) -> &mut AtomBuilder {
        self.element = ArrayString::from(elm).unwrap();
        self
    }
    pub fn charge(&mut self, chg: &str) -> &mut AtomBuilder {
        self.charge = ArrayString::from(chg).unwrap();
        self
    }
    pub fn insertion_code(&mut self, icd: char) -> &mut AtomBuilder {
        self.icode = icd as u8;
        self
    }
    pub fn alternate_location(&mut self, alt: char) -> &mut AtomBuilder {
        self.altloc = alt as u8;
        self
    }
    pub fn finalize(&self) -> Atom {
        Atom {
            record    : self.record,
            serial    : self.serial,
            name      : self.name,
            altloc    : self.altloc,
            resname   : self.resname,
            chainid   : self.chainid,
            resseq    : self.resseq,
            icode     : self.icode,
            x         : self.x,
            y         : self.y,
            z         : self.z,
            occupancy : self.occupancy,
            tempfactor: self.tempfactor,
            element   : self.element,
            charge    : self.charge,
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.element.is_empty() || self.element.len() == 1 {
            // Alignment of one-letter atom name such as C starts at column 14.
            write!(f, "{:<6}{:>5}  {:<3}{}{:<3} {}{:>4}{}   {:8.3}{:8.3}{:8.3}{:6.2}{:6.2}          {:>2}{:>2}",
                self.record, self.serial, self.name,
                self.altloc as char, self.resname, self.chainid as char,
                self.resseq, self.icode as char, self.x, self.y, self.z,
                self.occupancy, self.tempfactor, self.element, self.charge)
        } else {
            // Alignment of two-letter atom name such as FE starts at column 13.
            write!(f, "{:<6}{:>5} {:<4}{}{:<3} {}{:>4}{}   {:8.3}{:8.3}{:8.3}{:6.2}{:6.2}          {:>2}{:>2}",
                self.record, self.serial, self.name,
                self.altloc as char, self.resname, self.chainid as char,
                self.resseq, self.icode as char, self.x, self.y, self.z,
                self.occupancy, self.tempfactor, self.element, self.charge)
        }
    }
}

impl Particle for Atom {
    fn x(&self) -> f64 {self.x}
    fn y(&self) -> f64 {self.y}
    fn z(&self) -> f64 {self.z}
    fn vec(&self) -> Vector3<f64> {
        Vector3::<f64>::new(self.x, self.y, self.z)
    }
    fn name(&self) -> Option<&str> {
        Some(self.atom_name())
    }
}
