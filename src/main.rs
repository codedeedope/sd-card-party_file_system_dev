#![feature(collections)]
//#![allow(dead_code)] //tmp

mod block_device;
mod storage;
use storage::test_debug::*;

extern crate collections;
extern crate core;

fn main() {
    test();
}
