#![feature(collections)]
//#![allow(dead_code)] //tmp

mod storage;
use storage::test_debug::*;

extern crate collections;

/*
#![feature(core)]
#![feature(collections)]

extern crate collections;
use collections::vec::*;
*/

//use asserts instead of if ... {panic!(...)} !!!
//slices: Prefer over owned type if only immutable access is needed
//A concrete BlockDevice type for offset?
//check if compileable in microcontroller project!
//put file in root directory?

fn main() {
    test();
}
