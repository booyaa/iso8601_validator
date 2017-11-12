#[cfg(test)]
mod tests {
    #[test]
    fn should_fail_to_validate() {
        let expected = false;
        let actual = super::rust_validate("ppp");

        assert_eq!(expected, actual);
    }

    #[test]
    fn should_validate() {
        let expected = true;
        let actual = super::rust_validate("20060831T16:44+00:00");

        assert_eq!(expected, actual);
    }
}

extern crate iso8601;
extern crate libc;
use libc::*;
use std::ffi::CStr;
use std::str;

use iso8601::parsers::*;

#[no_mangle]
pub extern fn validate(s: *const c_char) -> boolean_t {
    let c_str = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };

    let r_str = c_str.to_str().unwrap();
    if rust_validate(r_str) { 1 } else { 0 }
}

pub fn rust_validate(input : &str) ->  bool {
    let result = parse_datetime(input.as_bytes());
    ! (result.is_err() || result.is_incomplete())
}