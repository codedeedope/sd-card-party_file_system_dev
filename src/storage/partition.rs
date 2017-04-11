use super::block_device::BlockDevice;
use super::get_bytes::*;
use collections::vec::*;

//sector here is just another name for block

//const CHS_FIRST_SECTOR_OFFSET: usize = 0x01;
const TYPE_OFFSET: usize = 0x04;
//const CHS_LAST_SECTOR_OFFSET: usize = 0x05;
const LBA_FIRST_SECTOR_OFFSET: usize = 0x08; //partition_entry[0x0B] must be valid
const LBA_NUMBER_OF_SECTORS_OFFSET: usize = 0x0C;

pub struct Partition<'a> {
    block_device: &'a BlockDevice,
    partition_type: u8,
    start_sector_lba: usize,
    number_of_sectors_lba: usize,
}

impl<'a> Partition<'a> {
    // partition_entry could be a reference
    // note: start_sector_lba is the offset on block_device
    pub fn new(block_device: &'a BlockDevice, partition_entry: Vec<u8>) -> Partition<'a> {
        if partition_entry.len() != 16 {
            panic!("16");
        }

        Partition {
            block_device: block_device,
            partition_type: partition_entry[TYPE_OFFSET],
            start_sector_lba: four_bytes_at_offset(&partition_entry, LBA_FIRST_SECTOR_OFFSET) as
                              usize,
            number_of_sectors_lba: four_bytes_at_offset(&partition_entry,
                                                        LBA_NUMBER_OF_SECTORS_OFFSET) as
                                   usize,
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

/*
fn encoded_chs_to_lba(first: u8, second: u8, third: u8) -> usize {
    let c: usize = (((second & 0b1100_0000_u8) as usize) << 2) | (third as usize);
    let h: usize = first as usize;
    let s: usize = (second & 0b0011_1111_u8) as usize;

    //LBA = ( (cylinder * heads_per_cylinder + heads ) * sectors_per_track ) + sector - 1
    //This allowed addressing 256 heads, 1024 cylinders per head and 64 sectors per cylinder
    //(In practice, the number 0 for each is not used.)
    (((c * 255 + h) + 63) + s - 1) //wrong
}
*/
