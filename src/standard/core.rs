// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

// table[i][n][s]
// i = 0, 1, ..., 8
// n = 0, 1, ..., 14
// s = 0, 1, 2, 3, 4
pub(super) type ShupaiTable = [[[u32; 5]; 15]; 9];

// table[i][n][s]
// i = 0, 1, ..., 6
// n = 0, 1, ..., 14
// s = 0, 1, 2, 3, 4
pub(super) type ZipaiTable = [[[u32; 5]; 15]; 7];
