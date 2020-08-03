use mceliece::{
    finite_field::{Field, F2},
    matrix::{ColVec, Mat},
};

use crate::instance::Instance;

pub fn prange(inst: &Instance, max_tries: Option<usize>) -> Option<ColVec<F2>> {
    let (w, h, s) = (inst.w(), inst.h(), inst.s());
    let k = h.cols() - h.rows();
    let f2 = h.field();
    let mut tries = 0;
    loop {
        // if let Some((u, _, p)) = h.random_standard_form() {
        //     let us = u * s;
        //     if us.weight() <= w {
        //         let z = ColVec::zero(Field::Some(f2), k);
        //         let z_us = ColVec::from(Mat::vconcat(&z.into(), &us.into()));
        //         return Some(p * z_us);
        //     }
        // }
        let (u, _, p) = h.parity_check_random_standard_form();
        let us = u * s;
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
