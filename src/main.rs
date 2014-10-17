#![feature(macro_rules)]

extern crate "nmatsakis-examples" as nmatsakis;

macro_rules! run {
    ($i:ident) => {
        {
            println!("nmatsakis::{}", stringify!($i));
            nmatsakis::$i::main();
        }
    }
}

pub fn main() {
    run!(example10);
    // run!(example20); infinite loop
    run!(example30);
    run!(example40);
}
