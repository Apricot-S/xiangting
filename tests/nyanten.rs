// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

#![allow(non_snake_case)]

#[cxx::bridge(namespace = "Nyanten")]
mod ffi {
    extern "C++" {
        include!("/workspaces/xiangting/include/cxx_nyanten.hpp");

        #[allow(dead_code)]
        unsafe fn calculateReplacementNumber(first: *const u8, last: *const u8) -> u8;

    }
}

#[allow(unused_imports)]
pub(crate) use ffi::calculateReplacementNumber;
