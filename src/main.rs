#![feature(collections)]

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
//test mc code language core features with mc project
//clippy!!!

fn main() {
    test();
}
