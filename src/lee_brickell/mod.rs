use mceliece::{
    finite_field::{Field, F2},
    matrix::ColVec,
};

use crate::instance::Instance;
use weighted_bit_vector::WeightedBitVector;

pub fn lee_brickell(inst: &Instance, p: usize, max_tries: Option<usize>) -> Option<ColVec<F2>> {
    let (n, w, h, s) = (inst.n(), inst.w(), inst.h(), inst.s());
    assert!(p <= w);
    let k = h.cols() - h.rows();
    let f2 = h.field();
    let mut tries = 0;
    loop {
        let (u, hs, pi) = h.parity_check_random_standard_form();
        let us = u * s;
        let mut sum_of_columns = us.clone();
        let mut selection = WeightedBitVector::new(k, p);
        loop {
            for &col in selection.support() {
                for i in 0..sum_of_columns.rows() {
                    sum_of_columns[i] += hs[(i, col)];
                }
            }
            if sum_of_columns.weight() <= w - p {
                let mut e = ColVec::zero(Field::Some(f2), n);
                for &col in selection.support() {
                    e[col] = 1;
                }
                for (i, x) in sum_of_columns.data().iter().enumerate() {
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

mod weighted_bit_vector;
