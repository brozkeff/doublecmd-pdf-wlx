// Copyright (C) 2026 Martin Brozkeff Malec
// SPDX-License-Identifier: EUPL-1.2 OR GPL-2.0-or-later OR AGPL-3.0-or-later

use std::env;

fn main() {
    println!("cargo:rerun-if-changed=src/qt5_shim.cpp");
    println!("cargo:rerun-if-changed=src/qt5_shim.hpp");
    println!("cargo:rerun-if-changed=src/poppler_shim.cpp");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_POPPLER_SPLASH");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_MUTOOL");

    let poppler_enabled = env::var_os("CARGO_FEATURE_POPPLER_SPLASH").is_some();
    let mutool_enabled = env::var_os("CARGO_FEATURE_MUTOOL").is_some();
    assert!(
        poppler_enabled ^ mutool_enabled,
        "enable exactly one renderer feature: `poppler-splash` or `mutool`"
    );

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

    if poppler_enabled {
        let poppler = pkg_config::Config::new()
            .probe("poppler-qt5")
            .expect("Poppler Qt5 development files are required for the Splash backend");
        build.file("src/poppler_shim.cpp");
        build.flag_if_supported("-Wno-deprecated-declarations");
        for include in poppler.include_paths {
            build.include(include);
        }
    }

    build.compile("pdf_wlx_qt5_shim");
}
