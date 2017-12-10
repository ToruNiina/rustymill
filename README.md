rustymill
====

[![Build Status](https://travis-ci.org/ToruNiina/rustymill.svg?branch=master)](https://travis-ci.org/ToruNiina/rustymill)

A tool for analyzing or preprocessing molecular dynamics data

## example

```rust
extern crate rustymill as mill;
use mill::pdb::AtomData;

fn main() {
    let atom_line =
        "ATOM  12345  CA  ARG A   1      11.281  86.699  94.383  1.00999.99           N  "

    let atom = atom_line.parse::<mill::pdb::Atom>().unwrap();
    assert_eq!(atom.to_string(), atom_line);

    println!("{}-th atom : name = {}, residue = {}, chain = {}",
             atom.atom_number(), atom.atom_name(), atom.residue_name(),
             atom.chain_id());
}
```
