extern crate bindgen;

use std::path::PathBuf;

fn main() {
    // println!("cargo:rustc-link-lib=cuda");

    let bindings = bindgen::Builder::default()
        .ctypes_prefix("::libc")
        .size_t_is_usize(true)
        .allowlist_type("^CU.*")
        .allowlist_type("^cuuint(32|64)_t")
        .allowlist_type("^cudaError_enum")
        .allowlist_type("^cu.*Complex$")
        .allowlist_type("^cuda.*")
        .allowlist_type("^libraryPropertyType.*")
        .allowlist_var("^CU.*")
        .allowlist_function("^cu.*")
        .clang_arg("-I")
        .clang_arg("/usr/local/cuda/include".to_string())
        .header("wrapper.h")
        .default_alias_style(bindgen::AliasVariation::TypeAlias)
        .derive_default(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_ord(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src/").join("cuda.rs");
    bindings
        .write_to_file(out_path)
        .expect("Unable to write");

}