// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::shupai_table::SHUPAI_TABLE;
use super::wanzi_19_table::WANZI_19_TABLE;
use super::zipai_table::ZIPAI_TABLE;

pub fn hash_shupai(single_color_bingpai: &[u8]) -> usize {
    let mut h: usize = 0;
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

pub fn hash_zipai(zipai_bingpai: &[u8]) -> usize {
    let mut h: usize = 0;
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

pub fn hash_19m(wanzi_bingpai: &[u8]) -> usize {
    let mut h: usize = 0;
    let mut n: u8 = 0;

    let c_1m = wanzi_bingpai[0];
    debug_assert!(c_1m <= 4);
    n += c_1m;
    h += WANZI_19_TABLE[0][n as usize][c_1m as usize];

    let c_9m = wanzi_bingpai[8];
    debug_assert!(c_9m <= 4);
    debug_assert!(n + c_9m <= 8);
    n += c_9m;
    h += WANZI_19_TABLE[1][n as usize][c_9m as usize];

    debug_assert!(n <= 8);

    h
}
