use std::{env, path::PathBuf};

fn main() {
    let sysroot = PathBuf::from(env::var("PKG_CONFIG_SYSROOT_DIR").unwrap());
    let include = &sysroot.join("usr/include").to_string_lossy().to_string();
    let lib = &sysroot.join("usr/lib").to_string_lossy().to_string();

    // Set rustflags sysroot
    println!("cargo:rustc-link-arg=--sysroot={}", sysroot.to_string_lossy());

    // Set search
    println!("cargo:rustc-link-search={lib}");

    // C appdir
    println!("cargo:rustc-link-lib=appdir");
    bindgen::Builder::default()
        .header(format!("{include}/appdir/appdir.h"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate appdir")
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("libappdir.rs"))
        .expect("Couldn't write appdir!");

    // Qt/cxx
    if cfg!(target_os = "linux") {
        let cpp_libs = vec!["Qt5Core", "Qt5Network"];
        let includes: Vec<Vec<PathBuf>> = cpp_libs.iter().map(|lib| pkg_config::probe_library(lib).unwrap().include_paths).collect();
        for bridge_name in ["format", "qnetwork"] {
            cxx_build::bridge(format!("src/cxx/rust/cxx_{bridge_name}.rs"))
                .includes(includes.iter().flatten())
                .std("c++17")
                .file(format!("src/cxx/cpp/{bridge_name}/cxx_{bridge_name}.cpp"))
                .compile(bridge_name);
            println!("cargo:rerun-if-changed=src/cxx/cpp/{bridge_name}/cxx_{bridge_name}.h");
            println!("cargo:rerun-if-changed=src/cxx/cpp/{bridge_name}/cxx_{bridge_name}.cpp");
            println!("cargo:rerun-if-changed=src/cxx/rust/cxx_{bridge_name}.rs");
        }
        for lib in cpp_libs {
            println!("cargo:rustc-link-lib={lib}");
        }
    }
}
