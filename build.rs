// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

#[cfg(feature = "correctness")]
fn main() {
    if std::env::var("CARGO_FEATURE_CORRECTNESS").is_ok() {
        cxx_build::bridge("tests/nyanten.rs")
            .include("/workspaces/nyanten")
            .include("/workspaces/xiangting/include")
            .flag_if_supported("-std=c++23")
            .compile("nyanten");

        println!("cargo:rerun-if-changed=/workspaces/nyanten/nyanten/replacement_number.hpp");
        println!("cargo:rerun-if-changed=include/nyanten_cxx.hpp");
        println!("cargo:rerun-if-changed=tests/nyanten.rs");
        println!("cargo:rerun-if-changed=tests/correctness.rs");
    }
}

#[cfg(not(feature = "correctness"))]
fn main() {}
