#[macro_use]
extern crate cpython;

use cpython::{Python, PyResult};

fn count_doubles(_py: Python, val: &str) -> PyResult<u64> {
    let mut total = 0u64;

    for (c1, c2) in val.chars().zip(val.chars().skip(1)) {
        if c1 == c2 {
            total += 1;
        }
    }

    Ok(total)
}

py_module_initializer!(libdoubles, initlibdoubles, PyInit_doubles, |py, m | {
    try!(m.add(py, "__doc__", "I love Python and Rust!! 저는 파이선 그리고 러스트를 좋아합니다."));
    try!(m.add(py, "count_doubles", py_fn!(py, count_doubles(val: &str))));
    Ok(())
});



#[test]
fn test_doubles() {

    let val = String::from("aaabbddasdsads`213123aaasdazxccxzaaasdd11123`");
    let mut total = 0u64;

    for (c1, c2) in val.chars().zip(val.chars().skip(1)) {
        if c1 == c2 {
            total += 1;
        }
    }

    println!("{}", total);
}
