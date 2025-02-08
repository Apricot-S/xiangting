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

// Each element contains the following structure:
//
// Index [0]:
//  0- 3 Bits: Replacement number of 0 pair 1 melds
//  4- 7 Bits: Replacement number of 0 pair 2 melds
//  8-11 Bits: Replacement number of 0 pair 3 melds
// 12-15 Bits: Replacement number of 0 pair 4 melds
// 16-19 Bits: Replacement number of 1 pair 0 melds
// 20-23 Bits: Replacement number of 1 pair 1 melds
// 24-27 Bits: Replacement number of 1 pair 2 melds
// 28-31 Bits: Replacement number of 1 pair 3 melds
//
// Index [1]:
//  0- 8 Bits: Necessary tiles of 0 pair 1 melds
//  9-17 Bits: Necessary tiles of 0 pair 2 melds
// 18-26 Bits: Necessary tiles of 0 pair 3 melds
// 27-31 Bits: Unused
//
// Index [2]:
//  0- 8 Bits: Necessary tiles of 0 pair 4 melds
//  9-17 Bits: Necessary tiles of 1 pair 0 melds
// 18-26 Bits: Necessary tiles of 1 pair 1 melds
// 27-31 Bits: Unused
//
// Index [3]:
//  0- 8 Bits: Necessary tiles of 1 pair 2 melds
//  9-17 Bits: Necessary tiles of 1 pair 3 melds
// 18-26 Bits: Necessary tiles of 1 pair 4 melds
// 27-30 Bits: Replacement number of 1 pair 4 melds
//    31  Bit: Unused
pub type MapValue = [u32; 4];
