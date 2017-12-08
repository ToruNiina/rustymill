extern crate rustymill as mill;

#[test]
fn write_pdb_line() {
    let atom = mill::pdb::Atom::from("ATOM     45  N  BARG A   3      11.281  86.699  94.383  1.00 39.29           N N");
    let line = format!("{}", atom);
    println!("{}", atom);
    assert_eq!(line, "ATOM     45  N  BARG A   3      11.281  86.699  94.383  1.00 39.29           N N");
}
