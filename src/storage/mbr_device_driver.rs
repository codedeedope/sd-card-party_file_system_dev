use super::block_device::BlockDevice;

//offsets in bytes
const MBR_PARTITION_TABLE_OFFSET: usize = 0x01BE;
const PARTITION_TABLE_CHS_FIRST_SECTOR_OFFSET: usize = 0x01;
const PARTITION_TABLE_TYPE_OFFSET: usize = 0x04;
const PARTITION_TABLE_CHS_LAST_SECTOR_OFFSET: usize = 0x05;

pub enum PartitionType {
    INVALID,
    FAT32 = 0x0B,
}

pub struct MbrDeviceDriver {
    mbr: Vec<u8>
}

impl MbrDeviceDriver {
    pub fn new(mbr: Vec<u8>) -> MbrDeviceDriver { //better: reference?!!!!
        if mbr.len() != 512 {
            panic!("mbr not valid");
        } else {
            MbrDeviceDriver {
                mbr: mbr
            }
        }
    }

    //0x0B FAT32 ->chs must be converted to lba
    pub fn get_first_partition_type(&self) ->PartitionType {
        let fat32 = (PartitionType::FAT32) as u8;
        match self.mbr[MBR_PARTITION_TABLE_OFFSET + PARTITION_TABLE_TYPE_OFFSET] {
            fat32 => PartitionType::FAT32,
            _ => PartitionType::INVALID,
        }
    }

    //sector of 512 bytes
        pub fn get_first_partition_startsector_lba(&self) ->u32 {
        let first = self.mbr[MBR_PARTITION_TABLE_OFFSET + PARTITION_TABLE_CHS_FIRST_SECTOR_OFFSET];
        let second = self.mbr[MBR_PARTITION_TABLE_OFFSET + PARTITION_TABLE_CHS_FIRST_SECTOR_OFFSET + 1];
        let third = self.mbr[MBR_PARTITION_TABLE_OFFSET + PARTITION_TABLE_CHS_FIRST_SECTOR_OFFSET + 2];

        encoded_chs_to_lba(first, second, third)
    }

    //sector size in FAT can be different
    pub fn get_first_partition_number_of_sectors_lba(&self) ->u32 {
        let first = self.mbr[MBR_PARTITION_TABLE_OFFSET + PARTITION_TABLE_CHS_LAST_SECTOR_OFFSET];
        let second = self.mbr[MBR_PARTITION_TABLE_OFFSET + PARTITION_TABLE_CHS_LAST_SECTOR_OFFSET + 1];
        let third = self.mbr[MBR_PARTITION_TABLE_OFFSET + PARTITION_TABLE_CHS_LAST_SECTOR_OFFSET + 2];

        self.get_first_partition_startsector_lba() - encoded_chs_to_lba(first, second, third) + 1
    }
}

fn encoded_chs_to_lba(first: u8, second: u8, third: u8) -> u32 {
    let c: u32 = (((second >> 6) as u32) << 8) | (third as u32);
    let h: u32 = first as u32;
    let s: u32 =  (second & 0b0011_1111_u8) as u32;

    //LBA = ( (cylinder * heads_per_cylinder + heads ) * sectors_per_track ) + sector - 1
    //This allowed addressing 256 heads, 1024 cylinders per head and 64 sectors per cylinder
    //(In practice, the number 0 for each is not used.) 
    (((c * 256 + h) + 64) + s - 1)
}
