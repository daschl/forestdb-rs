extern crate libc;

use libc::uint16_t;

#[repr(C)]
#[derive(Debug)]
pub struct fdb_config {
	chunksize: uint16_t
}

extern {
	pub fn fdb_get_default_config() -> fdb_config;
}

#[test]
fn foo() {
	unsafe {
		println!("{:?}", fdb_get_default_config());
	}
}