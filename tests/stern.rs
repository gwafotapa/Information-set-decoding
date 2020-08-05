// use std::fs;

use information_set_decoding::{instance::Instance, stern};

pub mod common;

#[ignore]
#[test]
fn list_weighted_vectors() {
    let l = stern::stern_list_weighted_vectors(10, 3);
    println!("{:?}", l);
}

fn compute_l(k: usize, p: usize) -> usize {
    let mut l = 0;
    let mut tmp = num_integer::binomial(k - k / 2, p);
    tmp >>= 1;
    while tmp != 0 {
        l += 1;
        tmp >>= 1;
    }
    l
}

// #[ignore]
#[test]
fn stern_10_0() {
    let inst = Instance::read_instance("instances/SD_10_0").unwrap();
    let k = inst.h().cols() - inst.h().rows();
    let p = 2;
    let l = compute_l(k, p);
    let e = stern::stern(&inst, p, l, None).unwrap();
    assert_eq!(inst.h() * e, *inst.s());
}

#[ignore]
#[test]
fn stern_20_0() {
    let inst = Instance::read_instance("instances/SD_20_0").unwrap();
    let k = inst.h().cols() - inst.h().rows();
    let p = 2;
    let l = compute_l(k, p);
    let e = stern::stern(&inst, p, l, None).unwrap();
    assert_eq!(inst.h() * e, *inst.s());
}

#[ignore]
#[test]
fn stern_100_0() {
    let inst = Instance::read_instance("instances/SD_100_0").unwrap();
    let k = inst.h().cols() - inst.h().rows();
    let p = 2;
    let l = compute_l(k, p);
    let e = stern::stern(&inst, p, l, None).unwrap();
    assert_eq!(inst.h() * e, *inst.s());
}

#[ignore]
#[test]
fn stern_200_0() {
    let inst = Instance::read_instance("instances/SD_200_0").unwrap();
    let k = inst.h().cols() - inst.h().rows();
    let p = 2;
    let l = compute_l(k, p);
    let e = stern::stern(&inst, p, l, None).unwrap();
    assert_eq!(inst.h() * e, *inst.s());
}
