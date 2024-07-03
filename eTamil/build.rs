extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=antlr/eTamil.g4");
    let output = std::process::Command::new("antlr4")
        .args(&["-Dlanguage=Rust", "antlr/eTamil.g4", "-o", "src"])
        .output()
        .expect("Failed to generate parser with ANTLR");
    if !output.status.success() {
        panic!("ANTLR failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    // You can add additional build logic here
    cc::Build::new()
        .file("src/some_c_file.c")
        .compile("some_c_file");
}