extern crate apodize;
extern crate hound;
extern crate image;
extern crate rustfft;
extern crate stats;

extern crate stopwatch;
use self::stopwatch::Stopwatch;

extern crate log4rs;
#[macro_use]
extern crate log;

use std::collections::HashMap;

extern crate multimap;

use multimap::MultiMap;

pub mod spectrogram;
pub mod hash;

use std::os::raw::c_char;
use std::ffi::{CStr, CString};

extern crate libc;
use libc::int32_t;

#[no_mangle]
pub extern "C" fn rust_greeting(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };

    CString::new("Hello".to_owned() + recipient)
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn rust_greeting_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

#[no_mangle]
pub extern "C" fn rust_compute_hashes(path: *const c_char) -> int32_t {
    0
}

#[no_mangle]
pub extern "C" fn rust_get_hashes_size() -> int32_t {
    4
}

#[no_mangle]
pub extern "C" fn rust_get_hashes() -> *mut c_char {
    let test = vec!["test1", "test2", "test3", "test4"];
    let transformed: Vec<*mut c_char> = test.iter()
        .map(|val| CString::new(*val).unwrap().into_raw())
        .collect();

    unsafe { transformed[0] }
}
