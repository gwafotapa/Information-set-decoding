use log::debug;
use main_error::MainError;
use std::env;

use syndrome_decoding_problem::isd::{self, Instance};

// type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> std::result::Result<(), MainError> {
    env_logger::init();
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        return Err("One argument expected.".into());
    }
    let inst = Instance::read_instance(&args[1])?;
    debug!(
        "n: {}  -  w: {}\n\n\
         h: {}\n\
         s: {}\n",
        inst.n(),
        inst.w(),
        inst.h(),
        inst.s()
    );
    let e = isd::prange(&inst, None).unwrap();
    for i in 0..e.rows() {
        print!("{}", e[i]);
    }
    println!();
    assert_eq!(inst.h() * e, *inst.s());
    Ok(())
}
