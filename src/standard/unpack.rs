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
pub(super) type UnpackedNumbers = [u32; 10];
pub(super) type UnpackedTiles = [u16; 10];

#[inline]
pub(super) fn unpack_replacement_number(pack: &ReplacementNumberMapValue) -> UnpackedNumbers {
    [
        0u32,
        pack & 0b11,
        (pack >> 2) & 0b111,
        (pack >> 5) & 0b1111,
        (pack >> 9) & 0b1111,
        (pack >> 13) & 0b11,
        (pack >> 15) & 0b111,
        (pack >> 18) & 0b1111,
        (pack >> 22) & 0b1111,
        (pack >> 26) & 0b1111,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unpack_replacement_number() {
        // 23489x : [0, 1, 1, 1, 0, 0, 0, 1, 1]
        let pack: ReplacementNumberMapValue = 0u32
            | 0
            | (1 << 2)
            | (4 << 5)
            | (7 << 9)
            | (1 << 13)
            | (1 << 15)
            | (3 << 18)
            | (6 << 22)
            | (9 << 26);
        let unpacked: UnpackedNumbers = [0, 0, 1, 4, 7, 1, 1, 3, 6, 9];

        assert_eq!(unpack_replacement_number(&pack), unpacked);
    }

    #[test]
    fn test_unpack_necessary_tiles() {
        // 23489x : [0, 1, 1, 1, 0, 0, 0, 1, 1]
        let pack: NecessaryTilesMapValue = [
            0b000000000 | (0b001000000 << 9) | (0b111111111 << (9 * 2)),
            0b111111111 | (0b110000000 << 9) | (0b111111111 << (9 * 2)),
            0b111111111 | (0b111111111 << 9) | (0b111111111 << (9 * 2)),
        ];
        let unpacked: UnpackedTiles = [
            0b000000000,
            0b000000000,
            0b001000000,
            0b111111111,
            0b111111111,
            0b110000000,
            0b111111111,
            0b111111111,
            0b111111111,
            0b111111111,
        ];

        assert_eq!(unpack_necessary_tiles(&pack), unpacked);
    }

    #[test]
    fn test_unpack_unnecessary_tiles() {
        // 23455689x : [0, 1, 1, 1, 2, 1, 0, 1, 1]
        let pack: UnnecessaryTilesMapValue = [
            0b110111110
                | (0b110110000 << 9)
                | (0b110110000 << (9 * 2))
                | ((0b000110000 & 0b111110000) << (9 * 3 - 4)),
            (0b000110000 & 0b1111)
                | (0b000000000 << 4)
                | (0b110101110 << (4 + 9))
                | (0b110100000 << (4 + 9 * 2)),
            0b100100000 | (0b000000000 << 9) | (0b000000000 << (9 * 2)),
        ];
        let unpacked: UnpackedTiles = [
            0b110111110,
            0b110110000,
            0b110110000,
            0b000110000,
            0b000000000,
            0b110101110,
            0b110100000,
            0b100100000,
            0b000000000,
            0b000000000,
        ];

        assert_eq!(unpack_unnecessary_tiles(&pack), unpacked);
    }
}
