rustymill
====

[![Build Status](https://travis-ci.org/ToruNiina/rustymill.svg?branch=master)](https://travis-ci.org/ToruNiina/rustymill)

A tool for analyzing or preprocessing molecular dynamics data

## example

```rs
extern crate rustymill as mill;
use mill::pdb;

fn main() {
    let atom = pdb::Atom::from("ATOM     45  N  BARG A   3      11.281  86.699  94.383  1.00 39.29           N N");

    println!("{}-th atom : name = {}, residue = {}, chain = {}",
             atom.atom_number(), atom.atom_name(), atom.residue_name(),
             atom.chain_id());
}
```
