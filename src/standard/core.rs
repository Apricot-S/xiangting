// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

// table[i][n][s]
// i = 0, 1, ..., N - 1 (N = 9 or 7 or 2)
// n = 0, 1, ..., 14
// s = 0, 1, 2, 3, 4
type Table<const I: usize> = [[[usize; 5]; 15]; I];

pub type ShupaiTable = Table<9>;
pub type ZipaiTable = Table<7>;
pub type Wanzi19Table = Table<2>;

// shupai_block_count.0 : number of melds
// shupai_block_count.1 : number of meld candidates (joints + pairs)
// shupai_block_count.2 : maximum number of pairs
// shupai_block_count.3 : number of isolated tiles
// shupai_block_count.4 : isolated tiles flag
// shupai_block_count.5 : open wait flag (record only the smaller one. e.g., 1 for 23)
// shupai_block_count.6 : edge & closed wait flag
// shupai_block_count.7 : dual pair wait flag
// shupai_block_count.8 : pair wait flag
pub type ShupaiBlockCount = (u8, u8, u8, u8, u16, u16, u16, u16, u16);

pub trait ShupaiBlockCountExt {
    fn num_mianzi(&self) -> u8;
    fn num_mianzi_candidate(&self) -> u8;
    fn num_duizi(&self) -> u8;
    fn num_gulipai(&self) -> u8;
    fn gulipai(&self) -> u16;
    fn liangmian_ting(&self) -> u16;
    fn biankanzhang_ting(&self) -> u16;
    fn shuangpeng_ting(&self) -> u16;
    fn danqi_ting(&self) -> u16;
}

impl ShupaiBlockCountExt for ShupaiBlockCount {
    #[inline]
    fn num_mianzi(&self) -> u8 {
        self.0
    }
    #[inline]
    fn num_mianzi_candidate(&self) -> u8 {
        self.1
    }
    #[inline]
    fn num_duizi(&self) -> u8 {
        self.2
    }
    #[inline]
    fn num_gulipai(&self) -> u8 {
        self.3
    }
    #[inline]
    fn gulipai(&self) -> u16 {
        self.4
    }
    #[inline]
    fn liangmian_ting(&self) -> u16 {
        self.5
    }
    #[inline]
    fn biankanzhang_ting(&self) -> u16 {
        self.6
    }
    #[inline]
    fn shuangpeng_ting(&self) -> u16 {
        self.7
    }
    #[inline]
    fn danqi_ting(&self) -> u16 {
        self.8
    }
}

// pattern[0] : pattern with the minimum number of isolated tiles
// pattern[1] : pattern with the maximum number of melds
pub type ShupaiBlockCountPattern = [ShupaiBlockCount; 2];

// zipai_block_count.0 : number of melds
// zipai_block_count.1 : number of pairs
// zipai_block_count.2 : number of isolated tiles
// zipai_block_count.3 : isolated tiles flag (= pair wait flag)
// zipai_block_count.4 : dual pair wait flag
pub type ZipaiBlockCount = (u8, u8, u8, u8, u8);

pub trait ZipaiBlockCountExt {
    fn num_mianzi(&self) -> u8;
    fn num_duizi(&self) -> u8;
    fn num_gulipai(&self) -> u8;
    fn gulipai(&self) -> u8;
    fn shuangpeng_ting(&self) -> u8;
}

impl ZipaiBlockCountExt for ZipaiBlockCount {
    #[inline]
    fn num_mianzi(&self) -> u8 {
        self.0
    }
    #[inline]
    fn num_duizi(&self) -> u8 {
        self.1
    }
    #[inline]
    fn num_gulipai(&self) -> u8 {
        self.2
    }
    #[inline]
    fn gulipai(&self) -> u8 {
        self.3
    }
    #[inline]
    fn shuangpeng_ting(&self) -> u8 {
        self.4
    }
}

// wanzi19_block_count.0 : number of melds
// wanzi19_block_count.1 : number of pairs
// wanzi19_block_count.2 : number of isolated tiles
// wanzi19_block_count.3 : isolated tiles flag (= pair wait flag)
// wanzi19_block_count.4 : dual pair wait flag
pub type Wanzi19BlockCount = (u8, u8, u8, u16, u16);

pub trait Wanzi19BlockCountExt {
    fn num_mianzi(&self) -> u8;
    fn num_duizi(&self) -> u8;
    fn num_gulipai(&self) -> u8;
    fn gulipai(&self) -> u16;
    fn shuangpeng_ting(&self) -> u16;
}

impl Wanzi19BlockCountExt for Wanzi19BlockCount {
    #[inline]
    fn num_mianzi(&self) -> u8 {
        self.0
    }
    #[inline]
    fn num_duizi(&self) -> u8 {
        self.1
    }
    #[inline]
    fn num_gulipai(&self) -> u8 {
        self.2
    }
    #[inline]
    fn gulipai(&self) -> u16 {
        self.3
    }
    #[inline]
    fn shuangpeng_ting(&self) -> u16 {
        self.4
    }
}

pub type ShupaiMapValue = ShupaiBlockCountPattern;
pub type ZipaiMapValue = ZipaiBlockCount;
pub type Wanzi19MapValue = Wanzi19BlockCount;
