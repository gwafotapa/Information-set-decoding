// use std::fs;

use information_set_decoding::{instance::Instance, prange};

pub mod common;

// fn isd_prange() {
//     let paths = fs::read_dir("instances/")
//         .unwrap()
//         .map(|p| p.unwrap().path());
//     for path in paths {
//         println!("Name: {}", path.display());
//         let inst = Instance::read_instance(path.to_str().unwrap()).unwrap();
//         let e = prange(&inst, None).unwrap();
//         assert_eq!(inst.h() * e, *inst.s());
//     }
// }

#[test]
fn prange_10_0() {
    let inst = Instance::read_instance("instances/SD_10_0").unwrap();
    let e = prange::prange(&inst, None).unwrap();
    assert_eq!(inst.h() * e, *inst.s());
}

#[test]
fn prange_20_0() {
    let inst = Instance::read_instance("instances/SD_20_0").unwrap();
    let e = prange::prange(&inst, None).unwrap();
    assert_eq!(inst.h() * e, *inst.s());
}

#[test]
fn prange_100_0() {
    let inst = Instance::read_instance("instances/SD_100_0").unwrap();
    let e = prange::prange(&inst, None).unwrap();
    assert_eq!(inst.h() * e, *inst.s());
}

#[ignore]
#[test]
fn prange_200_0() {
    let inst = Instance::read_instance("instances/SD_200_0").unwrap();
    let e = prange::prange(&inst, None).unwrap();
    assert_eq!(inst.h() * e, *inst.s());
}
