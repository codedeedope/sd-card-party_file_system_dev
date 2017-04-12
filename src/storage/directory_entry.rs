use super::get_bytes::*;
use collections::vec::*;
//use collections::string::*;

// offset and number could be a tuple
const NAME_OFFSET: usize = 0; //11
const ATTRIBUTE_OFFSET: usize = 11; //1
const FIRST_CLUSTER_HIGH_OFFSET: usize = 20; //2
const FIRST_CLUSTER_LOW_OFFSET: usize = 26; //2
const FILE_SIZE_OFFSET: usize = 28; //4

//File [sic!] cant be a proper BlockDevice yet ->see BlockDevice comments
//  should be a handle, that knows the mbr driver (?)
/// just a simple container
/// can represent a file or a directory
/// unfinished!!!
pub struct DirectoryEntry {
    is_file: bool,
    first_cluster_entry_number: usize,
    file_size: usize,
}

impl DirectoryEntry {
    pub fn new(directory_entry: &Vec<u8>) -> DirectoryEntry {
        if directory_entry.len() != 32 {
            panic!("32");
        }

        /*
        println!("dir len: {:?}", directory_entry.len());
        for n in directory_entry {
            print!("{0:02.x}", n); // !!!
        }
        println!("");
        */

        let high = two_bytes_at_offset(&directory_entry, FIRST_CLUSTER_HIGH_OFFSET) as u32;
        let low = two_bytes_at_offset(&directory_entry, FIRST_CLUSTER_LOW_OFFSET) as u32;
        let first_cluster_entry_number = (high << 16 | low) as usize;

        let mut is_file = true;
        let attr = directory_entry[ATTRIBUTE_OFFSET];
        //println!("attr: {:x}", attr);
        let no_name = directory_entry[NAME_OFFSET];
        //println!("no_name:{:x}", no_name);

        let mut is_volume_id = false;
        let mut is_directory = false;

        if attr & 0x08 != 0 {
            is_volume_id = true;
        }

        if attr & 0x10 != 0 {
            is_directory = true;
        }

        if is_volume_id || is_directory {
            is_file = false;
        } else if no_name == 0xE5 || no_name == 0 {
            is_file = false;
        }

        let file_size = four_bytes_at_offset(&directory_entry, FILE_SIZE_OFFSET) as usize;

        DirectoryEntry {
            is_file: is_file,
            first_cluster_entry_number: first_cluster_entry_number,
            file_size: file_size,
        }
    }

    pub fn first_cluster(&self) -> usize {
        self.first_cluster_entry_number
    }

    pub fn is_file(&self) -> bool {
        self.is_file
    }

    pub fn file_size(&self) -> usize {
        self.file_size
    }
}
