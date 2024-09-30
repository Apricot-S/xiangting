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

pub type MapValue = [u8; 5];
pub type Map = Vec<MapValue>;
