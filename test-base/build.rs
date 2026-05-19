use std::io::Write;

fn main() {
    write("memory.x", include_bytes!("memory.x"));
    println!("cargo:rustc-link-arg=-Tlink.x");
}

fn write(file: &str, contents: &[u8]) {
    let out = &std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::File::create(out.join("memory.x"))
        .unwrap()
        .write_all(contents)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed={}", file);
}
