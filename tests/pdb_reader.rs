extern crate rustymill as mill;
extern crate nalgebra  as na;
use mill::Particle;
use mill::pdb::AtomData;

#[test]
fn pdb_reader() {
    let data: &[u8] = b"\
ATOM      1  N   ARG A  10       1.281 106.699  14.383  0.50 35.88           N  
ATOM      2  CA  ARG A  10       2.353 105.696  14.456  0.50 36.67           C  
ATOM      3  C   ARG A  10       3.559 106.257  15.222  0.50 37.37           C  
ATOM      4  O   ARG A  10       3.753 107.471  15.270  0.50 37.74           O  
ATOM      5  CB  ARG A  10       2.774 105.306  13.039  0.50 37.25           C  
ATOM      6  CG  ARG A  10       1.754 104.432  12.321  0.50 38.44           C  
ATOM      7  CD  ARG A  10       1.698 104.678  10.815  0.50 38.51           C  
ATOM      8  NE  ARG A  10       2.984 104.447  10.163  0.50 39.94           N  
ATOM      9  CZ  ARG A  10       3.202 104.534   8.850  0.50 40.03           C  
ATOM     10  NH1 ARG A  10       2.218 104.840   8.007  0.50 40.76           N  
ATOM     11  NH2 ARG A  10       4.421 104.308   8.373  0.50 40.45           N  ";
    let mut reader = mill::pdb::Reader::new(data);
    let chain = reader.read_chain();

    assert_eq!(chain.len(), 11);
    assert_eq!(chain[ 0].to_string(), "ATOM      1  N   ARG A  10       1.281 106.699  14.383  0.50 35.88           N  ");
    assert_eq!(chain[ 1].to_string(), "ATOM      2  CA  ARG A  10       2.353 105.696  14.456  0.50 36.67           C  ");
    assert_eq!(chain[ 2].to_string(), "ATOM      3  C   ARG A  10       3.559 106.257  15.222  0.50 37.37           C  ");
    assert_eq!(chain[ 3].to_string(), "ATOM      4  O   ARG A  10       3.753 107.471  15.270  0.50 37.74           O  ");
    assert_eq!(chain[ 4].to_string(), "ATOM      5  CB  ARG A  10       2.774 105.306  13.039  0.50 37.25           C  ");
    assert_eq!(chain[ 5].to_string(), "ATOM      6  CG  ARG A  10       1.754 104.432  12.321  0.50 38.44           C  ");
    assert_eq!(chain[ 6].to_string(), "ATOM      7  CD  ARG A  10       1.698 104.678  10.815  0.50 38.51           C  ");
    assert_eq!(chain[ 7].to_string(), "ATOM      8  NE  ARG A  10       2.984 104.447  10.163  0.50 39.94           N  ");
    assert_eq!(chain[ 8].to_string(), "ATOM      9  CZ  ARG A  10       3.202 104.534   8.850  0.50 40.03           C  ");
    assert_eq!(chain[ 9].to_string(), "ATOM     10  NH1 ARG A  10       2.218 104.840   8.007  0.50 40.76           N  ");
    assert_eq!(chain[10].to_string(), "ATOM     11  NH2 ARG A  10       4.421 104.308   8.373  0.50 40.45           N  ");
}

