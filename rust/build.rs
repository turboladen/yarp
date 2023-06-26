use std::{
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};

fn main() -> Result<(), io::Error> {
    make()?;

    let dir_str = env!("CARGO_MANIFEST_DIR");
    let rust_path = Path::new(dir_str);
    let ruby_build_path = rust_path.join("../build/").canonicalize().unwrap();
    let ruby_include_path = rust_path.join("../include/").canonicalize().unwrap();

    println!("cargo:rustc-link-lib=static=rubyparser");
    println!(
        "cargo:rustc-link-search=native={}",
        ruby_build_path.display()
    );

    let bindings = bindgen::Builder::default()
        .header("../include/yarp/defines.h")
        .header("../include/yarp.h")
        .clang_arg(format!("-I{}", ruby_include_path.display()))
        // Structs
        .allowlist_type("yp_buffer_t")
        .allowlist_type("yp_comment_t")
        .allowlist_type("yp_comment_t")
        .allowlist_type("yp_diagnostic_t")
        .allowlist_type("yp_node_t")
        .allowlist_type("yp_parser_t")
        // Enums
        .rustified_non_exhaustive_enum("yp_comment_type_t")
        // Functions
        .allowlist_function("yp_buffer_init")
        .allowlist_function("yp_buffer_free")
        .allowlist_function("yp_node_destroy")
        .allowlist_function("yp_parse")
        .allowlist_function("yp_parse_serialize")
        .allowlist_function("yp_parser_free")
        .allowlist_function("yp_parser_init")
        .allowlist_function("yp_prettyprint")
        .allowlist_function("yp_serialize")
        .allowlist_function("yp_version")
        .generate()
        .expect("Unable to generate yarp bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}

fn make() -> Result<(), io::Error> {
    let output = Command::new("rake").arg("compile").output()?;

    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    println!("`rake compile` exit status: {}", output.status);

    Ok(())
}
