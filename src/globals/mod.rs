pub fn get_system_table() -> uefi_raw::table::system::SystemTable {
    unsafe {
        return uefi::table::system_table_raw().unwrap().read();
    }
}
