// use log::debug;
use mceliece::{
    finite_field::{Field, F2},
    matrix::{ColVec, Mat, Perm, SubMat},
};
use std::rc::Rc;

use crate::instance::Instance;
use crate::weighted_vector::WeightedVector;

pub fn stern(inst: &Instance, p: usize, l: usize, max_tries: Option<usize>) -> Option<ColVec<F2>> {
    let (n, w, h, s) = (inst.n(), inst.w(), inst.h(), inst.s());
    let k = h.cols() - h.rows();
    assert!(2 * p <= w);
    assert!(0 < l && l <= n - k);
    let k2 = k / 2;
    let k1 = k - k2;
    let f2 = h.field();
    let mut tries = 0;
    let mut u = Mat::zero(Rc::clone(&f2), n - k, n - k);
    let mut h_sf = Mat::zero(Rc::clone(&f2), n - k, n);
    let mut pi = Perm::identity(n);
    let mut us = ColVec::zero(Rc::clone(&f2), n - k);
    let e1_selection = Rc::new(stern_list_weighted_vectors(k1, p));
    let e2_selection = if k1 == k2 {
        Rc::clone(&e1_selection)
    } else {
        Rc::new(stern_list_weighted_vectors(k2, p))
    };
    let e1_len = e1_selection.len();
    let e2_len = e2_selection.len();
    // let e1_len = num_integer::binomial(k1, p);
    // let e2_len = num_integer::binomial(k2, p);
    let mut l1 = vec![ColVec::zero(Rc::clone(&f2), l); e1_len];
    let mut q2e2_us = ColVec::zero(Rc::clone(&f2), l);
    let mut qe_us = ColVec::zero(Rc::clone(&f2), n - k);
    loop {
        h.parity_check_random_standard_form(&mut u, &mut h_sf, &mut pi);
        us.mul(&u, s);
        let q1 = SubMat::new(&h_sf, 0, l, 0, k1);
        for i in 0..e1_len {
            for j in 0..l {
                l1[i][j] = f2.zero();
                for m in 0..p {
                    f2.add_assign(&mut l1[i][j], &q1[(j, e1_selection[i].support()[m])]);
                }
            }
        }
        let q2 = SubMat::new(&h_sf, 0, l, k1, k);
        for i in 0..e2_len {
            for j in 0..l {
                q2e2_us[j] = us[j];
                for m in 0..p {
                    f2.add_assign(&mut q2e2_us[j], &q2[(j, e2_selection[i].support()[m])]);
                }
            }
            for j in 0..e1_len {
                if l1[j] == q2e2_us {
                    for m in 0..us.rows() {
                        qe_us[m] = us[m];
                    }
                    for &col in e1_selection[j].support() {
                        for i in 0..us.rows() {
                            f2.add_assign(&mut qe_us[i], &h_sf[(i, col)]);
                        }
                    }
                    for &col in e2_selection[i].support() {
                        for i in 0..us.rows() {
                            f2.add_assign(&mut qe_us[i], &h_sf[(i, k1 + col)]);
                        }
                    }
                    if qe_us.weight() <= w - 2 * p {
                        let mut e = ColVec::zero(Rc::clone(&f2), n);
                        for &col in e1_selection[j].support() {
                            e[col] = f2.one();
                        }
                        for &col in e2_selection[i].support() {
                            e[k1 + col] = f2.one();
                        }
                        for (i, x) in qe_us.data().iter().enumerate() {
                            if *x == 1 {
                                e[k + i] = f2.one();
                            }
                        }
                        return Some(pi * e);
                    }
                }
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

pub fn stern_list_weighted_vectors(n: usize, w: usize) -> Vec<WeightedVector> {
    let len = num_integer::binomial(n, w);
    let mut list = Vec::with_capacity(len);
    let mut e = WeightedVector::new(n, w);
    for _ in 0..len {
        list.push(e.clone());
        e.next();
    }
    list
}
