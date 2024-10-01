// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::core::{ShupaiBlockCountPattern, Wanzi19BlockCount, ZipaiBlockCount};
use super::hash::{hash_19m, hash_shupai, hash_zipai};
use super::shupai_map::SHUPAI_MAP;
use super::wanzi_19_map::WANZI_19_MAP;
use super::zipai_map::ZIPAI_MAP;

#[inline]
pub(super) fn count_shupai_block(single_color_bingpai: &[u8; 9]) -> ShupaiBlockCountPattern {
    SHUPAI_MAP[hash_shupai(single_color_bingpai)]
}

#[inline]
pub(super) fn count_zipai_block(zipai_bingpai: &[u8; 7]) -> ZipaiBlockCount {
    ZIPAI_MAP[hash_zipai(zipai_bingpai)]
}

#[inline]
pub(super) fn count_19m_block(wanzi_bingpai: &[u8; 9]) -> Wanzi19BlockCount {
    WANZI_19_MAP[hash_19m(wanzi_bingpai)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_shupai_block_works() {
        let mut single_color_bingpai = [1, 0, 3, 1, 2, 1, 0, 1, 0];
        let r = count_shupai_block(&mut single_color_bingpai);
        assert_eq!(r[0].0, 1);
        assert_eq!(r[0].1, 3);
        assert_eq!(r[0].2, 0);
        assert_eq!(r[0].3, 0);
        assert_eq!(r[0].4, 0b000000000);
        assert_eq!(r[0].5, 0b000000000);
        assert_eq!(r[0].6, 0b001001010);
        assert_eq!(r[0].7, 0b000000000);
        assert_eq!(r[0].8, 0b000000000);

        assert_eq!(r[1].0, 2);
        assert_eq!(r[1].1, 0);
        assert_eq!(r[1].2, 0);
        assert_eq!(r[1].3, 3);
        assert_eq!(r[1].4, 0b010010001);
        assert_eq!(r[1].5, 0b000000000);
        assert_eq!(r[1].6, 0b000000000);
        assert_eq!(r[1].7, 0b000000000);
        assert_eq!(r[1].8, 0b010010001);
    }

    #[test]
    fn count_shupai_block_empty() {
        let mut shupai_bingpai = [0, 0, 0, 0, 0, 0, 0, 0, 0];
        let r = count_shupai_block(&mut shupai_bingpai);
        assert_eq!(r[0].0, 0);
        assert_eq!(r[0].1, 0);
        assert_eq!(r[0].2, 0);
        assert_eq!(r[0].3, 0);
        assert_eq!(r[0].4, 0b000000000);
        assert_eq!(r[0].5, 0b000000000);
        assert_eq!(r[0].6, 0b000000000);
        assert_eq!(r[0].7, 0b000000000);
        assert_eq!(r[0].8, 0b000000000);

        assert_eq!(r[1].0, 0);
        assert_eq!(r[1].1, 0);
        assert_eq!(r[1].2, 0);
        assert_eq!(r[1].3, 0);
        assert_eq!(r[1].4, 0b000000000);
        assert_eq!(r[1].5, 0b000000000);
        assert_eq!(r[1].6, 0b000000000);
        assert_eq!(r[1].7, 0b000000000);
        assert_eq!(r[1].8, 0b000000000);
    }

    #[test]
    fn count_zipai_block_works() {
        let zipai_bingpai = [1, 0, 3, 1, 2, 4, 0];
        let r = count_zipai_block(&zipai_bingpai);
        assert_eq!(r.0, 2);
        assert_eq!(r.1, 1);
        assert_eq!(r.2, 3);
        assert_eq!(r.3, 0b0101001);
        assert_eq!(r.4, 0b0010000);
    }

    #[test]
    fn count_zipai_block_empty() {
        let zipai_bingpai = [0, 0, 0, 0, 0, 0, 0];
        let r = count_zipai_block(&zipai_bingpai);
        assert_eq!(r.0, 0);
        assert_eq!(r.1, 0);
        assert_eq!(r.2, 0);
        assert_eq!(r.3, 0b0000000);
        assert_eq!(r.4, 0b0000000);
    }

    #[test]
    fn count_19m_block_works() {
        let wanzi_bingpai = [4, 0, 0, 0, 0, 0, 0, 0, 2];
        let r = count_19m_block(&wanzi_bingpai);
        assert_eq!(r.0, 1);
        assert_eq!(r.1, 1);
        assert_eq!(r.2, 1);
        assert_eq!(r.3, 0b000000001);
        assert_eq!(r.4, 0b100000000);
    }

    #[test]
    fn count_19m_block_empty() {
        let wanzi_bingpai = [0, 0, 0, 0, 0, 0, 0, 0, 0];
        let r = count_19m_block(&wanzi_bingpai);
        assert_eq!(r.0, 0);
        assert_eq!(r.1, 0);
        assert_eq!(r.2, 0);
        assert_eq!(r.3, 0b000000000);
        assert_eq!(r.4, 0b000000000);
    }

    #[test]
    fn count_19m_block_ignore_2m_to_8m() {
        let wanzi_bingpai = [4, 3, 5, 0, 0, 0, 0, 1, 2];
        let r = count_19m_block(&wanzi_bingpai);
        assert_eq!(r.0, 1);
        assert_eq!(r.1, 1);
        assert_eq!(r.2, 1);
        assert_eq!(r.3, 0b000000001);
        assert_eq!(r.4, 0b100000000);
    }
}
