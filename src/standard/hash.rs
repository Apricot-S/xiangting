// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::shupai_table::SHUPAI_TABLE;
use super::zipai_table::ZIPAI_TABLE;

pub fn hash_shupai(single_color_bingpai: &[u8]) -> u32 {
    let mut h: u32 = 0;
    let mut n: u8 = 0;

    for (i, &c) in single_color_bingpai.iter().enumerate() {
        debug_assert!(i < 9);
        debug_assert!(c <= 4);
        debug_assert!(n + c <= 14);
        n += c;
        h += SHUPAI_TABLE[i][n as usize][c as usize];
    }

    debug_assert!(n <= 14);

    h
}

pub fn hash_zipai(zipai_bingpai: &[u8]) -> u32 {
    let mut h: u32 = 0;
    let mut n: u8 = 0;

    for (i, &c) in zipai_bingpai.iter().enumerate() {
        debug_assert!(i < 7);
        debug_assert!(c <= 4);
        debug_assert!(n + c <= 14);
        n += c;
        h += ZIPAI_TABLE[i][n as usize][c as usize];
    }

    debug_assert!(n <= 14);

    h
}