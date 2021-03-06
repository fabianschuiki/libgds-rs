// Copyright (c) 2016 Fabian Schuiki

//! This module implements a wrapper around `gds_reader_t`.

use std::ptr;
use std::ffi::CString;
use libc::{c_char, c_void, c_int};


#[link(name = "gds")]
extern {
	fn gds_reader_open_file(rd: *mut *mut c_void, file: *const c_char, flags: c_int) -> c_int;
	fn gds_reader_close(rd: *mut c_void);
}


/// A struct that reads records from a GDS stream.
pub struct Reader {
	pub opaque: *mut c_void,
}

impl Reader {
	/// Open a GDS file for reading.
	pub fn open_file(path: &str, flags: i32) -> Result<Reader, ()> {
		let mut p: *mut c_void = ptr::null_mut();
		let path_cstr = CString::new(path).unwrap();
		let fr = unsafe {
			gds_reader_open_file(&mut p, path_cstr.as_ptr(), flags)
		};
		if fr == 0 {
			assert!(!p.is_null());
			Ok(Reader { opaque: p })
		} else {
			Err(())
		}
	}
}

impl Drop for Reader {
	fn drop(&mut self) {
		unsafe {
			gds_reader_close(self.opaque);
		}
	}
}
