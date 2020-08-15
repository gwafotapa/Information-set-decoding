// use std::fs;

use information_set_decoding::{instance::Instance, stern};
use information_set_decoding::{ColVec, Field, Mat, F2};
use log::debug;
use std::rc::Rc;

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

// #[ignore]
#[test]
fn stern_20_0() {
    let inst = Instance::read_instance("instances/SD_20_0").unwrap();
    let k = inst.h().cols() - inst.h().rows();
    let p = 2;
    let l = compute_l(k, p);
    let e = stern::stern(&inst, p, l, None).unwrap();
    assert_eq!(inst.h() * e, *inst.s());
}

// #[ignore]
#[test]
fn stern_100_0() {
    let inst = Instance::read_instance("instances/SD_100_0").unwrap();
    let k = inst.h().cols() - inst.h().rows();
    let p = 2;
    let l = compute_l(k, p);
    let e = stern::stern(&inst, p, l, None).unwrap();
    assert_eq!(inst.h() * e, *inst.s());
}

// #[ignore]
#[test]
fn stern_200_0() {
    let inst = Instance::read_instance("instances/SD_200_0").unwrap();
    let k = inst.h().cols() - inst.h().rows();
    let p = 2;
    let l = compute_l(k, p);
    let e = stern::stern(&inst, p, l, None).unwrap();
    assert_eq!(inst.h() * e, *inst.s());
}

// #[test]
// fn stern_with_solution_10() {
//     common::log_setup();
//     let inst = Instance::read_instance("instances/stern_10").unwrap();
//     let k = inst.h().cols() - inst.h().rows();
//     let p = 2;
//     let l = compute_l(k, p);
//     debug!(
//         "n={}   k={}   w={}   p={}   l={}\n",
//         inst.n(),
//         k,
//         inst.w(),
//         p,
//         l
//     );
//     let e = stern::stern(&inst, p, l, None).unwrap();
//     assert_eq!(inst.h() * e, *inst.s());
// }

#[test]
fn stern_with_solution_10() {
    common::log_setup();
    let (n, w, p) = (10, 4, 2);
    let k = n / 2;
    let l = compute_l(k, p);
    let (inst, x) = generate_instance_with_solution(n, w, p, l);
    assert!(x.weight() == w);
    assert_eq!(inst.h() * &x, *inst.s());
    debug!(
        "h: {}\nx: {}\ns: {}",
        inst.h(),
        x.transpose(),
        inst.s().transpose()
    );
    let e = stern::stern(&inst, p, l, None).unwrap();
    assert_eq!(inst.h() * e, *inst.s());
}

fn generate_instance_with_solution(
    n: usize,
    w: usize,
    p: usize,
    l: usize,
) -> (Instance, ColVec<F2>) {
    let f2 = Rc::new(F2::generate(()));
    let k = n / 2;
    let k2 = k / 2;
    let k1 = k - k2;
    let h = Mat::random_standard_form_parity_check_matrix(Rc::clone(&f2), n, k);
    let mut e = ColVec::zero(Rc::clone(&f2), n);
    for i in 0..p {
        e[i] = f2.one();
    }
    for i in k1..k1 + p {
        e[i] = f2.one();
    }
    for i in k + l..k + l + w - 2 * p {
        e[i] = f2.one();
    }
    let s = &h * &e;
    let inst = Instance::new(n, w, h, s);

    (inst, e)
}
