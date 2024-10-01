// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use bitvec::prelude::*;
use xiangting::standard::core::{BlockCount, BlockCountPattern};
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

pub(super) struct BlockCountImpl {
    pub(super) num_mianzi: u8,
    pub(super) num_dazi: u8,
    pub(super) num_duizi: u8,
    pub(super) num_gulipai: u8,
    pub(super) gulipai: SingleColorTileFlag,
    pub(super) four_tiles_gulipai: SingleColorTileFlag,
}

impl BlockCountImpl {
    pub(super) fn to_entry(&self) -> BlockCount {
        (
            self.num_mianzi,
            self.num_dazi,
            self.num_duizi,
            self.num_gulipai,
            self.gulipai.load(),
            self.four_tiles_gulipai.load(),
        )
    }
}

pub(super) struct BlockCountPatternImpl {
    pub(super) a: BlockCountImpl, // Pattern with the minimum number of isolated tiles
    pub(super) b: BlockCountImpl, // Pattern with the maximum number of melds
}

impl BlockCountPatternImpl {
    pub(super) fn to_entry(&self) -> BlockCountPattern {
        [self.a.to_entry(), self.b.to_entry()]
    }
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
    fn it_works() {}
}
