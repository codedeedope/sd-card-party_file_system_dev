pub trait Storage {
    fn get_data(&self, offset: usize, number: usize) ->Vec<u8>;
    fn len(&self) ->usize;
}
