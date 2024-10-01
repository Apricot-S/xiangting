// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

// table[i][n][s]
// i = 0, 1, ..., 8
// n = 0, 1, ..., 14
// s = 0, 1, 2, 3, 4
pub type ShupaiTable = [[[usize; 5]; 15]; 9];

// table[i][n][s]
// i = 0, 1, ..., 6
// n = 0, 1, ..., 14
// s = 0, 1, 2, 3, 4
pub type ZipaiTable = [[[usize; 5]; 15]; 7];

// table[i][n][s]
// i = 0, 1
// n = 0, 1, ..., 14
// s = 0, 1, 2, 3, 4
pub type Wanzi19Table = [[[usize; 5]; 15]; 2];

// shupai_block_count.0 : number of melds
// shupai_block_count.1 : number of joints
// shupai_block_count.2 : number of pairs
// shupai_block_count.3 : number of isolated tiles
// shupai_block_count.4 : isolated tiles flag
// shupai_block_count.5 : open wait flag
// shupai_block_count.6 : edge & closed wait flag
// shupai_block_count.7 : dual pair wait flag
// shupai_block_count.8 : pair wait flag
pub type ShupaiBlockCount = (u8, u8, u8, u8, u16, u16, u16, u16, u16);

// pattern[0] : pattern with the minimum number of isolated tiles
// pattern[1] : pattern with the maximum number of melds
pub type ShupaiBlockCountPattern = [ShupaiBlockCount; 2];

// zipai_block_count.0 : number of melds
// zipai_block_count.1 : number of pairs
// zipai_block_count.2 : number of isolated tiles
// zipai_block_count.3 : isolated tiles flag (= pair wait flag)
// zipai_block_count.4 : dual pair wait flag
pub type ZipaiBlockCount = (u8, u8, u8, u8, u8);

// wanzi19_block_count.0 : number of melds
// wanzi19_block_count.1 : number of pairs
// wanzi19_block_count.2 : number of isolated tiles
// wanzi19_block_count.3 : isolated tiles flag (= pair wait flag)
// wanzi19_block_count.4 : dual pair wait flag
pub type Wanzi19BlockCount = (u8, u8, u8, u16, u16);

pub type ShupaiMapValue = ShupaiBlockCountPattern;
pub type ShupaiMap = Vec<ShupaiMapValue>;

pub type ZipaiMapValue = ZipaiBlockCount;
pub type ZipaiMap = Vec<ZipaiMapValue>;

pub type Wanzi19MapValue = Wanzi19BlockCount;
pub type Wanzi19Map = Vec<Wanzi19MapValue>;
