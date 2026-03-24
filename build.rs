fn main() {
    cxx_build::bridge("src/lib.rs")
        .compile("fluvio_client_cpp");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
