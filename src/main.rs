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

fn main() {
    test();
}
