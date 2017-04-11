use super::block_device::BlockDevice;
use super::file::File;
use super::get_bytes::*;
use collections::vec::*;

//number of used bytes?!!
const BYTE_PER_SECTOR_OFFSET: usize = 0x0B;
const SECTORS_PER_CLUSTER_OFFSET: usize = 0x0D;
const NUMBER_OF_RESERVED_SECTORS_OFFSET: usize = 0x0E;
const NUMBER_OF_FATS_OFFSET: usize = 0x010;
const NUMBER_OF_SECTORS_PER_FAT_OFFSET: usize = 0x024; //4
const CLUSTER_NUMBER_ROOT_DIRECTORY_OFFSET: usize = 0x02C;

/*
cluster chain:
next cluster:
0x?0000002 - 0x?FFFFFEF
not: end of cluster-chain
*/

pub struct Fat32DeviceDriver<'a> {
    block_device: &'a BlockDevice,
    block_size_sector: usize,
    block_size_cluster: usize,
    number_of_reserved_blocks: usize,
    number_of_fats: usize,
    number_of_sectors_per_fat: usize,
    data_region_block_offset: usize,
    root_directory_cluster_offset: usize,
}

impl<'a> Fat32DeviceDriver<'a> {
    /// Partition::get_partition_type() == 0x0B has to be checked bevore
    pub fn new(block_device: &'a BlockDevice) -> Fat32DeviceDriver<'a> {
        if !(block_device.block_size() >= 512 && block_device.block_size() % 512 == 0) {
            panic!("wrong block_size()");
        }
        let block = block_device.read_blocks(0, 1);

        let byte_per_sector = two_bytes_at_offset(&block, BYTE_PER_SECTOR_OFFSET) as usize;
        let sectors_per_cluster = block[SECTORS_PER_CLUSTER_OFFSET] as usize;
        let number_of_reserved_sectors = two_bytes_at_offset(&block, NUMBER_OF_RESERVED_SECTORS_OFFSET) as usize;
        let cluster_number_root_directory = four_bytes_at_offset(&block, CLUSTER_NUMBER_ROOT_DIRECTORY_OFFSET) as usize;

        if !(byte_per_sector % 512 == 0) {
            panic!("wrong byte_per_sector");
        }
        let block_size_sector = byte_per_sector / block_device.block_size();
        let block_size_cluster = sectors_per_cluster * block_size_sector;
        let number_of_reserved_blocks = number_of_reserved_sectors / block_size_sector;
        let number_of_fats = block[NUMBER_OF_FATS_OFFSET] as usize;
        let number_of_sectors_per_fat = four_bytes_at_offset(&block, NUMBER_OF_SECTORS_PER_FAT_OFFSET) as usize;
        let data_region_block_offset = number_of_fats * number_of_sectors_per_fat * block_size_sector + number_of_reserved_blocks;
        //number_of_sector_per_fat ->FatSz
        //read in fat: block_offset * 4 ->4 * n:FATOffset
        //BPB_RsvdSecCnt ->number_of_reserved_sectors
        //ThisFATSecNum = number_of_reserved_sectors + ((offset * 4) / bytes_per_sector)
        //ThisFATEntOffset = (n * 4) % bytes_per_sector
        //entry extraction: FAT32ClusEntryVal = (*((DWORD *) &SecBuff[ThisFATEntOffset])) & 0x0FFFFFFF;
        println!("dat reg: {:?}", number_of_sectors_per_fat);
        let root_directory_cluster_offset = cluster_number_root_directory;
        println!("{:?}", root_directory_cluster_offset);
        println!("number_of_reserved_blocks: {:?}", number_of_reserved_blocks);
        Fat32DeviceDriver {
            block_device: block_device,
            block_size_sector: block_size_sector,
            block_size_cluster: block_size_cluster,
            number_of_reserved_blocks: number_of_reserved_blocks,
            number_of_fats: number_of_fats,
            number_of_sectors_per_fat: number_of_sectors_per_fat,
            data_region_block_offset: data_region_block_offset,
            root_directory_cluster_offset: root_directory_cluster_offset,
        }
    }

    /// unfinished
    pub fn read_root_directory(&self) -> Vec<u8> {
        self.read_cluster_data_region(self.root_directory_cluster_offset)
    }

    /// entry in fat -> traversion not implemented yet
    fn read_in_fat(&self, offset: usize) ->usize {
        //buffer for read block_device with offset
        let block = self.block_device.read_blocks(self.number_of_reserved_blocks, 1);
        //4: byte-size of u32
        four_bytes_at_offset(&block, offset * 4) as usize
    }

    /// in data region
    fn read_cluster_data_region(&self, offset: usize) ->Vec<u8> {
        //root directory is in data section
        println!("::: {:?}", self.data_region_block_offset);
        self.block_device.read_blocks(self.data_region_block_offset + offset * self.block_size_cluster, self.block_size_cluster)
    }
}
