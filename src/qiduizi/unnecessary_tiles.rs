// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::bingpai::{Bingpai, Bingpai3p};
use crate::tile::TileFlags;

pub(in super::super) fn calculate_unnecessary_tiles(bingpai: &Bingpai) -> (u8, TileFlags) {
    if bingpai.num_required_bingpai_mianzi() < 4 {
        return (u8::MAX, 0);
    }

    let (num_duizi, num_kinds, discards, discard_candidates) =
        bingpai.tile_counts().iter().enumerate().fold(
            (0, 0, 0u64, 0u64),
            |(num_duizi, num_kinds, discards, discard_candidates), (i, &count)| match count {
                0 => (num_duizi, num_kinds, discards, discard_candidates),
                1 => (
                    num_duizi,
                    num_kinds + 1,
                    discards,
                    discard_candidates | (1 << i),
                ),
                2 => (num_duizi + 1, num_kinds + 1, discards, discard_candidates),
                3..=4 => (
                    num_duizi + 1,
                    num_kinds + 1,
                    discards | (1 << i),
                    discard_candidates,
                ),
                _ => unreachable!("tile {i} count must be 4 or less but was {count}"),
            },
        );

    let replacement_number = 7 - num_duizi + 7u8.saturating_sub(num_kinds);

    let unnecessary_tiles = if num_kinds > 7 {
        discards | discard_candidates
    } else {
        discards
    };

    (replacement_number, unnecessary_tiles)
}

pub(in super::super) fn calculate_unnecessary_tiles_3p(bingpai: &Bingpai3p) -> (u8, TileFlags) {
    if bingpai.num_required_bingpai_mianzi() < 4 {
        return (u8::MAX, 0);
    }

    let (num_duizi, num_kinds, discards, discard_candidates) = bingpai
        .tile_counts()
        .iter()
        .enumerate()
        .filter(|(i, _)| !matches!(i, 1..=7))
        .fold(
            (0, 0, 0u64, 0u64),
            |(num_duizi, num_kinds, discards, discard_candidates), (i, &count)| match count {
                0 => (num_duizi, num_kinds, discards, discard_candidates),
                1 => (
                    num_duizi,
                    num_kinds + 1,
                    discards,
                    discard_candidates | (1 << i),
                ),
                2 => (num_duizi + 1, num_kinds + 1, discards, discard_candidates),
                3..=4 => (
                    num_duizi + 1,
                    num_kinds + 1,
                    discards | (1 << i),
                    discard_candidates,
                ),
                _ => unreachable!("tile {i} count must be 4 or less but was {count}"),
            },
        );

    let replacement_number = 7 - num_duizi + 7u8.saturating_sub(num_kinds);

    let unnecessary_tiles = if num_kinds > 7 {
        discards | discard_candidates
    } else {
        discards
    };

    (replacement_number, unnecessary_tiles)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::FromTileCode;
    use crate::tile::TileCounts;

    #[test]
    fn calculate_unnecessary_tiles_without_pair() {
        let tile_counts = TileCounts::from_code("19m19p19s1234567z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let (replacement_number, unnecessary_tiles) = calculate_unnecessary_tiles(&bingpai);
        assert_eq!(replacement_number, 7);
        assert_eq!(unnecessary_tiles, TileFlags::from_code("19m19p19s1234567z"));
    }

    #[test]
    fn calculate_unnecessary_tiles_with_quadruple() {
        let tile_counts = TileCounts::from_code("1188m288p55s1111z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let (replacement_number, unnecessary_tiles) = calculate_unnecessary_tiles(&bingpai);
        assert_eq!(replacement_number, 3);
        assert_eq!(unnecessary_tiles, TileFlags::from_code("1z"));
    }

    #[test]
    fn calculate_unnecessary_tiles_with_triplet() {
        let tile_counts = TileCounts::from_code("1188m2388p55s111z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let (replacement_number, unnecessary_tiles) = calculate_unnecessary_tiles(&bingpai);
        assert_eq!(replacement_number, 2);
        assert_eq!(unnecessary_tiles, TileFlags::from_code("1z"));
    }

    #[test]
    fn calculate_unnecessary_tiles_with_2_triplets() {
        let tile_counts = TileCounts::from_code("1188m288p555s111z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let (replacement_number, unnecessary_tiles) = calculate_unnecessary_tiles(&bingpai);
        assert_eq!(replacement_number, 3);
        assert_eq!(unnecessary_tiles, TileFlags::from_code("5s1z"));
    }

    #[test]
    fn calculate_unnecessary_tiles_tenpai() {
        let tile_counts = TileCounts::from_code("1188m288p55s1177z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let (replacement_number, unnecessary_tiles) = calculate_unnecessary_tiles(&bingpai);
        assert_eq!(replacement_number, 1);
        assert_eq!(unnecessary_tiles, TileFlags::from_code(""));
    }

    #[test]
    fn calculate_unnecessary_tiles_win() {
        let tile_counts = TileCounts::from_code("1188m2288p55s1177z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let (replacement_number, unnecessary_tiles) = calculate_unnecessary_tiles(&bingpai);
        assert_eq!(replacement_number, 0);
        assert_eq!(unnecessary_tiles, TileFlags::from_code(""));
    }

    #[test]
    fn calculate_unnecessary_tiles_incomplete_hand() {
        let tile_counts = TileCounts::from_code("1188m55s1122z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let (replacement_number, unnecessary_tiles) = calculate_unnecessary_tiles(&bingpai);
        assert_eq!(replacement_number, u8::MAX);
        assert_eq!(unnecessary_tiles, TileFlags::from_code(""));
    }

    #[test]
    fn calculate_unnecessary_tiles_3p_with_quadruple() {
        let tile_counts = TileCounts::from_code("1199m288p55s1111z");
        let bingpai = Bingpai3p::new(&tile_counts).unwrap();
        let (replacement_number, unnecessary_tiles) = calculate_unnecessary_tiles_3p(&bingpai);
        assert_eq!(replacement_number, 3);
        assert_eq!(unnecessary_tiles, TileFlags::from_code("1z"));
    }
}
