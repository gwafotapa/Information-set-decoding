use mceliece::{
    finite_field::F2,
    matrix::{ColVec, Mat, Perm},
};
use std::rc::Rc;

use crate::instance::Instance;
use crate::weighted_vector::WeightedVector;

pub fn lee_brickell(inst: &Instance, p: usize, max_tries: Option<usize>) -> Option<ColVec<F2>> {
    let (n, w, h, s) = (inst.n(), inst.w(), inst.h(), inst.s());
    assert!(p <= w);
    let k = h.cols() - h.rows();
    let f2 = h.field();
    let mut tries = 0;
    let mut u = Mat::zero(Rc::clone(&f2), n - k, n - k);
    let mut h_sf = Mat::zero(Rc::clone(&f2), n - k, n);
    let mut pi = Perm::identity(n);
    let mut us = ColVec::zero(Rc::clone(&f2), n - k);
    let mut selection = WeightedVector::new(k, p);
    loop {
        h.parity_check_random_standard_form(&mut u, &mut h_sf, &mut pi);
        us.mul(&u, s);
        selection.reset();
        loop {
            for &col in selection.support() {
                for i in 0..us.rows() {
                    us[i] += h_sf[(i, col)];
                }
            }
            if us.weight() <= w - p {
                let mut e = ColVec::zero(Rc::clone(&f2), n);
                for &col in selection.support() {
                    e[col] = 1;
                }
                for (i, x) in us.data().iter().enumerate() {
                    if *x == 1 {
                        e[k + i] = 1;
                    }
                }
                return Some(pi * e);
            }
            if !selection.next() {
                break;
            }
        }
        if let Some(max) = max_tries {
            tries += 1;
            if tries == max {
                return None;
            }
        }
    }
}
