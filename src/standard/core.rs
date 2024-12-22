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
// [Bits 0-3]: Replacement number without a pair
// [Bits 4-12]: Necessary tiles without a pair
// [Bits 13-15]: Unused
// [Bits 16-19]: Replacement number with a pair
// [Bits 20-28]: Necessary tiles with a pair
// [Bits 29-31]: Unused
//
// Index:
// [0]: 0 melds
// [1]: 1 melds
// [2]: 2 melds
// [3]: 3 melds
// [4]: 4 melds
pub type MapValue = [u32; 5];
