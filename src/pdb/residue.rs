use arrayvec::ArrayString;
use std::ascii::AsciiExt;
use std::iter::{Extend, IntoIterator, FromIterator};
use std::vec::Vec;
use std::fmt;
use super::{AtomData, ResidueData};

pub struct Residue<T: AtomData> {
    pub atoms : Vec<T>,
    name      : ArrayString<[u8;3]>,
    resseq    : i32,
    chain_id  : char,
}

impl<T: AtomData> Residue<T> {
    pub fn new() -> Self {
        Residue{
            atoms: Vec::new(), name: ArrayString::new(), resseq: 1, chain_id: 'A'
        }
    }

    pub fn from(v: Vec<T>) -> Option<Self> {
        if v.is_empty() {
            None
        } else {
            let resn = ArrayString::<[u8;3]>::from(
                v.first().unwrap().residue_name()).unwrap();
            let resi = v.first().unwrap().residue_number();
            let chid = v.first().unwrap().chain_id();
            let mut res = Vec::<T>::new();
            for elem in v {
                if elem.residue_name()   == resn.as_str() &&
                   elem.residue_number() == resi &&
                   elem.chain_id()       == chid {
                    res.push(elem);
                }
            }
            Some(Residue{atoms: res, name: resn, resseq: resi, chain_id: chid})
        }
    }
}

impl<T: AtomData> ResidueData for Residue<T> {
    fn residue_name(&self)   -> &str {self.name.as_str()}
    fn residue_number(&self) -> i32  {self.resseq}
    fn chain_id(&self)       -> char {self.chain_id}
}

impl<T: AtomData> IntoIterator for Residue<T> {
    type Item = T;
    type IntoIter = ::std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.atoms.into_iter()
    }
}

impl<T: AtomData> FromIterator<T> for Residue<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let atms = ::std::vec::Vec::<T>::from_iter(iter);
        if atms.is_empty() {
            Residue{
                atoms: atms, name: ArrayString::new(), resseq: 1, chain_id: 'A'
            }
        } else {
            let resn = ArrayString::<[u8;3]>::from(
                atms.first().unwrap().residue_name()).unwrap();
            let resi = atms.first().unwrap().residue_number();
            let chid = atms.first().unwrap().chain_id();
            Residue{
                atoms: atms, name: resn, resseq: resi, chain_id: chid
            }
        }
    }
}

impl<T: AtomData> Extend<T> for Residue<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for elem in iter {
            if (elem.residue_name() == self.residue_name() &&
                elem.chain_id() == self.chain_id()) || self.atoms.is_empty() {
                self.atoms.push(elem);
            }
        }
    }
}

pub struct ResidueSlice<'a, T: AtomData + 'a> {
    pub atoms : &'a [T],
    name      : ArrayString<[u8;3]>,
    resseq    : i32,
    chain_id  : char,
}

impl<'a, T:AtomData + 'a> ResidueData for ResidueSlice<'a, T> {
    fn residue_name(&self)   -> &str {self.name.as_str()}
    fn residue_number(&self) -> i32  {self.resseq}
    fn chain_id(&self)       -> char {self.chain_id}
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
    type Item = ResidueSlice<'a, T>;

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
            self.first = lst;
            self.last = idx;
        }

        Some(ResidueSlice{
            atoms    : &self.chain[fst .. lst],
            name     : ArrayString::from(rep.residue_name()).unwrap(),
            resseq   : rep.residue_number(),
            chain_id : rep.chain_id(),
        })
    }
}
