// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::common::YAOJIUPAI_INDICES;
use crate::shoupai::Shoupai;
use crate::tile::TileFlags;

pub(in super::super) fn calculate_unnecessary_tiles(shoupai: &Shoupai) -> (u8, TileFlags) {
    if shoupai.num_required_bingpai_mianzi() < 4 {
        return (u8::MAX, 0);
    }

    let (num_kinds, num_jiangpai, discards, discard_candidates) = YAOJIUPAI_INDICES
        .iter()
        .map(|&i| (i, &shoupai.bingpai()[i]))
        .fold(
            (0, 0, 0u64, 0u64),
            |(num_kinds, num_jiangpai, discards, discard_candidates), (i, &count)| match count {
                0 => (num_kinds, num_jiangpai, discards, discard_candidates),
                1 => (num_kinds + 1, num_jiangpai, discards, discard_candidates),
                2 => (
                    num_kinds + 1,
                    num_jiangpai + 1,
                    discards,
                    discard_candidates | (1 << i),
                ),
                3..=4 => (
                    num_kinds + 1,
                    num_jiangpai + 1,
                    discards | (1 << i),
                    discard_candidates,
                ),
                _ => unreachable!("tile {i} count must be 4 or less but was {count}"),
            },
        );

    const ZHONGZHANGPAI_INDICES: [usize; 21] = [
        1, 2, 3, 4, 5, 6, 7, 10, 11, 12, 13, 14, 15, 16, 19, 20, 21, 22, 23, 24, 25,
    ];
    let discards = ZHONGZHANGPAI_INDICES
        .iter()
        .map(|&i| (i, &shoupai.bingpai()[i]))
        .filter(|(_, count)| **count > 0)
        .fold(discards, |d, (i, _)| d | (1 << i));

    let replacement_number = 14 - num_kinds - (if num_jiangpai > 0 { 1 } else { 0 });

    let unnecessary_tiles = if num_jiangpai >= 2 {
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
    use crate::tile::{TileCounts, TileFlags};

    #[test]
    fn calculate_unnecessary_tiles_no_terminals_and_honors() {
        let bingpai = TileCounts::from_code("23455m345p45678s");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let (replacement_number, unnecessary_tiles) = calculate_unnecessary_tiles(&shoupai);
        assert_eq!(replacement_number, 14);
        assert_eq!(unnecessary_tiles, TileFlags::from_code("23455m345p45678s"));
    }

    #[test]
    fn calculate_unnecessary_tiles_without_pair() {
        let bingpai = TileCounts::from_code("189m12p249s12345z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let (replacement_number, unnecessary_tiles) = calculate_unnecessary_tiles(&shoupai);
        assert_eq!(replacement_number, 5);
        assert_eq!(unnecessary_tiles, TileFlags::from_code("8m2p24s"));
    }

    #[test]
    fn calculate_unnecessary_tiles_with_pair() {
        let bingpai = TileCounts::from_code("119m12p299s12345z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let (replacement_number, unnecessary_tiles) = calculate_unnecessary_tiles(&shoupai);
        assert_eq!(replacement_number, 4);
        assert_eq!(unnecessary_tiles, TileFlags::from_code("1m2p29s"));
    }

    #[test]
    fn calculate_unnecessary_tiles_tenpai() {
        let bingpai = TileCounts::from_code("11m19p19s1234567z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let (replacement_number, unnecessary_tiles) = calculate_unnecessary_tiles(&shoupai);
        assert_eq!(replacement_number, 1);
        assert_eq!(unnecessary_tiles, TileFlags::from_code(""));
    }

    #[test]
    fn calculate_unnecessary_tiles_tenpai_13_wait() {
        let bingpai = TileCounts::from_code("19m19p19s1234567z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let (replacement_number, unnecessary_tiles) = calculate_unnecessary_tiles(&shoupai);
        assert_eq!(replacement_number, 1);
        assert_eq!(unnecessary_tiles, TileFlags::from_code(""));
    }

    #[test]
    fn calculate_unnecessary_tiles_win() {
        let bingpai = TileCounts::from_code("119m19p19s1234567z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let (replacement_number, unnecessary_tiles) = calculate_unnecessary_tiles(&shoupai);
        assert_eq!(replacement_number, 0);
        assert_eq!(unnecessary_tiles, TileFlags::from_code(""));
    }

    #[test]
    fn calculate_unnecessary_tiles_incomplete_hand() {
        let bingpai = TileCounts::from_code("19p19s1234567z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let (replacement_number, unnecessary_tiles) = calculate_unnecessary_tiles(&shoupai);
        assert_eq!(replacement_number, u8::MAX);
        assert_eq!(unnecessary_tiles, TileFlags::from_code(""));
    }
}
