// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::bingpai::{Bingpai, Bingpai3p};
use crate::tile::TileFlags;

pub(in super::super) fn calculate_necessary_tiles(bingpai: &Bingpai) -> (u8, TileFlags) {
    if bingpai.num_required_bingpai_mianzi() < 4 {
        return (u8::MAX, 0);
    }

    let (num_duizi, num_kinds, waits, wait_candidates) =
        bingpai.tile_counts().iter().enumerate().fold(
            (0, 0, 0u64, 0u64),
            |(num_duizi, num_kinds, waits, wait_candidates), (i, &count)| match count {
                0 => (num_duizi, num_kinds, waits, wait_candidates | (1 << i)),
                1 => (num_duizi, num_kinds + 1, waits | (1 << i), wait_candidates),
                2..=4 => (num_duizi + 1, num_kinds + 1, waits, wait_candidates),
                _ => unreachable!("tile {i} count must be 4 or less but was {count}"),
            },
        );

    let replacement_number = 7 - num_duizi + 7u8.saturating_sub(num_kinds);

    let necessary_tiles = if num_kinds < 7 {
        waits | wait_candidates
    } else {
        waits
    };

    (replacement_number, necessary_tiles)
}

pub(in super::super) fn calculate_necessary_tiles_3p(bingpai: &Bingpai3p) -> (u8, TileFlags) {
    if bingpai.num_required_bingpai_mianzi() < 4 {
        return (u8::MAX, 0);
    }

    let (num_duizi, num_kinds, waits, wait_candidates) = bingpai
        .tile_counts()
        .iter()
        .enumerate()
        .filter(|(i, _)| !matches!(i, 1..=7))
        .fold(
            (0, 0, 0u64, 0u64),
            |(num_duizi, num_kinds, waits, wait_candidates), (i, &count)| match count {
                0 => (num_duizi, num_kinds, waits, wait_candidates | (1 << i)),
                1 => (num_duizi, num_kinds + 1, waits | (1 << i), wait_candidates),
                2..=4 => (num_duizi + 1, num_kinds + 1, waits, wait_candidates),
                _ => unreachable!("tile {i} count must be 4 or less but was {count}"),
            },
        );

    let replacement_number = 7 - num_duizi + 7u8.saturating_sub(num_kinds);

    let necessary_tiles = if num_kinds < 7 {
        waits | wait_candidates
    } else {
        waits
    };

    (replacement_number, necessary_tiles)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::FromTileCode;
    use crate::tile::{TileCounts, TileFlags};

    #[test]
    fn calculate_necessary_tiles_without_pair() {
        let tile_counts = TileCounts::from_code("19m19p19s1234567z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles(&bingpai);
        assert_eq!(replacement_number, 7);
        assert_eq!(necessary_tiles, TileFlags::from_code("19m19p19s1234567z"));
    }

    #[test]
    fn calculate_necessary_tiles_with_quadruple() {
        let tile_counts = TileCounts::from_code("1188m288p55s1111z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles(&bingpai);
        assert_eq!(replacement_number, 3);
        assert_eq!(
            necessary_tiles,
            TileFlags::from_code("2345679m12345679p12346789s234567z")
        );
    }

    #[test]
    fn calculate_necessary_tiles_with_triplet() {
        let tile_counts = TileCounts::from_code("1188m2388p55s111z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles(&bingpai);
        assert_eq!(replacement_number, 2);
        assert_eq!(necessary_tiles, TileFlags::from_code("23p"));
    }

    #[test]
    fn calculate_necessary_tiles_with_2_triplets() {
        let tile_counts = TileCounts::from_code("1188m288p555s111z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles(&bingpai);
        assert_eq!(replacement_number, 3);
        assert_eq!(
            necessary_tiles,
            TileFlags::from_code("2345679m12345679p12346789s234567z")
        );
    }

    #[test]
    fn calculate_necessary_tiles_tenpai() {
        let tile_counts = TileCounts::from_code("1188m288p55s1177z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles(&bingpai);
        assert_eq!(replacement_number, 1);
        assert_eq!(necessary_tiles, TileFlags::from_code("2p"));
    }

    #[test]
    fn calculate_necessary_tiles_win() {
        let tile_counts = TileCounts::from_code("1188m2288p55s1177z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles(&bingpai);
        assert_eq!(replacement_number, 0);
        assert_eq!(necessary_tiles, TileFlags::from_code(""));
    }

    #[test]
    fn calculate_necessary_tiles_incomplete_hand() {
        let tile_counts = TileCounts::from_code("1188m55s1122z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles(&bingpai);
        assert_eq!(replacement_number, u8::MAX);
        assert_eq!(necessary_tiles, TileFlags::from_code(""));
    }

    #[test]
    fn calculate_necessary_tiles_3p_with_quadruple() {
        let tile_counts = TileCounts::from_code("1199m288p55s1111z");
        let bingpai = Bingpai3p::new(&tile_counts).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles_3p(&bingpai);
        assert_eq!(replacement_number, 3);
        assert_eq!(
            necessary_tiles,
            TileFlags::from_code("12345679p12346789s234567z")
        );
    }
}
