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

// block_count.0 : number of mianzi
// block_count.1 : number of dazi
// block_count.2 : number of duizi
// block_count.3 : number of gulipai
// block_count.4 : gulipai flag
// block_count.5 : four tiles gulipai flag
pub type BlockCount = (u8, u8, u8, u8, u16, u16);

// pattern[0] : pattern with the minimum number of isolated tiles
// pattern[1] : pattern with the maximum number of melds
pub type BlockCountPattern = [BlockCount; 2];

pub type ShupaiMapValue = BlockCountPattern;
pub type ShupaiMap = Vec<ShupaiMapValue>;

pub type ZipaiMapValue = BlockCount;
pub type ZipaiMap = Vec<ZipaiMapValue>;
