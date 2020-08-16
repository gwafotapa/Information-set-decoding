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
    let e1_supports = Rc::new(stern_list_weighted_vectors(k1, p));
    let e2_supports = if k1 == k2 {
        Rc::clone(&e1_supports)
    } else {
        Rc::new(stern_list_weighted_vectors(k2, p))
    };
    let e1_len = e1_supports.len();
    let e2_len = e2_supports.len();
    let mut l1 = vec![ColVec::zero(Rc::clone(&f2), l); e1_len];
    let mut q2e2_us = ColVec::zero(Rc::clone(&f2), l);
    loop {
        // Randomly select an information set and put h in standard form
        h.parity_check_random_standard_form(&mut u, &mut h_sf, &mut pi);

        // Compute the altered syndrome
        us.mul(&u, s);

        // Denoting X the set of the first k1 columns of h restricted to their first l rows,
        // compute the list l1 of p-subset sums of X.
        let q1 = SubMat::new(&h_sf, 0, l, 0, k1);
        for i in 0..e1_len {
            for j in 0..l {
                l1[i][j] = f2.zero();
                for m in 0..p {
                    f2.add_assign(&mut l1[i][j], &q1[(j, e1_supports[i].support()[m])]);
                }
            }
        }

        // Denoting Y the set of columns k1, k1 + 1, ..., k - 1 of h restricted to their first l rows,
        // for each p-subset sum of Y:
        // - add to it (the first l rows of) the altered syndrome,
        // - look for a collision q1e1 = q2e2 + us with list l1 such that the complete sum (on all rows)
        //   q1e1 + q2e2 + us has weight at most 2 * p.
        let q2 = SubMat::new(&h_sf, 0, l, k1, k);
        for i in 0..e2_len {
            for j in 0..l {
                q2e2_us[j] = us[j];
                for m in 0..p {
                    f2.add_assign(&mut q2e2_us[j], &q2[(j, e2_supports[i].support()[m])]);
                }
            }
            for j in 0..e1_len {
                if l1[j] == q2e2_us {
                    let mut weight = 0;
                    for m in l..n - k {
                        let mut qe_us_m = us[m]; // qe_us_m is the mth coefficient of column vector q * e + u * s
                        for &col in e1_supports[j].support() {
                            f2.add_assign(&mut qe_us_m, &h_sf[(m, col)]);
                        }
                        for &col in e2_supports[i].support() {
                            f2.add_assign(&mut qe_us_m, &h_sf[(m, k1 + col)]);
                        }
                        if qe_us_m == f2.one() {
                            weight += 1;
                            if weight > w - 2 * p {
                                break;
                            }
                        }
                    }
                    if weight <= w - 2 * p {
                        let mut e = ColVec::zero(Rc::clone(&f2), n);
                        for &col in e1_supports[j].support() {
                            e[col] = f2.one();
                        }
                        for &col in e2_supports[i].support() {
                            e[k1 + col] = f2.one();
                        }
                        let qe_us = &h_sf * &e + &us;
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

pub fn compute_l(k: usize, p: usize) -> usize {
    let mut l = 0;
    let mut tmp = num_integer::binomial(k - k / 2, p);
    tmp >>= 1;
    while tmp != 0 {
        l += 1;
        tmp >>= 1;
    }
    l
}
