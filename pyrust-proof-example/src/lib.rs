#[macro_use]
extern crate cpython;

#[macro_use]
extern crate crypto;

use cpython::{Python, PyResult};
use self::crypto::sha2::Sha256;
use self::crypto::digest::Digest;


pub fn valid_proof(mut hasher: Sha256, last_proof: u32, proof: u32, diff: usize) -> bool {
    let guess = &format!("{}{}", last_proof, proof);
    hasher.input_str(guess);
    let output = hasher.result_str();
    &output[..diff] == "0".repeat(diff)
}

fn proofs(_py: Python, diff: u32) -> PyResult<u32>{

    let mut proof = 0;
    let hasher = Sha256::new();
    let last_proof = 0;

    while valid_proof(hasher, last_proof, proof, diff as usize) == false {
         proof = proof + 1
    }

    Ok(proof)
}

py_module_initializer!(pycon2018_proofs, initproofs, PyInit_proofs, |py, m| {
    try!(m.add(py, "__doc__", "파이콘 만세 ~ 러스트 만세!"));
    m.add(py, "proofs", py_fn!(py, proofs(diff: u32)))?;
    Ok(())
});


#[test]
fn test_simple_pychain_proofs() {

    let mut proof = 0;
    let hasher = Sha256::new();
    let last_proof = 0;
    let diff = 4;

    while valid_proof(hasher, last_proof, proof, diff as usize) == false {
        proof += 1;
    }

}
