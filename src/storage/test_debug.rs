use super::file_storage_debug::*;
use super::mbr_device_driver::*;
use super::block_device::*;
use super::fat32_device_driver::*;

use std::path::Path;
use std::string::*;

//unsafe code coulf increase speed at some points
//use asserts instead of if ... {panic!(...)} !!!
//slices: Prefer over owned type if only immutable access is needed
//A concrete BlockDevice type for offset?
//check if compileable in microcontroller project!
//tuple ->which size??
//optimize memory usage ->microcontroller doesnt have that much

pub fn test() {
    let fsd = FileStorageDebug::new(Path::new("storage.img"));
    let mbr_device_driver = MbrDeviceDriver::new(&fsd);
    let partition = mbr_device_driver.get_first_partition();

    if partition.get_partition_type() != 0x0B {
        panic!("not FAT32");
    }

    let fat32_device_driver = Fat32DeviceDriver::new(&partition);
    let file_vec = fat32_device_driver.read_file_to_vec("tst.txt");
    if file_vec.is_some() {
        let file = String::from_utf8(file_vec.unwrap()).unwrap();
        //println!("{:?}", file);
    } else {
        println!("file not found");
    }

    //push to main repository
    //prompt metadata?!
    //check warnings
}
