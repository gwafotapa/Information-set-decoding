use main_error::MainError;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
    result,
};

use mceliece::{finite_field::Field, finite_field::FieldTrait, finite_field::F2, matrix::Mat};

// type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn read_instance_params(
    file_name: &str,
) -> result::Result<(usize, Mat<F2>, usize, Mat<F2>), Box<dyn Error>> {
    let f = File::open(file_name)?;
    let f = BufReader::new(f);
    let mut lines = f.lines().map(|l| l.unwrap());
    lines.next().ok_or("Line 0 is missing.")?;
    let line1 = lines.next().ok_or("Line 1 is missing.")?;
    let n = line1.parse::<usize>()?;
    lines.next().ok_or("Line 2 is missing.")?;
    lines.next().ok_or("Line 3 is missing.")?;
    lines.next().ok_or("Line 4 is missing.")?;
    let line5 = lines.next().ok_or("Line 5 is missing.")?;
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
    let mut s = Mat::zero(Field::Some(f2), n / 2, 1);
    lines.next().ok_or("Missing line after definition of H.")?;
    let line = lines.next().ok_or("Missing line for definition of s.")?;
    let mut bits = line.chars().map(|c| c.to_digit(2).unwrap());
    for i in 0..n / 2 {
        s[(i, 0)] = bits.next().ok_or("Missing bits for definition of s.")?;
    }
    Ok((n, h, w, s))
}

fn main() -> result::Result<(), MainError> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        return Err("One argument expected.".into());
    }
    let (n, h, w, s) = read_instance_params(&args[1])?;
    println!(
        "n: {}\n\
         h: {}\n\
         w: {}\n\
         s: {}\n",
        n, h, w, s
    );
    Ok(())
}
