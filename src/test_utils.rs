// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::tile::TileCounts;

pub(crate) trait TileCountsExt {
    /// Converts a Tenhou-style tile string into an array representing
    /// the counts of 34 types of tiles.
    fn from_code(hand: &str) -> TileCounts;
}

impl TileCountsExt for TileCounts {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_code_normal() {
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
    fn test_from_code_multiple_types() {
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
    fn test_from_code_empty() {
        let counts = TileCounts::from_code("");
        let expected_counts: TileCounts = [0u8; 34];
        assert_eq!(counts, expected_counts);
    }

    #[test]
    #[should_panic]
    fn test_from_code_no_type() {
        TileCounts::from_code("123456");
    }

    #[test]
    #[should_panic]
    fn test_from_code_offset_out_of_range_number() {
        // 0m does not exist
        TileCounts::from_code("0m");
    }

    #[test]
    #[should_panic]
    fn test_from_code_offset_out_of_range_z() {
        // 8z does not exist
        TileCounts::from_code("8z");
    }
}
