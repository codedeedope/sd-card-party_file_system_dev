use super::block_device::BlockDevice;
use collections::vec::*;

pub struct File {
    //
}

impl File {
    pub fn new() -> File {
        File {}
    }
}

impl BlockDevice for File {
    fn read_blocks(&self, offset: usize, number: usize) -> Vec<u8> {
        Vec::new()
    }

    #[allow(unused_variables)]
    fn write_blocks(&self, offset: usize, blocks: Vec<u8>) -> Result<usize, ()> {
        unimplemented!();
    }

    fn number_of_blocks(&self) -> usize {
        42
    }

    fn block_size(&self) -> usize {
        42
    }
}
