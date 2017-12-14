use arrayvec::ArrayString;
use std::ascii::AsciiExt;
use std::iter::Iterator;
use std::vec::Vec;
use std::fmt;
use super::AtomData;

pub struct Residue<'a, T: AtomData + 'a> {
    pub atoms : &'a [T],
//     resseq    : i32,
//     name      : ArrayString<[u8;3]>,
//     chain_id  : char,
}

pub struct ResidueIterator<'a, T:AtomData + 'a> {
    first    : usize,
    last     : usize,
    chain_id : char,
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
            Some(ResidueIterator{
                first    : 0,
                last     : idx,
                chain_id : atoms.first().unwrap().chain_id(),
                chain    : &atoms
            })
        }
    }
}

impl<'a, T:AtomData + 'a> Iterator for ResidueIterator<'a, T> {
    type Item = Residue<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Residue{
            atoms  : &self.chain[self.first .. self.last],
        })
    }
}
