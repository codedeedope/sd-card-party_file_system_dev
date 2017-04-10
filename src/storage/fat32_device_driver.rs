use super::block_device::BlockDevice;
use super::file::File;

const BYTE_PER_SECTOR_OFFSET: usize = 0x0B;
const SECTORS_PER_CLUSTER_OFFSET: usize = 0x0D;
const NUMBER_OF_RESERVED_SECTORS_OFFSET: usize = 0x0E;
const NUMBER_OF_SECTORS_OFFSET: usize = 0x13;

//bootsector
pub struct FatMeta {
    byte_per_sector: u16,
    sectors_per_cluster: u8,
    number_of_reserved_sectors: u16,
    number_of_sectors: u16, //0 not supproted ->doc!
}

pub struct Fat32DeviceDriver<'a> {
    block_device: &'a BlockDevice,
}

impl<'a> Fat32DeviceDriver<'a> {
    /// Partition::get_partition_type() == 0x0B has to be checked bevore
    pub fn new(block_device: &'a BlockDevice) -> Fat32DeviceDriver<'a> {
        if !(block_device.block_size() >= 512 && block_device.block_size() % 512 == 0) {
            panic!("512");
        }
        Fat32DeviceDriver {
            block_device: block_device,
        }
    }
}
