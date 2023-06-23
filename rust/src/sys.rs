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

    #[test]
    fn parse_and_print_test() {
        let (ruby_file_contents, len) = ruby_file_contents();
        let source = ruby_file_contents.as_ptr();
        let mut parser = MaybeUninit::<yp_parser_t>::uninit();
        let mut buffer = MaybeUninit::<yp_buffer_t>::uninit();

        unsafe {
            yp_parser_init(parser.as_mut_ptr(), source, len, std::ptr::null());
            let parser = parser.assume_init_mut();
            let node = yp_parse(parser);

            if !yp_buffer_init(buffer.as_mut_ptr()) {
                panic!("Failed to init buffer");
            }

            let buffer = buffer.assume_init_mut();
            yp_prettyprint(parser, node, buffer);

            let string =
                String::from_raw_parts(buffer.value as *mut u8, buffer.length, buffer.capacity);
            assert!(string.starts_with("ProgramNode"));

            yp_node_destroy(parser, node);

            yp_parser_free(parser);
        }
    }

    #[test]
    fn serialize_test() {
        let (ruby_file_contents, len) = ruby_file_contents();
        let source = ruby_file_contents.as_ptr();
        let mut parser = MaybeUninit::<yp_parser_t>::uninit();
        let mut buffer = MaybeUninit::<yp_buffer_t>::uninit();

        unsafe {
            yp_parser_init(parser.as_mut_ptr(), source, len, std::ptr::null());
            let parser = parser.assume_init_mut();
            let node = yp_parse(parser);

            if !yp_buffer_init(buffer.as_mut_ptr()) {
                panic!("Failed to init buffer");
            }

            let buffer = buffer.assume_init_mut();
            yp_serialize(parser, node, buffer);

            let serialized =
                Vec::from_raw_parts(buffer.value as *mut u8, buffer.length, buffer.capacity);

            assert_eq!(&serialized[0..4], b"YARP");
            assert_eq!(serialized[4..5][0], 0); // YP_VERSION_MAJOR
            assert_eq!(serialized[5..6][0], 4); // YP_VERSION_MINOR
            assert_eq!(serialized[6..7][0], 0); // YP_VERSION_PATCH

            yp_node_destroy(parser, node);
            yp_parser_free(parser);
        }
    }

    #[test]
    fn parse_serialize_test() {
        let (ruby_file_contents, len) = ruby_file_contents();
        let source = ruby_file_contents.as_ptr();
        let mut parser = MaybeUninit::<yp_parser_t>::uninit();
        let mut serialize_buffer = MaybeUninit::<yp_buffer_t>::uninit();
        let mut parse_serialize_buffer = MaybeUninit::<yp_buffer_t>::uninit();

        let serialized = unsafe {
            yp_parser_init(parser.as_mut_ptr(), source, len, std::ptr::null());
            let parser = parser.assume_init_mut();
            let node = yp_parse(parser);

            if !yp_buffer_init(serialize_buffer.as_mut_ptr()) {
                panic!("Failed to init buffer");
            }

            let serialize_buffer = serialize_buffer.assume_init_mut();
            yp_serialize(parser, node, serialize_buffer);

            yp_node_destroy(parser, node);
            yp_parser_free(parser);

            // Can't use String -> CString here because `value` contains nul bytes.
            Vec::from_raw_parts(
                serialize_buffer.value as *mut u8,
                serialize_buffer.length,
                serialize_buffer.capacity,
            )
        };

        unsafe {
            if !yp_buffer_init(parse_serialize_buffer.as_mut_ptr()) {
                panic!("Failed to init buffer");
            }

            let parse_serialize_buffer = parse_serialize_buffer.assume_init_mut();

            yp_parse_serialize(
                serialized.as_ptr() as *const i8,
                serialized.len(),
                parse_serialize_buffer,
            );

            let string = String::from_raw_parts(
                parse_serialize_buffer.value as *mut u8,
                parse_serialize_buffer.length,
                parse_serialize_buffer.capacity,
            );
            assert!(string.starts_with("YARP"));
        }
    }
}
