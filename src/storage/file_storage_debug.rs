use super::storage_device::StorageDevice;

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct FileStorageDebug {
    file_buffer: Vec<u8>,
}

impl StorageDevice for FileStorageDebug {
    fn read(&self, offset: usize, number: usize) ->Vec<u8> {
        if self.file_buffer.len() >= offset + number {
            let mut buf: Vec<u8> = Vec::new();
            buf.reserve(number);
            for b in self.file_buffer[offset..(offset + number)].iter() {
                buf.push(*b);
            }
            buf
        } else {
            Vec::new()
        }
    }
} 

//#[allow(match_wild_err_arm)] //clippy
impl FileStorageDebug {
    pub fn new(path: &Path) ->FileStorageDebug {
        let mut file = match File::open(path) {
            Err(_) => panic!("file not present"),
            Ok(the_file) => the_file
        };
        let mut buf = Vec::new();
        match file.read_to_end(&mut buf) {
            Err(_) => println!("no data"),
            Ok(number) => println!("{} bytes read", number)
        }
        FileStorageDebug {
            file_buffer: buf
        }
    }

    pub fn len() ->usize {
        file_buffer.len()
    }
}
