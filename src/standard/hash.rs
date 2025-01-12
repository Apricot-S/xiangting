// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::shupai_table::SHUPAI_TABLE;
use super::wanzi_19_table::WANZI_19_TABLE;
use super::zipai_table::ZIPAI_TABLE;

pub fn hash_shupai(single_color_bingpai: &[u8]) -> usize {
    let (hash, _) = single_color_bingpai
        .iter()
        .enumerate()
        .fold((0, 0), |(h, n), (i, &c)| {
            debug_assert!(i < 9);
            debug_assert!(c <= 4);
            debug_assert!(n + c <= 14);
            let updated_n = n + c;
            let updated_h = h + SHUPAI_TABLE[i][updated_n as usize][c as usize];
            (updated_h, updated_n)
        });
    hash
}

pub fn hash_zipai(zipai_bingpai: &[u8]) -> usize {
    let (hash, _) = zipai_bingpai
        .iter()
        .enumerate()
        .fold((0, 0), |(h, n), (i, &c)| {
            debug_assert!(i < 7);
            debug_assert!(c <= 4);
            debug_assert!(n + c <= 14);
            let updated_n = n + c;
            let updated_h = h + ZIPAI_TABLE[i][updated_n as usize][c as usize];
            (updated_h, updated_n)
        });
    hash
}

pub fn hash_19m(wanzi_bingpai: &[u8]) -> usize {
    let (hash, _) = wanzi_bingpai
        .iter()
        .step_by(8)
        .enumerate()
        .fold((0, 0), |(h, n), (i, &c)| {
            debug_assert!(i == 0 || i == 8);
            debug_assert!(c <= 4);
            debug_assert!(n + c <= 8);
            let index = if i == 0 { 0 } else { 1 };
            let updated_n = n + c;
            let updated_h = h + WANZI_19_TABLE[index][updated_n as usize][c as usize];
            (updated_h, updated_n)
        });
    hash
}
