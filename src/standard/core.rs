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

// Value contains the following structure:
//
//  0- 1 Bits: Replacement number of 0 pair 1 melds
//  2- 4 Bits: Replacement number of 0 pair 2 melds
//  5- 8 Bits: Replacement number of 0 pair 3 melds
//  9-12 Bits: Replacement number of 0 pair 4 melds
// 13-14 Bits: Replacement number of 1 pair 0 melds
// 15-17 Bits: Replacement number of 1 pair 1 melds
// 18-21 Bits: Replacement number of 1 pair 2 melds
// 22-25 Bits: Replacement number of 1 pair 3 melds
// 26-29 Bits: Replacement number of 1 pair 4 melds
pub type ReplacementNumberMapValue = u32;

// Each element contains the following structure:
//
// Index [0]:
//  0- 8 Bits: Necessary tiles of 0 pair 1 melds
//  9-17 Bits: Necessary tiles of 0 pair 2 melds
// 18-26 Bits: Necessary tiles of 0 pair 3 melds
// 27-31 Bits: Unused
//
// Index [1]:
//  0- 8 Bits: Necessary tiles of 0 pair 4 melds
//  9-17 Bits: Necessary tiles of 1 pair 0 melds
// 18-26 Bits: Necessary tiles of 1 pair 1 melds
// 27-31 Bits: Unused
//
// Index [2]:
//  0- 8 Bits: Necessary tiles of 1 pair 2 melds
//  9-17 Bits: Necessary tiles of 1 pair 3 melds
// 18-26 Bits: Necessary tiles of 1 pair 4 melds
// 27-31 Bits: Unused
pub type NecessaryTilesMapValue = [u32; 3];
