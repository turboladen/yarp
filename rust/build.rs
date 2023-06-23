use std::{
    io::{self, Write},
    path::Path,
    process::Command,
};

fn main() -> Result<(), io::Error> {
    make()?;

    let dir_str = env!("CARGO_MANIFEST_DIR");
    let rust_path = Path::new(dir_str);
    let ruby_build_path = rust_path.join("../build/").canonicalize().unwrap();

    println!("cargo:rustc-link-lib=static=rubyparser");
    println!(
        "cargo:rustc-link-search=native={}",
        ruby_build_path.display()
    );

    Ok(())
}

fn make() -> Result<(), io::Error> {
    let output = Command::new("rake").arg("compile").output()?;

    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    println!("`rake compile` exit status: {}", output.status);

    Ok(())
}
