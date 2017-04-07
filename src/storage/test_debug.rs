use super::file_storage_debug::*;
use super::mbr_device_driver::*;
//use super::block_device::*;

use std::path::Path;
use std::char;

/*
pub enum PartitionType {
    INVALID,
    FAT32 = 0x0B,
}

    //0x0B FAT32 ->chs must be converted to lba
    pub fn get_first_partition_type(&self) ->PartitionType {
        const PT_FAT32: u8 = (PartitionType::FAT32) as u8;
        match self.mbr[MBR_PARTITION_TABLE_OFFSET + PARTITION_TABLE_TYPE_OFFSET] {
        PT_FAT32 => PartitionType::FAT32,
            _ => PartitionType::INVALID,
        }
    }
*/

pub fn test() {
    let fsd = FileStorageDebug::new(Path::new("fat32.img"));
    let mbr_device_driver = MbrDeviceDriver::new(&fsd);
    //let pt: u8 = mbr_device_triver.get_first_partition_type();
    //let fs: usize = mbr_device_triver.get_first_partition_startsector_lba();
    //let ns: usize = mbr_device_triver.get_first_partition_number_of_sectors_lba();

    let pt = mbr_device_driver.get_first_partition().get_partition_type();

    let opt: Option<char> = char::from_u32(pt as u32); //from collections?
    let ch = match opt {
        None => '$',
        Some(c) => c,
    };
    let x = ch as u32;
    println!("val: {:x}", x);
}
