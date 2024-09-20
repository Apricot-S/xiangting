// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

fn main() {
    if std::env::var("CARGO_FEATURE_CORRECTNESS").is_ok() {
        cxx_build::bridge("tests/nyanten.rs")
            .file("/workspaces/nyanten/nyanten/replacement_number.hpp")
            .flag_if_supported("-std=c++23")
            .compile("nyanten");

        println!("cargo:rerun-if-changed=/workspaces/nyanten/nyanten/replacement_number.hpp");
        println!("cargo:rerun-if-changed=tests/nyanten.rs");
        println!("cargo:rerun-if-changed=tests/correctness.rs");
    }
}
