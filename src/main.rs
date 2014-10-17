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
    run!(example50);
    run!(example60);
    run!(example70);
    run!(example80);
    run!(example90);
}
