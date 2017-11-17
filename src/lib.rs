#[cfg(test)]
mod tests { // 1 - Always be testing
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

    // URLO: Should I have also tested the actual FFI function?
}

// 2 - After we've added the dependency in our Cargo.toml we need to reference 
// the crate 
extern crate iso8601;

// 3 - We need this crate to allow us to FFI 
extern crate libc; 

// 4 - Ah finally something that looks familar to perl hackers! This does the 
// same thing, introduces the module namespace so we can use it.
use libc::*;

// 5 - Needed for handling a C pointer, which is how our string from perl will
// arrive as.
use std::ffi::CStr;
use std::str;

// 6 - The crate we wish to wrap in FFI
use iso8601::parsers::*;


// 7 - This is our FFI wrapped function, the types you see here are C 
// representations. 
//
// no_mangle is to stop the compiler from jumbling the name during compilation. 
// We'll need this to reference within perl.
#[no_mangle]
pub extern fn validate(s: *const c_char) -> boolean_t {
    // 8 - Remember we spoke about the safety aspects of Rust at the beginning 
    // of the talk? 
    // 
    // Occassionally we need to do things that the compiler won't let us do, 
    // deferencing pointers are one of these things. The compiler can't make
    // the safety guarentees.
    //
    // There's a risk that we could get a null pointer when deferencing it.
    //
    // So when the compiler can't do the checking, we use unsafe to take over
    // and do the check manually.
    let c_str = unsafe {
        // 9 - And here's how we avoid harm... the assert will cause our library
        // to panic (exit with error).
        assert!(!s.is_null());

        // 10 - having tested for a null pointer we can safely return C string
        // make sure you point out the three reasons why this call is "unsafe"
        // https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.from_ptr
        CStr::from_ptr(s)
    };

    // 11 - there's several things going on here (L-R)
    // a) The CStr::from_ptr(s) - casts a raw C string to a safe C string 
    // wrapper
    // b) to_str() - turns this into a Result type which may contain a string 
    // slice or an error
    // c) unwrap() - takes the Result and either returns a value or causes a 
    // a panic. 
    // A minor note: unwrapping in a library is considered bad form. It's better
    // to check for an error and handle accordingly.
    let r_str = c_str.to_str().unwrap();

    // 12 - pass the string slice to the rust function that will do the 
    // validation
    if rust_validate(r_str) { 1 } else { 0 }
}

pub fn rust_validate(input : &str) ->  bool {
    // 13 - the parse_datetime function expects a byte slice rather than string
    // slice, so we convert it accordingly.
    let result = parse_datetime(input.as_bytes());

    // 14  finally we check the outcome, because the iso8601 crate uses nom 
    // (which works with streams of data), we can either return some data, an 
    // error or incomplete (if the stream is truncated)
    result.is_done() // thanks badboy!
}
