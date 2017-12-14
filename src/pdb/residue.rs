use arrayvec::ArrayString;
use std::ascii::AsciiExt;
use std::iter::Iterator;
use std::vec::Vec;
use std::fmt;
use super::AtomData;

pub struct Residue<'a, T: AtomData + 'a> {
    pub atoms : &'a [T],
    pub name  : ArrayString<[u8;3]>,
    resseq    : i32,
    chain_id  : char,
}

impl<'a, T:AtomData + 'a> Residue<'a, T> {
    pub fn residue_number(&self) -> i32  {self.resseq}
    pub fn chain_id(&self)       -> char {self.chain_id}
}

pub struct ResidueIterator<'a, T:AtomData + 'a> {
    first    : usize,
    last     : usize,
    chain    : &'a Vec<T>,
}

impl<'a, T:AtomData + 'a> ResidueIterator<'a, T> {
    pub fn new(atoms : &Vec<T>) -> Option<ResidueIterator<T>> {
        if atoms.is_empty() { None } else {
            let mut idx: usize = 0;
            let resn = &atoms[0].residue_name();
            while idx < atoms.len() && &atoms[idx].residue_name() == resn {
                idx += 1;
            }
            Some(ResidueIterator{first : 0, last : idx, chain : &atoms})
        }
    }
}

impl<'a, T:AtomData + 'a> Iterator for ResidueIterator<'a, T> {
    type Item = Residue<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first == self.chain.len() {
            return None
        }

        let fst = self.first;
        let lst = self.last;
        let rep = &(self.chain[self.first]);

        if lst != self.chain.len() {
            let mut idx = lst;
            let next_resn = &(self.chain[self.last]).residue_name();
            let next_chid = &(self.chain[self.last]).chain_id();

            while idx < self.chain.len() &&
                &self.chain[idx].chain_id() == next_chid &&
                &self.chain[idx].residue_name() == next_resn {
                idx += 1;
            }
            self.first = self.last;
            self.last = idx;
        }

        Some(Residue{
            atoms    : &self.chain[fst .. lst],
            name     : ArrayString::from(rep.residue_name()).unwrap(),
            resseq   : rep.residue_number(),
            chain_id : rep.chain_id(),
        })
    }
}
