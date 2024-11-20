use std::ffi::CStr;

use libc::c_char;

#[cfg(test)]
mod test;

extern "C" {
    fn c_test() -> *const c_char;
}

#[allow(dead_code)]
fn call_test() -> &'static str {
    unsafe { CStr::from_ptr(c_test()) }.to_str().unwrap()
}
