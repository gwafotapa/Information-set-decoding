// use std::fs;

use information_set_decoding::{instance::Instance, lee_brickell};

pub mod common;

#[test]
fn lee_brickell_10_0() {
    let inst = Instance::read_instance("instances/SD_10_0").unwrap();
    let e = lee_brickell::lee_brickell(&inst, 1, None).unwrap();
    assert_eq!(inst.h() * e, *inst.s());
}

#[test]
fn lee_brickell_20_0() {
    let inst = Instance::read_instance("instances/SD_20_0").unwrap();
    let e = lee_brickell::lee_brickell(&inst, 1, None).unwrap();
    assert_eq!(inst.h() * e, *inst.s());
}

#[test]
fn lee_brickell_100_0() {
    let inst = Instance::read_instance("instances/SD_100_0").unwrap();
    let e = lee_brickell::lee_brickell(&inst, 2, None).unwrap();
    assert_eq!(inst.h() * e, *inst.s());
}

#[ignore]
#[test]
fn lee_brickell_200_0() {
    let inst = Instance::read_instance("instances/SD_200_0").unwrap();
    let e = lee_brickell::lee_brickell(&inst, 2, None).unwrap();
    assert_eq!(inst.h() * e, *inst.s());
}
