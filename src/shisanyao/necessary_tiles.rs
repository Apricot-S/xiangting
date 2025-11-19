// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::common::YAOJIUPAI_INDICES;
use crate::shoupai::Shoupai;
use crate::tile::TileFlags;

pub(in super::super) fn calculate_necessary_tiles(shoupai: &Shoupai) -> (u8, TileFlags) {
    if shoupai.num_required_bingpai_mianzi() < 4 {
        return (u8::MAX, 0);
    }

    let (num_kinds, has_jiangpai, waits, wait_candidates) = YAOJIUPAI_INDICES
        .iter()
        .map(|&i| (i, &shoupai.bingpai()[i]))
        .fold(
            (0, false, 0u64, 0u64),
            |(num_kinds, has_jiangpai, waits, wait_candidates), (i, &count)| match count {
                0 => (num_kinds, has_jiangpai, waits | (1 << i), wait_candidates),
                1 => (
                    num_kinds + 1,
                    has_jiangpai,
                    waits,
                    wait_candidates | (1 << i),
                ),
                2..=4 => (num_kinds + 1, true, waits, wait_candidates),
                _ => unreachable!("tile {i} count must be 4 or less but was {count}"),
            },
        );

    let replacement_number = 14 - num_kinds - (has_jiangpai as u8);

    let necessary_tiles = if has_jiangpai {
        waits
    } else {
        waits | wait_candidates
    };

    (replacement_number, necessary_tiles)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::FromTileCode;
    use crate::tile::{TileCounts, TileFlags};

    #[test]
    fn calculate_necessary_tiles_no_terminals_and_honors() {
        let bingpai = TileCounts::from_code("23455m345p45678s");
        let shoupai = Shoupai::new(&bingpai).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles(&shoupai);
        assert_eq!(replacement_number, 14);
        assert_eq!(necessary_tiles, TileFlags::from_code("19m19p19s1234567z"));
    }

    #[test]
    fn calculate_necessary_tiles_without_pair() {
        let bingpai = TileCounts::from_code("189m12p249s12345z");
        let shoupai = Shoupai::new(&bingpai).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles(&shoupai);
        assert_eq!(replacement_number, 5);
        assert_eq!(necessary_tiles, TileFlags::from_code("19m19p19s1234567z"));
    }

    #[test]
    fn calculate_necessary_tiles_with_pair() {
        let bingpai = TileCounts::from_code("119m12p299s12345z");
        let shoupai = Shoupai::new(&bingpai).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles(&shoupai);
        assert_eq!(replacement_number, 4);
        assert_eq!(necessary_tiles, TileFlags::from_code("9p1s67z"));
    }

    #[test]
    fn calculate_necessary_tiles_tenpai() {
        let bingpai = TileCounts::from_code("11m19p19s1234567z");
        let shoupai = Shoupai::new(&bingpai).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles(&shoupai);
        assert_eq!(replacement_number, 1);
        assert_eq!(necessary_tiles, TileFlags::from_code("9m"));
    }

    #[test]
    fn calculate_necessary_tiles_tenpai_13_wait() {
        let bingpai = TileCounts::from_code("19m19p19s1234567z");
        let shoupai = Shoupai::new(&bingpai).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles(&shoupai);
        assert_eq!(replacement_number, 1);
        assert_eq!(necessary_tiles, TileFlags::from_code("19m19p19s1234567z"));
    }

    #[test]
    fn calculate_necessary_tiles_win() {
        let bingpai = TileCounts::from_code("119m19p19s1234567z");
        let shoupai = Shoupai::new(&bingpai).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles(&shoupai);
        assert_eq!(replacement_number, 0);
        assert_eq!(necessary_tiles, TileFlags::from_code(""));
    }

    #[test]
    fn calculate_necessary_tiles_incomplete_hand() {
        let bingpai = TileCounts::from_code("19p19s1234567z");
        let shoupai = Shoupai::new(&bingpai).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles(&shoupai);
        assert_eq!(replacement_number, u8::MAX);
        assert_eq!(necessary_tiles, TileFlags::from_code(""));
    }
}
