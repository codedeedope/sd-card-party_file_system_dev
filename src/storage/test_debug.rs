use super::*;

fn test() {
    let fsd = FileStorageDebug::new();
    println!("len: {}", fsd.len());
    let mbr = fsd.read_blocks(0, 1);
    let mbr_device_triver = MbrDeviceDriver::new(mbr;
    let pt: PartitionType mbrDeviceDriver.get_first_partition_type();
    let fs: u32 mbrDeviceDriver.get_first_partition_startsector_lba();
    let ns: u32 mbrDeviceDriver.get_first_partition_number_of_sectors_lba();

    let opt: Option<char> = char::from_u32(pt as u32);
    let ch = match opt {
        None => '$',
        Some(c) => c,
    };
    let mut str = String::new();
    str.push(ch);
    println!("char: {}", str); //dbg: not available on mc
    println!("val: {:x}", b); //:x -> hex
}
