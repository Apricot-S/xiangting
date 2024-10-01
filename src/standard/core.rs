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
// shupai_block_count.1 : number of joints
// shupai_block_count.2 : number of pairs
// shupai_block_count.3 : number of isolated tiles
// shupai_block_count.4 : isolated tiles flag
// shupai_block_count.5 : open wait flag (record only the smaller one. e.g., 1 for 23)
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
pub type ZipaiMapValue = ZipaiBlockCount;
pub type Wanzi19MapValue = Wanzi19BlockCount;
