pub trait BlockDevice {
    /// the size of one block is 512 byte
    /// offset: location of the first block to be read
    /// if the offset *512 is larger than the last address, get_data shall return an empty vector
    /// if number == 0, get_data shalll return an empty vector
    /// if (offset + number) *512 is larger than the last address,
    /// get_data shall return an vector with all bytes till the end
    /// read_blocks(...).len() % 512 == 0 must always be true
    /// implementation could use Vec::reserve_exact(...)
    fn read_blocks(&self, offset: usize, number: usize) ->Vec<u8>;
}
