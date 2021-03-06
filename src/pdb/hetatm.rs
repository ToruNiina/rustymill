use arrayvec::ArrayString;
use nalgebra::Vector3;
use std::ascii::AsciiExt;
use std::string::String;
use std::str::FromStr;
use std::str;
use std::fmt;
use super::Particle;
use super::AtomData;

// ---------------------------------- HETATM -----------------------------------

/// The HETATM record in PDB 3.30 format.
#[derive(Debug)]
pub struct Hetatm {
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

impl AtomData for Hetatm {
    fn record_name(&self)        -> &str {"HETATM"}
    fn atom_number(&self)        -> i32  {self.serial}
    fn atom_name(&self)          -> &str {self.name.as_str()}
    fn alternate_location(&self) -> char {self.altloc as char}
    fn residue_name(&self)       -> &str {self.resname.as_str()}
    fn chain_id(&self)           -> char {self.chainid as char}
    fn residue_number(&self)     -> i32  {self.resseq}
    fn insertion_code(&self)     -> char {self.icode as char}
    fn occupancy(&self)          -> f64  {self.occupancy}
    fn temperature_factor(&self) -> f64  {self.tempfactor}
    fn element_symbol(&self)     -> &str {self.element.as_str()}
    fn charge(&self)             -> &str {self.charge.as_str()}
}

impl Particle for Hetatm {
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

impl FromStr for Hetatm {
    type Err = String; // TODO!!!
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if !line.is_ascii() {
            return Err(format!("the line is not encoded as ASCII. \n{}", line).to_owned())
        }
        if line.len() < 54 {
            return Err(format!("the line doesn't have enough length. \n{}", line).to_owned())
        }
        if &line[0..6] != "HETATM" {
            return Err(format!("the line is not HETATM line. \n{}", line).to_owned())
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

        Ok(Hetatm{
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

impl Hetatm {
    pub fn new() -> Hetatm {
        Hetatm{
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
}

pub struct HetatmBuilder {
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

impl HetatmBuilder {
    pub fn new() -> HetatmBuilder {
        HetatmBuilder{
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
    pub fn atom_number(&mut self, an: i32) -> &mut HetatmBuilder {
        self.serial = an;
        self
    }
    pub fn residue_number(&mut self, rn: i32) -> &mut HetatmBuilder {
        self.resseq = rn;
        self
    }
    pub fn chain_id(&mut self, ch: char) -> &mut HetatmBuilder {
        self.chainid = ch as u8;
        self
    }
    pub fn x(&mut self, crd: f64) -> &mut HetatmBuilder {self.x = crd; self}
    pub fn y(&mut self, crd: f64) -> &mut HetatmBuilder {self.y = crd; self}
    pub fn z(&mut self, crd: f64) -> &mut HetatmBuilder {self.z = crd; self}
    pub fn pos(&mut self, px: f64, py: f64, pz: f64) -> &mut HetatmBuilder {
        self.x = px;
        self.y = py;
        self.z = pz;
        self
    }
    pub fn atom_name(&mut self, atm: &str) -> &mut HetatmBuilder {
        self.name = ArrayString::from(atm).unwrap();
        self
    }
    pub fn residue_name(&mut self, res: &str) -> &mut HetatmBuilder {
        self.resname = ArrayString::from(res).unwrap();
        self
    }
    pub fn atom_residue_chain(&mut self, atm: &str, res: &str, chn: char)
        -> &mut HetatmBuilder {
        self.name    = ArrayString::from(atm).unwrap();
        self.resname = ArrayString::from(res).unwrap();
        self.chainid = chn as u8;
        self
    }

    pub fn occupancy(&mut self, occ: f64) -> &mut HetatmBuilder {
        self.occupancy = occ;
        self
    }
    pub fn temperature_factor(&mut self, tfc: f64) -> &mut HetatmBuilder {
        self.tempfactor = tfc;
        self
    }
    pub fn element(&mut self, elm: &str) -> &mut HetatmBuilder {
        self.element = ArrayString::from(elm).unwrap();
        self
    }
    pub fn charge(&mut self, chg: &str) -> &mut HetatmBuilder {
        self.charge = ArrayString::from(chg).unwrap();
        self
    }
    pub fn insertion_code(&mut self, icd: char) -> &mut HetatmBuilder {
        self.icode = icd as u8;
        self
    }
    pub fn alternate_location(&mut self, alt: char) -> &mut HetatmBuilder {
        self.altloc = alt as u8;
        self
    }
    pub fn finalize(&self) -> Hetatm {
        Hetatm {
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

impl fmt::Display for Hetatm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.element.is_empty() || self.element.len() == 1 {
            // Alignment of one-letter atom name such as C starts at column 14.
            write!(f, "{:<6}{:>5}  {:<3}{}{:<3} {}{:>4}{}   {:8.3}{:8.3}{:8.3}{:6.2}{:6.2}          {:>2}{:>2}",
                self.record_name(), self.serial, self.name,
                self.altloc as char, self.resname, self.chainid as char,
                self.resseq, self.icode as char, self.x, self.y, self.z,
                self.occupancy, self.tempfactor, self.element, self.charge)
        } else {
            // Alignment of two-letter atom name such as FE starts at column 13.
            write!(f, "{:<6}{:>5} {:<4}{}{:<3} {}{:>4}{}   {:8.3}{:8.3}{:8.3}{:6.2}{:6.2}          {:>2}{:>2}",
                self.record_name(), self.serial, self.name,
                self.altloc as char, self.resname, self.chainid as char,
                self.resseq, self.icode as char, self.x, self.y, self.z,
                self.occupancy, self.tempfactor, self.element, self.charge)
        }
    }
}
