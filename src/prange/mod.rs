use mceliece::{
    finite_field::{Field, F2},
    matrix::{ColVec, Mat, Perm},
};

use crate::instance::Instance;

pub fn prange(inst: &Instance, max_tries: Option<usize>) -> Option<ColVec<F2>> {
    let (n, w, h, s) = (inst.n(), inst.w(), inst.h(), inst.s());
    let k = h.cols() - h.rows();
    let f2 = h.field();
    let mut tries = 0;
    let mut u = Mat::zero(Field::Some(f2), n - k, n - k);
    let mut h_sf = Mat::zero(Field::Some(f2), n - k, n);
    let mut p = Perm::identity(n);
    let mut us = ColVec::zero(Field::Some(f2), n - k);
    loop {
        h.parity_check_random_standard_form(&mut u, &mut h_sf, &mut p);
        us.mul(&u, s);
        if us.weight() <= w {
            let z = ColVec::zero(Field::Some(f2), k);
            let z_us = ColVec::from(Mat::vconcat(&z.into(), &us.into()));
            return Some(p * z_us);
        }
        if let Some(max) = max_tries {
            tries += 1;
            if tries == max {
                return None;
            }
        }
    }
}
