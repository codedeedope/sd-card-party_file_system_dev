mod storage_management;
mod storage_management_dbg;

use storage_management::storage::*;
use storage_management_dbg::file_storage_debug::*;

fn main() {
    let mbr_partition_table_offset: usize = 0x01BE;
    let partition_table_type_offset: usize = 0x04;

    let fsd = FileStorageDebug::new();
    println!("size: {}", fsd.len());
    let dat = fsd.get_data(0, fsd.len());
    let b: u8 = dat[mbr_partition_table_offset + partition_table_type_offset];
    let opt: Option<char> = std::char::from_u32(b as u32);
    let ch = match opt {
        None => '$',
        Some(c) => c,
    };
    let mut str = String::new();
    str.push(ch);
    println!("char: {}", str);
    println!("val: {:x}", b); //:x -> hex
}
