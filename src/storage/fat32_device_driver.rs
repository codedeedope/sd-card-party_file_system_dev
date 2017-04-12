use super::block_device::BlockDevice;
use super::directory_entry::DirectoryEntry;
use super::get_bytes::*;
use collections::vec::*;

//number of used bytes?!!
const BYTE_PER_SECTOR_OFFSET: usize = 0x0B;
const SECTORS_PER_CLUSTER_OFFSET: usize = 0x0D;
const NUMBER_OF_RESERVED_SECTORS_OFFSET: usize = 0x0E;
const NUMBER_OF_FATS_OFFSET: usize = 0x010;
const NUMBER_OF_SECTORS_PER_FAT_OFFSET: usize = 0x024;
const CLUSTER_NUMBER_ROOT_DIRECTORY_OFFSET: usize = 0x02C;

/*
dbg:
in the file:
2048 :partition
(2048 + 32) * 512 = 1064960 :FAT (table-entries)
(2048 + 4022) * 512 = 3107840 :first cluster (data)
*/

/*
cluster chain:
next cluster:
0x?0000002 - 0x?FFFFFEF
not: end of cluster-chain (not standard)
*/

//(not implemented yet) Searches for directory-entry of name <>,
//put the clusters together and returns a "DirectoryEntry"
//--will be hardcoded first
//can be a file or a directory and can be free or not

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
        let number_of_reserved_sectors =
            two_bytes_at_offset(&block, NUMBER_OF_RESERVED_SECTORS_OFFSET) as usize;
        let cluster_number_root_directory =
            four_bytes_at_offset(&block, CLUSTER_NUMBER_ROOT_DIRECTORY_OFFSET) as usize;

        if !(byte_per_sector % 512 == 0) {
            panic!("wrong byte_per_sector");
        }
        let block_size_sector = byte_per_sector / block_device.block_size();
        let block_size_cluster = sectors_per_cluster * block_size_sector;
        let number_of_reserved_blocks = number_of_reserved_sectors / block_size_sector;
        let number_of_fats = block[NUMBER_OF_FATS_OFFSET] as usize;
        let number_of_sectors_per_fat =
            four_bytes_at_offset(&block, NUMBER_OF_SECTORS_PER_FAT_OFFSET) as usize;
        let data_region_block_offset = number_of_fats * number_of_sectors_per_fat *
                                       block_size_sector +
                                       number_of_reserved_blocks;
        let root_directory_cluster_offset = cluster_number_root_directory;

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

    /// dbg test
    /// only one file in root
    /// only one cluster per file
    pub fn read_first_file_to_vec(&self) -> Vec<u8> {
        let file = self.first_file_directory_entry();
        //let mut full = self.read_cluster_data_region(file.first_cluster());
        let mut full = self.compile_clusters_begin_with_number(file.first_cluster());
        println!("0: {:0}", file.first_cluster());
        println!("1: {:0}", file.file_size());
        println!("2: {:0}", full.len());
        full.split_off(file.file_size());
        full
    }

    fn compile_clusters_begin_with_number(&self, offset: usize) ->Vec<u8> {
        let mut all = Vec::new();
        let mut current_offset = offset;
        //[0x?0000002; 0x?FFFFFF6] //max should be checked
        while (current_offset & 0x0FFFFFFF) >= 0x2 && (current_offset & 0x0FFFFFFF) <= 0xFFFFFF6 {
            all.append(&mut self.read_cluster_data_region(current_offset));
            current_offset = self.read_in_fat(current_offset);
        }
        all
    }

    fn first_file_directory_entry(&self) -> DirectoryEntry {
        let root = self.read_root_directory();
        let number: usize = root.len() / 32;
        println!("number: {:?}", number);
        for i in 0..number {
            println!("i: {:?}", i);
            let mut directory_entry = Vec::with_capacity(32);
            for j in 0..32 {
                directory_entry.push(root[i * 32 + j]);
            }
            let dir_entr = DirectoryEntry::new(&directory_entry);
            if dir_entr.is_file() {
                return dir_entr;
            }
        }
        panic!("no file here");
    }

    pub fn read_root_directory(&self) -> Vec<u8> {
        self.read_cluster_data_region(self.root_directory_cluster_offset)
    }

    /// entry in fat -> traversion not implemented yet
    fn read_in_fat(&self, offset: usize) -> usize {
        //buffer for read block_device with offset
        let block = self.block_device
            .read_blocks(self.number_of_reserved_blocks, 1);
        //4: byte-size of u32
        (four_bytes_at_offset(&block, offset * 4) & 0x0FFFFFFF) as usize
    }

    fn read_cluster_data_region(&self, cluster_entry_offset: usize) -> Vec<u8> {
        //- 2 because the first two cluster-entries in the FAT are reserved
        //and dont represent clusters in the data section
        self.block_device
            .read_blocks(self.data_region_block_offset +
                         (cluster_entry_offset - 2) * self.block_size_cluster,
                         self.block_size_cluster)
    }
}
