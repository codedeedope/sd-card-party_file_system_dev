use super::file_storage_debug::*;
use super::mbr_device_driver::*;
use super::block_device::*;

use std::path::Path;
use std::char;

pub fn test() {
    let fsd = FileStorageDebug::new(Path::new("fat32.img"));
    println!("len: {}", fsd.len());
    let mbr = fsd.read_blocks(0, 1);
    let mbr_device_triver = MbrDeviceDriver::new(mbr);
    let pt: PartitionType = mbr_device_triver.get_first_partition_type();
    let fs: u32 = mbr_device_triver.get_first_partition_startsector_lba();
    let ns: u32 = mbr_device_triver.get_first_partition_number_of_sectors_lba();

    let opt: Option<char> = char::from_u32(pt as u32); //from collections?
    let ch = match opt {
        None => '$',
        Some(c) => c,
    };
    let x = ch as u32;
    println!("char: {}", x); //dbg: not available on mc
    println!("val: {:x}", x); //:x -> hex
}
