use super::block_device::BlockDevice;
use super::partition::Partition;

use collections::vec::Vec;

//offsets in bytes
const PARTITION_TABLE_OFFSET: usize = 0x01BE;

pub struct MbrDeviceDriver<'a> {
    block_device: &'a BlockDevice,
    mbr: Vec<u8>,
}

impl<'a> MbrDeviceDriver<'a> {
    pub fn new(block_device: &'a BlockDevice) -> MbrDeviceDriver<'a> {
        if !(block_device.block_size() >= 512 && block_device.block_size() % 512 == 0) {
            panic!("512");
        }
        MbrDeviceDriver {
            block_device: block_device,
            mbr: block_device.read_blocks(0, 1),
        }
    }

    // shouldn't be created every time
    pub fn get_first_partition(&self) -> Partition {
        let mut first_entry: Vec<u8> = Vec::with_capacity(16);
        for i in 0..16 {
            first_entry.push(self.mbr[PARTITION_TABLE_OFFSET + i]);
        }
        Partition::new(self.block_device, first_entry)
    }
}
