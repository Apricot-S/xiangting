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

// map_value[x].0 : replacement number
// map_value[x].1 : necesaary tiles and unnecessary tiles
// [0] : 0 pairs, 0 melds
// [1] : 0 pairs, 1 melds
// [2] : 0 pairs, 2 melds
// [3] : 0 pairs, 3 melds
// [4] : 0 pairs, 4 melds
// [5] : 1 pairs, 0 melds
// [6] : 1 pairs, 1 melds
// [7] : 1 pairs, 2 melds
// [8] : 1 pairs, 3 melds
// [9] : 1 pairs, 4 melds
pub type MapValue = [(u8, u16); 10];
