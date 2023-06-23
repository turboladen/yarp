#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::{
        ffi::{CStr, CString},
        mem::MaybeUninit,
        path::Path,
    };

    use super::*;

    fn ruby_file_contents() -> (CString, usize) {
        let rust_path = Path::new(env!("CARGO_MANIFEST_DIR"));
        let ruby_file_path = rust_path.join("../lib/yarp.rb").canonicalize().unwrap();
        let file_contents = std::fs::read_to_string(ruby_file_path).unwrap();
        let len = file_contents.len();

        (CString::new(file_contents).unwrap(), len)
    }

    #[test]
    fn init_test() {
        let (ruby_file_contents, len) = ruby_file_contents();
        let source = ruby_file_contents.as_ptr();
        let mut parser = MaybeUninit::<yp_parser_t>::uninit();

        unsafe {
            yp_parser_init(parser.as_mut_ptr(), source, len, std::ptr::null());
            let parser = parser.assume_init_mut();

            yp_parser_free(parser);
        }
    }

    #[test]
    fn version_test() {
        let cstring = unsafe {
            let version = yp_version();
            CStr::from_ptr(version)
        };

        assert_eq!(&cstring.to_string_lossy(), "0.4.0");
    }
}
