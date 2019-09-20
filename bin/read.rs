use ase;
use std::{env, fs};

fn main() {
    let fname = env::args().nth(1).unwrap();
    let mut file = fs::File::open(fname).unwrap();
    let ase = ase::Aseprite::from_read(&mut file).unwrap();
    println!("{:#?}", ase);
}
