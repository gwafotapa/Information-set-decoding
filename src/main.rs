use log::debug;
use main_error::MainError;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
    result,
};

use mceliece::{
    finite_field::Field, finite_field::FieldTrait, finite_field::F2, matrix::ColVec, matrix::Mat,
};

// type Result<T> = std::result::Result<T, Box<dyn Error>>;

struct Instance {
    n: usize,
    w: usize,
    h: Mat<F2>,
    s: ColVec<F2>,
}

fn read_instance(file_name: &str) -> result::Result<Instance, Box<dyn Error>> {
    let f = File::open(file_name)?;
    let f = BufReader::new(f);
    let mut lines = f.lines().map(|l| l.unwrap());
    lines.next().ok_or("Line 0 is missing.")?;
    let line1 = lines.next().ok_or("Line 1 (value of n) is missing.")?;
    let n = line1.parse::<usize>()?;
    lines.next().ok_or("Line 2 is missing.")?;
    lines.next().ok_or("Line 3 (value of seed) is missing.")?;
    lines.next().ok_or("Line 4 is missing.")?;
    let line5 = lines.next().ok_or("Line 5 (value of w) is missing.")?;
    let w = line5.parse::<usize>()?;
    lines.next().ok_or("Line 6 is missing.")?;
    let f2 = &Rc::new(F2::generate(()));
    let mut h = Mat::zero(Field::Some(f2), n / 2, n);
    for i in 0..n / 2 {
        let line = lines.next().ok_or("Missing line for definition of H.")?;
        let mut bits = line.chars().map(|c| c.to_digit(2).unwrap());
        for j in 0..n / 2 {
            h[(i, j)] = bits.next().ok_or("Missing bits for definition of H.")?;
        }
        for j in n / 2..n {
            h[(i, j)] = 0;
        }
        h[(i, n / 2 + i)] = 1;
    }
    let mut s = ColVec::zero(Field::Some(f2), n / 2);
    lines.next().ok_or("Missing line after definition of H.")?;
    let line = lines.next().ok_or("Missing line for definition of s.")?;
    let mut bits = line.chars().map(|c| c.to_digit(2).unwrap());
    for i in 0..n / 2 {
        s[i] = bits.next().ok_or("Missing bits for definition of s.")?;
    }
    Ok(Instance { n, w, h, s })
}

fn prange(instance: &Instance, max_tries: Option<usize>) -> Option<ColVec<F2>> {
    let (w, h, s) = (instance.w, &instance.h, &instance.s);
    let k = h.cols() - h.rows();
    let f2 = h.field();
    let mut tries = 0;
    loop {
        if let Some((u, _, p)) = h.random_standard_form() {
            let us = u * s;
            if us.weight() == w {
                let z = ColVec::zero(Field::Some(f2), k);
                let z_us = ColVec::from(Mat::vconcat(&z.into(), &us.into()));
                return Some(p * z_us);
            }
        }
        tries += 1;
        if let Some(max) = max_tries {
            if tries == max {
                return None;
            }
        }
    }
}

fn main() -> result::Result<(), MainError> {
    env_logger::init();
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        return Err("One argument expected.".into());
    }
    let instance = read_instance(&args[1])?;
    debug!(
        "n: {}  -  w: {}\n\n\
         h: {}\n\
         s: {}\n",
        instance.n, instance.w, instance.h, instance.s
    );
    let e = prange(&instance, None).unwrap();
    println!("{}", e.transpose());
    let ss = instance.h * e;
    println!("{}", ss);
    Ok(())
}
