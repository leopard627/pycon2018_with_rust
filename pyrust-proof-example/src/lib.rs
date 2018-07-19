#[macro_use]
extern crate cpython;

#[macro_use]
extern crate crypto;

use cpython::{Python, PyResult};
use self::crypto::sha2::Sha256;
use self::crypto::digest::Digest;

 pub fn valid_proof(last_proof: u32, proof: u32) -> bool {

    let mut hasher = Sha256::new();
    let guess = &format!("{}{}", last_proof, proof);
    hasher.input_str(guess);
    let output = hasher.result_str();
    &output[..5] == "00000"
}

 fn example(_py: Python, ) -> PyResult<u32>{

    let mut proof = 0;
    let last_proof = 0;

    while valid_proof(last_proof, proof) == false {
         proof = proof + 1
    }

    Ok(proof)
}
py_module_initializer!(while_with_sha, initexample, PyInit_example, |py, m| {

    m.add(py, "example", py_fn!(py, example()))?;
    Ok(())
});

//py_module_initializer!(example, initexample, PyInit_example, |py, m| {
    //m.add(py, "count_doubles", py_fn!(py, count_doubles(var: &str)))?;
    //Ok(())
/*}*/

