//TODO: Add tests for this module

// use std::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
pub struct WeightedVector {
    support: Vec<usize>,
    len: usize,
}

// impl Index<usize> for WeightedVector {
//     type Output = u32;

//     fn index(&self, index: usize) -> &Self::Output {
//         &self.data[index]
//     }
// }

// impl IndexMut<usize> for WeightedVector {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         &mut self.data[index]
//     }
// }

impl WeightedVector {
    // pub fn new(n: usize, w: usize) -> Self {
    //     let vec = vec![0; n];
    //     for i in 0..w {
    //         vec[i] = 1;
    //     }
    //     Self {
    //         data: vec,
    //         weight: w,
    //     }
    // }

    // pub fn next(&mut self) -> bool {
    //     let fsb = 0;
    //     while (self[fsb] == 0) {
    //         fsb += 1;
    //     }
    //     let next_zero_bit = fsb + 1;
    //     while ()
    //     self[fsb] = 0
    // }
    pub fn new(len: usize, weight: usize) -> Self {
        assert!(weight <= len);
        Self {
            support: (0..weight).collect(),
            len,
        }
    }

    pub fn next(&mut self) -> bool {
        let n = self.len();
        let w = self.weight();
        if w == 0 {
            return false;
        }

        self.support[w - 1] += 1;
        if self.support[w - 1] != n {
            return true;
        }

        let mut i = 2;
        while (i <= w) && (self.support[w - i] == n - i) {
            i += 1;
        }

        if i > w {
            return false;
        }

        self.support[w - i] += 1;
        for j in 1..i {
            self.support[w - i + j] = self.support[w - i] + j;
        }

        return true;
    }

    pub fn reset(&mut self) {
        for i in 0..self.weight() {
            self.support[i] = i;
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn weight(&self) -> usize {
        self.support.len()
    }

    pub fn support(&self) -> &Vec<usize> {
        &self.support
    }
}
