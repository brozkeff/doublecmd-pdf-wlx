// Copyright (C) 2026 Martin Brozkeff Malec
// Licensed under the EUPL, Version 1.2.

fn main() {
    println!("cargo:rerun-if-changed=src/qt5_shim.cpp");
    let qt = pkg_config::Config::new()
        .atleast_version("5.15")
        .probe("Qt5Widgets")
        .expect("Qt 5.15 Widgets development files are required");

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .file("src/qt5_shim.cpp")
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-fPIC")
        .flag_if_supported("-fvisibility=hidden")
        .define("QT_NO_DEBUG", None);
    for include in qt.include_paths {
        build.include(include);
    }
    build.compile("pdf_wlx_qt5_shim");
}
