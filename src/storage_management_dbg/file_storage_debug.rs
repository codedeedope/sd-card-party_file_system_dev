use std::fs::File;
use std::io::Read;
use storage_management::storage::*;

pub struct FileStorageDebug {
    file_buffer: Vec<u8>,
}

impl Storage for FileStorageDebug {
    fn get_data(&self, offset: usize, number: usize) ->Vec<u8> {
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

    fn len(&self) ->usize {
        self.file_buffer.len()
    }
} 

//#[allow(match_wild_err_arm)] //clippy
impl FileStorageDebug {
    pub fn new() ->FileStorageDebug
 {
        let mut buf = Vec::new();
        let mut file = match File::open("fat32.img") {
            Err(_) => panic!("file not present"),
            Ok(the_file) => the_file
        };
        match file.read_to_end(&mut buf) {
            Err(_) => println!("no data"),
            Ok(number) => println!("read {} bytes", number)
        }
        FileStorageDebug {
            file_buffer: buf
        }
    }
}
