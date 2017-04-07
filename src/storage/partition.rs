use super::block_device::BlockDevice;

use collections::vec::Vec;

//sector here is just another name for block

const CHS_FIRST_SECTOR_OFFSET: usize = 0x01;
const TYPE_OFFSET: usize = 0x04;
const CHS_LAST_SECTOR_OFFSET: usize = 0x05;

pub struct Partition<'a> {
    block_device: &'a BlockDevice,
    partition_type: u8,
    start_sector_lba: usize,
    number_of_sectors_lba: usize,
}

impl<'a> Partition<'a> {
    pub fn new(block_device: &'a BlockDevice, partition: Vec<u8>) -> Partition<'a> {
        if partition.len() != 16 {
            panic!("16");
        }

        let partition_type = partition[TYPE_OFFSET];
        let start_sector_lba = encoded_chs_to_lba(partition[CHS_FIRST_SECTOR_OFFSET],
                                                  partition[CHS_FIRST_SECTOR_OFFSET + 1],
                                                  partition[CHS_FIRST_SECTOR_OFFSET + 2]);
        let last_sector_lba = encoded_chs_to_lba(partition[CHS_LAST_SECTOR_OFFSET],
                                                 partition[CHS_LAST_SECTOR_OFFSET + 1],
                                                 partition[CHS_LAST_SECTOR_OFFSET + 2]);
        let number_of_sectors_lba = last_sector_lba - start_sector_lba + 1;

        Partition {
            block_device: block_device,
            partition_type: partition_type,
            start_sector_lba: start_sector_lba,
            number_of_sectors_lba: number_of_sectors_lba,
        }
    }

    pub fn get_partition_type(&self) -> u8 {
        self.partition_type
    }
}

impl<'a> BlockDevice for Partition<'a> {
    fn read_blocks(&self, offset: usize, number: usize) -> Vec<u8> {
        self.block_device
            .read_blocks(self.start_sector_lba + offset, number)
    }

    #[allow(unused_variables)]
    fn write_blocks(&self, offset: usize, blocks: Vec<u8>) -> Result<usize, ()> {
        unimplemented!();
    }

    fn number_of_blocks(&self) -> usize {
        self.number_of_sectors_lba
    }

    fn block_size(&self) -> usize {
        self.block_device.block_size()
    }
}

fn encoded_chs_to_lba(first: u8, second: u8, third: u8) -> usize {
    let c: usize = (((second & 0b1100_0000_u8) as usize) << 2) | (third as usize);
    let h: usize = first as usize;
    let s: usize = (second & 0b0011_1111_u8) as usize;

    //LBA = ( (cylinder * heads_per_cylinder + heads ) * sectors_per_track ) + sector - 1
    //This allowed addressing 256 heads, 1024 cylinders per head and 64 sectors per cylinder
    //(In practice, the number 0 for each is not used.)
    (((c * 256 + h) + 64) + s - 1)
}
