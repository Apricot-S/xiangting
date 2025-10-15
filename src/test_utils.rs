// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::tile::{TileCounts, TileFlags};

pub(crate) trait FromTileCode<T> {
    /// Converts a Tenhou-style tile string into `T`.
    fn from_code(hand: &str) -> T;
}

impl FromTileCode<TileCounts> for TileCounts {
    fn from_code(hand: &str) -> TileCounts {
        let mut current_color: Option<usize> = None;
        let mut result: TileCounts = [0u8; 34];

        for c in hand.chars().rev() {
            match c {
                'm' => current_color = Some(0),
                'p' => current_color = Some(9),
                's' => current_color = Some(18),
                'z' => current_color = Some(27),
                _ => {
                    let d = c.to_digit(10).expect("invalid digit") as usize;
                    let base = current_color.expect("digit without type");
                    if !(1..=9).contains(&d) {
                        panic!("tile number must be 1-9, got {}", d);
                    }
                    if base == 27 && d > 7 {
                        panic!("honor tile must be 1-7, got {}", d);
                    }
                    result[base + d - 1] += 1;
                }
            }
        }

        result
    }
}

impl FromTileCode<TileFlags> for TileFlags {
    fn from_code(hand: &str) -> TileFlags {
        let mut current_color: Option<usize> = None;
        let mut result: TileFlags = 0u64;

        for c in hand.chars().rev() {
            match c {
                'm' => current_color = Some(0),
                'p' => current_color = Some(9),
                's' => current_color = Some(18),
                'z' => current_color = Some(27),
                _ => {
                    let d = c.to_digit(10).expect("invalid digit") as usize;
                    let base = current_color.expect("digit without type");
                    if !(1..=9).contains(&d) {
                        panic!("tile number must be 1-9, got {}", d);
                    }
                    if base == 27 && d > 7 {
                        panic!("honor tile must be 1-7, got {}", d);
                    }
                    result |= 1 << (base + d - 1);
                }
            }
        }

        result
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
    #[should_panic]
    fn test_from_code_tile_counts_no_type() {
        TileCounts::from_code("123456");
    }

    #[test]
    #[should_panic]
    fn test_from_code_tile_counts_offset_out_of_range_number() {
        // 0m does not exist
        TileCounts::from_code("0m");
    }

    #[test]
    #[should_panic]
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
