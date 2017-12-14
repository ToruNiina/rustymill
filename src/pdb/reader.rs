use std::io::{BufRead, BufReader, Read};
use std::string::String;
use std::vec::Vec;
use super::{Atom, Hetatm, Ter};

pub enum Record {
    MODEL{serial : i32},
    ATOM(Atom),
    HETATM(Hetatm),
    TER(Ter),
    ENDMDL,
    Other(String),
}

impl Record {
    fn new(line: String) -> Record {
        let length = line.len();
             if let Ok(atm) = line.parse::<Atom>()   { Record::ATOM(atm) }
        else if let Ok(htm) = line.parse::<Hetatm>() { Record::HETATM(htm) }
        else if let Ok(ter) = line.parse::<Ter>()    { Record::TER(ter) }
        else if length >= 5 && &line[0..5] == "MODEL" {
            let serial: i32 = if length >= 14 {
                (&line[10..14]).parse().unwrap_or(0)
            } else {0};
            Record::MODEL{serial}
        }
        else if length >= 6 && &line[0..6] == "ENDMDL" { Record::ENDMDL }
        else {Record::Other(line)}
    }
}

pub struct Reader<R> {
    reader : BufReader<R>,
}

impl<R : Read> Reader<R> {
    pub fn new(inner: R) -> Self {
        Reader {
            reader: BufReader::new(inner)
        }
    }

    pub fn read_chain(&mut self) -> Vec<Atom> {
        let mut reader = &mut self.reader;
        let mut chain = Vec::<Atom>::new();
        for result in reader.lines() {
            let line = match result {
                Ok(s) => s, Err(e) => panic!(e.to_string())
            };
            match Record::new(line) {
                Record::MODEL{serial} => println!("model {} found.", serial),
                Record::ENDMDL        => println!("model end."),
                Record::TER(_)        => println!("chain terminated."),
                Record::ATOM(atm)     => {println!("ATOM found"); chain.push(atm)},
                Record::HETATM(_)     => println!("heterogen found."),
                Record::Other(_)      => println!("unrecognizable line found."),
            }
        }
        chain
    }
}
