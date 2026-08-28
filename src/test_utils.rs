// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::tile::{TileCounts, TileFlags};

pub trait FromTileCode: Sized {
    /// Converts a Tenhou-style tile string into `T`.
    fn from_code(hand: &str) -> Self {
        let mut suit_offset = None;
        let mut result = Self::empty();

        for byte in hand.bytes().rev() {
            match byte {
                b'm' => suit_offset = Some(0),
                b'p' => suit_offset = Some(9),
                b's' => suit_offset = Some(18),
                b'z' => suit_offset = Some(27),
                b'1'..=b'9' => {
                    let number = byte - b'0';
                    let base = suit_offset.expect("no type specified before the tile number");
                    let tile_index = base + usize::from(number - 1);
                    result = result.apply(tile_index);
                }
                _ => panic!("invalid character in hand: {byte:?}"),
            }
        }

        result
    }

    fn empty() -> Self;
    fn apply(self, idx: usize) -> Self;
}

impl FromTileCode for TileCounts {
    fn empty() -> Self {
        [0u8; 34]
    }

    fn apply(mut self, idx: usize) -> Self {
        self[idx] += 1;
        self
    }
}

impl FromTileCode for TileFlags {
    fn empty() -> Self {
        0u64
    }

    fn apply(self, idx: usize) -> Self {
        self | (1 << idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_code_tile_counts_normal() {
        let counts = TileCounts::from_code("123m456p789s12344z");
        let expected_counts: TileCounts = [
            1, 1, 1, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 1, 1, 1, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 1, 1, 1, // s
            1, 1, 1, 2, 0, 0, 0, // z
        ];
        assert_eq!(counts, expected_counts);
    }

    #[test]
    fn test_from_code_tile_counts_multiple_types() {
        let counts = TileCounts::from_code("11m22p33s44z11m2p7s");
        let expected_counts: TileCounts = [
            4, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 3, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 2, 0, 0, 0, 1, 0, 0, // s
            0, 0, 0, 2, 0, 0, 0, // z
        ];
        assert_eq!(counts, expected_counts);
    }

    #[test]
    fn test_from_code_tile_counts_empty() {
        let counts = TileCounts::from_code("");
        let expected_counts: TileCounts = [0u8; 34];
        assert_eq!(counts, expected_counts);
    }

    #[test]
    #[should_panic(expected = "no type specified before the tile number")]
    fn test_from_code_tile_counts_no_type() {
        TileCounts::from_code("123456");
    }

    #[test]
    #[should_panic(expected = "invalid character in hand")]
    fn test_from_code_invalid_character() {
        TileCounts::from_code("123x");
    }

    #[test]
    #[should_panic(expected = "invalid character in hand")]
    fn test_from_code_tile_counts_offset_out_of_range_number() {
        // 0m does not exist
        TileCounts::from_code("0m");
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_from_code_tile_counts_offset_out_of_range_z() {
        // 8z does not exist
        TileCounts::from_code("8z");
    }

    #[test]
    fn test_from_code_tile_flags_multiple_types() {
        let flags = TileFlags::from_code("11m22p33s44z11m2p7s");
        assert_eq!(flags, 0b0001000_001000100_000000010_000000001);
    }
}
