use arrayvec::ArrayString;
use std::ascii::AsciiExt;
use std::string::String;
use std::str::FromStr;
use std::fmt;
use super::AtomData;

pub struct Ter {
    serial  : i32,
    resname : ArrayString<[u8;3]>,
    chainid : u8,
    resseq  : i32,
    icode   : u8,
}

impl FromStr for Ter {
    type Err = String; // TODO!!!

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if !line.is_ascii() {
            return Err(format!("the line is not encoded as ASCII. \n{}", line).to_owned())
        }
        if &line[0..3] != "TER" {
            return Err(format!("the line is not TER line.\n{}", line).to_owned())
        }

        let srl: i32 = if line.len() >= 11 {
            try!((&line)[ 6..11].trim().parse::<i32>().map_err(
                 |e| [e.to_string(), format!("at\n{}", line)].concat()))
        } else {1};

        let res : ArrayString<[u8;3]> = if line.len() >= 20 {
            ArrayString::from((&line)[17..20].trim()).unwrap()
        } else {ArrayString::new()};

        let chid : u8 = if line.len() >= 22 {(&line).as_bytes()[21]} else {b'A'};

        let resn: i32 = if line.len() >= 26 {
            try!((&line)[22..26].trim().parse::<i32>().map_err(
                 |e| [e.to_string(), format!("at\n{}", line)].concat()))
        } else {1};

        let icd : u8 = if line.len() >= 27 {(&line).as_bytes()[26]} else {b' '};

        Ok(Ter{
            serial  : srl,
            resname : res,
            chainid : chid,
            resseq  : resn,
            icode   : icd,
        })
    }
}


impl Ter {
    pub fn record_name(&self)        -> &str {"TER"}
    pub fn atom_number(&self)        -> i32  {self.serial}
    pub fn residue_name(&self)       -> &str {self.resname.as_str()}
    pub fn chain_id(&self)           -> char {self.chainid as char}
    pub fn residue_number(&self)     -> i32  {self.resseq}
    pub fn insertion_code(&self)     -> char {self.icode as char}

    pub fn new<T: AtomData>(last : &T) -> Ter {
        Ter {
            serial  : last.atom_number(),
            resname : ArrayString::from(last.residue_name()).unwrap(),
            chainid : last.chain_id() as u8,
            resseq  : last.residue_number(),
            icode   : last.insertion_code() as u8,
        }
    }
}

impl fmt::Display for Ter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TER   {:>5}      {:<3} {}{:>4}{}                                                     ",
            self.atom_number(), self.residue_name(),
            self.chain_id(), self.residue_number(), self.insertion_code())
    }
}

