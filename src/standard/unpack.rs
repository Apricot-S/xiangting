// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::core::{NecessaryTilesMapValue, ReplacementNumberMapValue, UnnecessaryTilesMapValue};

// Index:
// [0] : 0 pair, 0 melds
// [1] : 0 pair, 1 melds
// [2] : 0 pair, 2 melds
// [3] : 0 pair, 3 melds
// [4] : 0 pair, 4 melds
// [5] : 1 pair, 0 melds
// [6] : 1 pair, 1 melds
// [7] : 1 pair, 2 melds
// [8] : 1 pair, 3 melds
// [9] : 1 pair, 4 melds
pub(super) type UnpackedNumbers = [u8; 10];
pub(super) type UnpackedTiles = [u16; 10];

#[inline]
pub(super) fn unpack_replacement_number(pack: &ReplacementNumberMapValue) -> UnpackedNumbers {
    [
        0u8,
        (pack & 0b11) as u8,
        ((pack >> 2) & 0b111) as u8,
        ((pack >> 5) & 0b1111) as u8,
        ((pack >> 9) & 0b1111) as u8,
        ((pack >> 13) & 0b11) as u8,
        ((pack >> 15) & 0b111) as u8,
        ((pack >> 18) & 0b1111) as u8,
        ((pack >> 22) & 0b1111) as u8,
        ((pack >> 26) & 0b1111) as u8,
    ]
}

#[inline]
pub(super) fn unpack_necessary_tiles(pack: &NecessaryTilesMapValue) -> UnpackedTiles {
    [
        0u16,
        (pack[0] & 0x01FF) as u16,
        ((pack[0] >> 9) & 0x01FF) as u16,
        ((pack[0] >> (9 * 2)) & 0x01FF) as u16,
        (pack[1] & 0x01FF) as u16,
        ((pack[1] >> 9) & 0x01FF) as u16,
        ((pack[1] >> (9 * 2)) & 0x01FF) as u16,
        (pack[2] & 0x01FF) as u16,
        ((pack[2] >> 9) & 0x01FF) as u16,
        ((pack[2] >> (9 * 2)) & 0x01FF) as u16,
    ]
}

#[inline]
pub(super) fn unpack_unnecessary_tiles(pack: &UnnecessaryTilesMapValue) -> UnpackedTiles {
    [
        (pack[0] & 0x01FF) as u16,
        ((pack[0] >> 9) & 0x01FF) as u16,
        ((pack[0] >> (9 * 2)) & 0x01FF) as u16,
        (((pack[0] >> (9 * 3 - 4)) & 0x01F0) | (pack[1] & 0x0F)) as u16,
        ((pack[1] >> 4) & 0x01FF) as u16,
        ((pack[1] >> (4 + 9)) & 0x01FF) as u16,
        ((pack[1] >> (4 + 9 * 2)) & 0x01FF) as u16,
        (pack[2] & 0x01FF) as u16,
        ((pack[2] >> 9) & 0x01FF) as u16,
        ((pack[2] >> (9 * 2)) & 0x01FF) as u16,
    ]
}
