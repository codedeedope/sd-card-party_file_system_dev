use super::file_storage_debug::*;
use super::mbr_device_driver::*;
use super::block_device::*;
use super::fat32_device_driver::*;

use std::path::Path;
use std::char;
use std::string::*;

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

    if partition.get_partition_type() != 0x0B {
        panic!("not FAT32");
    }

    let fat32_device_driver = Fat32DeviceDriver::new(&partition);
    let file = String::from_utf8(fat32_device_driver.read_first_file_to_vec()).unwrap();
    println!("{:?}", file);
    //prompt metadata?!
}
