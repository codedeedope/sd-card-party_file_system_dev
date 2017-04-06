use super::storage_device::StorageDevice;

//offsets in bytes
const MBR_PARTITION_TABLE_OFFSET: usize = 0x01BE;
const PARTITION_TABLE_TYPE_OFFSET: usize = 0x04;

struct MbrDeviceDriver {
    mbr: Vec<u8>
}

pub fn new(mbr: Vec<u8>) {
    if mbr.len() != 512 {
        panic!("mbr not valid");
    } else {
        MbrDeviceDriver {
            mbr: mbr
        }
    }
}

mbrDeviceDriverd.get_first_partition_type(&self) ->u8 { //0bh FAT32 ->chs must be converted to lba
    //let part_type: u8 = self.mbr.
}

mbrDeviceDriver.get_first_partition_startsector_lba(&self) ->u32 { //sector of 512 bytes
}

mbrDeviceDriver.get_first_partition_number_of_sectors_lba(&self) ->u32 { //sector size in FAT can be different
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
    println!("char: {}", str); //dbg: not available on mc
    println!("val: {:x}", b); //:x -> hex
}
