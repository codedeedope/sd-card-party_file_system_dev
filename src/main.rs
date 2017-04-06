mod storage_management;
mod storage_management_dbg;

use storage_management::*;

fn main() {
    mbr_fat32_reader::test();
}
