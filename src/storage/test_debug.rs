use super::*;

fn test() {
    let fsd = FileStorageDebug::new();
    println!("len: {}", fsd.len());
    MbrDeviceDriver::new(fsd.read_blocks(0, 1));
    let pt: u8 mbrDeviceDriverd.get_first_partition_type();
    let fs: u32 mbrDeviceDriver.get_first_partition_startsector_lba();
    let ns: u32 mbrDeviceDriver.get_first_partition_number_of_sectors_lba();
}
