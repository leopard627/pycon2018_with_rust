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
    println!("{}", output);

    &output[..diff] == "0".repeat(diff)
}

fn example(_py: Python, diff: u32) -> PyResult<u32>{

    let mut proof = 0;
    let hasher = Sha256::new();
    let last_proof = 0;

    while valid_proof(hasher, last_proof, proof, diff as usize) == false {
         proof = proof + 1
    }

    Ok(proof)
}

py_module_initializer!(while_with_sha, initexample, PyInit_example, |py, m| {
    m.add(py, "example", py_fn!(py, example(diff: u32)))?;
    Ok(())
});


#[test]
fn test_simple_pychain() {

    let mut proof = 0;
    let hasher = Sha256::new();
    let last_proof = 0;
    let diff = 4;

    while valid_proof(hasher, last_proof, proof, diff as usize) == false {
        proof += 1;
    }

}
