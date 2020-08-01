use std::fs;

use syndrome_decoding_problem::isd::{prange, Instance};

pub mod common;

#[test]
fn isd_prange() {
    let paths = fs::read_dir("instances/")
        .unwrap()
        .map(|p| p.unwrap().path());
    for path in paths {
        println!("Name: {}", path.display());
        let inst = Instance::read_instance(path.to_str().unwrap()).unwrap();
        let e = prange(&inst, None).unwrap();
        assert_eq!(inst.h() * e, *inst.s());
    }
}
