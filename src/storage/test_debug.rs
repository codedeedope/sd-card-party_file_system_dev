use super::file_storage_debug::*;
use super::mbr_device_driver::*;
use super::block_device::*;
use super::fat32_device_driver::*;

use std::path::Path;
use std::char;
use std::string::*;

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

//use asserts instead of if ... {panic!(...)} !!!
//slices: Prefer over owned type if only immutable access is needed
//A concrete BlockDevice type for offset?
//check if compileable in microcontroller project!
//tuple ->which size??

// only useable with one file in the root directory, cant be searched; first will be used
pub fn test() {
    let fsd = FileStorageDebug::new(Path::new("storage.img"));
    let mbr_device_driver = MbrDeviceDriver::new(&fsd);
    let partition = mbr_device_driver.get_first_partition();
    //check if part_type is 0x0B ->all the stuff here in another file?
    let fat32_device_driver = Fat32DeviceDriver::new(&partition);
    //let root = fat32_device_driver.read_root_directory();
    let file = String::from_utf8(fat32_device_driver.read_first_file_to_vec());
    println!("{:?}", file);
    //prompt the file!! ->String.from_utf()
    //prompt metadate?!
}
