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
    run!(example010);
    // run!(example020); infinite loop
    run!(example025);
    run!(example030);
    run!(example040);
    run!(example050);
    run!(example060);
    run!(example070);
    run!(example080);
    run!(example090);
}
