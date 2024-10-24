
fn main() {
    let rocksdb_path = if let Ok(s) = std::env::var("ROCKSDB_PATH") {
        s
    } else {
        panic!("ENV VAR: ROCKSDB_PATH NOT SET")
    };

    cxx_build::bridge("src/lib.rs")
        .file("src/db.cc")
        .include(format!("{}/include", rocksdb_path))
        .std("c++17")
        .compile("out");

    println!("cargo:rustc-link-lib=static=rocksdb");
    println!("cargo:rustc-link-search=native={}/", rocksdb_path);

    if let Err(_) = std::env::var("IN_LAB") {
        println!("cargo:rustc-link-lib=zstd");
        println!("cargo:rustc-link-lib=lz4");
        println!("cargo:rustc-link-lib=bz2");
        println!("cargo:rustc-link-lib=snappy");
    }

    println!("cargo:rustc-link-lib=z");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/foo.cc");
    println!("cargo:rerun-if-changed=include/foo.h");
}
