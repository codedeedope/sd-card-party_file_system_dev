use super::block_device::BlockDevice;
use super::file::File;
use super::get_bytes::*;
use collections::vec::*;

const BYTE_PER_SECTOR_OFFSET: usize = 0x0B;
const SECTORS_PER_CLUSTER_OFFSET: usize = 0x0D;
const NUMBER_OF_RESERVED_SECTORS_OFFSET: usize = 0x0E;
//const NUMBER_OF_SECTORS_OFFSET: usize = 0x13;
const CLUSTER_NUMBER_ROOT_DIRECTORY_OFFSET: usize = 0x02C;

/*
cluster chain:
next cluster:
0x?0000002 - 0x?FFFFFEF
not: end of cluster-chain
*/

//bootsector
struct FatMeta {
    byte_per_sector: u16, //block: %512
    sectors_per_cluster: u8,
    number_of_reserved_sectors: u16,
//    number_of_sectors: u16, //0 not supproted ->doc!
    cluster_number_root_directory: u32,
}

pub struct Fat32DeviceDriver<'a> {
    block_device: &'a BlockDevice,
    block_size_sector: usize,
    block_size_cluster: usize,
    number_of_reserved_blocks: usize,
    root_directory_cluster_offset: usize,
}

impl<'a> Fat32DeviceDriver<'a> {
    /// Partition::get_partition_type() == 0x0B has to be checked bevore
    pub fn new(block_device: &'a BlockDevice) -> Fat32DeviceDriver<'a> {
        if !(block_device.block_size() >= 512 && block_device.block_size() % 512 == 0) {
            panic!("wrong block_size()");
        }
        let block = block_device.read_blocks(0, 1);
        let fat_meta = FatMeta {
            byte_per_sector: two_bytes_at_offset(&block, BYTE_PER_SECTOR_OFFSET),
            sectors_per_cluster: block[SECTORS_PER_CLUSTER_OFFSET],
            number_of_reserved_sectors: two_bytes_at_offset(&block, NUMBER_OF_RESERVED_SECTORS_OFFSET),
            //number_of_sectors: two_bytes_at_offset(&block, NUMBER_OF_SECTORS_OFFSET),
            cluster_number_root_directory: four_bytes_at_offset(&block, CLUSTER_NUMBER_ROOT_DIRECTORY_OFFSET),
        };
        if !(fat_meta.byte_per_sector % 512 == 0) {
            panic!("wrong byte_per_sector");
        }
        let block_size_sector = fat_meta.byte_per_sector as usize / block_device.block_size();
        let block_size_cluster = fat_meta.sectors_per_cluster as usize * block_size_sector;
        let number_of_reserved_blocks = fat_meta.number_of_reserved_sectors as usize / block_size_sector;
        let root_directory_cluster_offset = fat_meta.cluster_number_root_directory as usize;
        println!("number_of_reserved_blocks: {:?}", number_of_reserved_blocks);
        Fat32DeviceDriver {
            block_device: block_device,
            block_size_sector: block_size_sector,
            block_size_cluster: block_size_cluster,
            number_of_reserved_blocks: number_of_reserved_blocks,
            root_directory_cluster_offset: root_directory_cluster_offset,
        }
    }

    pub fn read_root_directory(&self) -> Vec<u8> {
        self.read_cluster(self.root_directory_cluster_offset)
    }

    fn read_cluster(&self, offset: usize) ->Vec<u8> {
        //root directory is in data section
        println!("{:?}", self.number_of_reserved_blocks + offset * self.block_size_cluster);
        self.block_device.read_blocks(self.number_of_reserved_blocks + offset * self.block_size_cluster, self.block_size_cluster)
    }
}
