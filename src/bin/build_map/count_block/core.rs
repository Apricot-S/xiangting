// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use bitvec::prelude::*;
use xiangting::standard::core::{
    ShupaiBlockCount, ShupaiBlockCountPattern, Wanzi19BlockCount, ZipaiBlockCount,
};

pub(super) type SingleColorTileFlag = BitArr!(for 9);

pub(super) fn to_flag(single_color_bingpai: &[u8]) -> SingleColorTileFlag {
    single_color_bingpai.iter().enumerate().fold(
        SingleColorTileFlag::ZERO,
        |mut flag, (i, &count)| {
            flag.set(i, count > 0);
            flag
        },
    )
}

pub(super) struct ShupaiBlockCountImpl {
    pub(super) num_mianzi: u8,
    pub(super) num_dazi: u8,
    pub(super) num_duizi: u8,
    pub(super) num_gulipai: u8,
    pub(super) gulipai: SingleColorTileFlag,
    pub(super) liangmian_ting: SingleColorTileFlag,
    pub(super) biankanzhang_ting: SingleColorTileFlag,
    pub(super) shuangpeng_ting: SingleColorTileFlag,
    pub(super) danqi_ting: SingleColorTileFlag,
}

impl ShupaiBlockCountImpl {
    pub(super) fn to_entry(&self) -> ShupaiBlockCount {
        (
            self.num_mianzi,
            self.num_dazi,
            self.num_duizi,
            self.num_gulipai,
            self.gulipai.load(),
            self.liangmian_ting.load(),
            self.biankanzhang_ting.load(),
            self.shuangpeng_ting.load(),
            self.danqi_ting.load(),
        )
    }
}

pub(super) struct ShupaiBlockCountPatternImpl {
    pub(super) a: ShupaiBlockCountImpl, // Pattern with the minimum number of isolated tiles
    pub(super) b: ShupaiBlockCountImpl, // Pattern with the maximum number of melds
}

impl ShupaiBlockCountPatternImpl {
    pub(super) fn to_entry(&self) -> ShupaiBlockCountPattern {
        [self.a.to_entry(), self.b.to_entry()]
    }
}

pub(super) struct ZipaiBlockCountImpl {
    pub(super) num_mianzi: u8,
    pub(super) num_duizi: u8,
    pub(super) num_gulipai: u8,
    pub(super) gulipai: SingleColorTileFlag,
    pub(super) shuangpeng_ting: SingleColorTileFlag,
}

impl ZipaiBlockCountImpl {
    pub(super) fn to_entry(&self) -> ZipaiBlockCount {
        (
            self.num_mianzi,
            self.num_duizi,
            self.num_gulipai,
            self.gulipai.load(),
            self.shuangpeng_ting.load(),
        )
    }
}

pub(super) struct Wanzi19BlockCountImpl {
    pub(super) num_mianzi: u8,
    pub(super) num_duizi: u8,
    pub(super) num_gulipai: u8,
    pub(super) gulipai: SingleColorTileFlag,
    pub(super) shuangpeng_ting: SingleColorTileFlag,
}

impl Wanzi19BlockCountImpl {
    pub(super) fn to_entry(&self) -> Wanzi19BlockCount {
        (
            self.num_mianzi,
            self.num_duizi,
            self.num_gulipai,
            self.gulipai.load(),
            self.shuangpeng_ting.load(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_flag_empty() {
        let single_color_bingpai = [0u8; 9];
        let flag = to_flag(&single_color_bingpai);
        assert_eq!(flag.load::<u16>(), 0b000000000);
    }

    #[test]
    fn to_flag_shupai() {
        let shupai_bingpai = [1, 0, 3, 1, 2, 1, 0, 1, 0];
        let flag = to_flag(&shupai_bingpai);
        assert_eq!(flag.load::<u16>(), 0b010111101);
    }

    #[test]
    fn to_flag_zipai() {
        let zipai_bingpai = [1, 0, 0, 1, 4, 0, 1];
        let flag = to_flag(&zipai_bingpai);
        assert_eq!(flag.load::<u8>(), 0b1011001);
    }
}
