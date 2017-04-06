use super::storage::*;
use storage_management_dbg::file_storage_debug::*;

use std::char;

const MBR_PARTITION_TABLE_OFFSET: usize = 0x01BE;
const PARTITION_TABLE_TYPE_OFFSET: usize = 0x04;

//pub fn test(storage: Storage) { ->later! ; //maybe storage buffer between
pub fn test() {
    check_partition_type();
}

fn check_partition_type() {
    let fsd = FileStorageDebug::new();
    println!("size: {}", fsd.len());
    let dat = fsd.get_data(0, fsd.len());
    let b: u8 = dat[MBR_PARTITION_TABLE_OFFSET + PARTITION_TABLE_TYPE_OFFSET];
    let opt: Option<char> = char::from_u32(b as u32);
    let ch = match opt {
        None => '$',
        Some(c) => c,
    };
    let mut str = String::new();
    str.push(ch);
    println!("char: {}", str);
    println!("val: {:x}", b); //:x -> hex
}