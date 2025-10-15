// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use crate::shoupai::{Shoupai, Shoupai3Player};
use crate::tile::TileFlags;

pub(in super::super) fn calculate_necessary_tiles(shoupai: &Shoupai) -> (u8, TileFlags) {
    if shoupai.num_required_bingpai_mianzi < 4 {
        return (u8::MAX, 0);
    }

    let (num_duizi, num_kinds, waits, wait_candidates) = shoupai.bingpai.iter().enumerate().fold(
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

pub(in super::super) fn calculate_necessary_tiles_3_player(
    shoupai: &Shoupai3Player,
) -> (u8, TileFlags) {
    if shoupai.num_required_bingpai_mianzi < 4 {
        return (u8::MAX, 0);
    }

    let (num_duizi, num_kinds, waits, wait_candidates) = shoupai
        .bingpai
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
    use crate::test_utils::TileCountsExt;
    use crate::tile::TileCounts;

    #[test]
    fn calculate_necessary_tiles_without_pair() {
        let bingpai = TileCounts::from_code("19m19p19s1234567z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles(&shoupai);
        assert_eq!(replacement_number, 7);
        assert_eq!(necessary_tiles, 0b1111111_100000001_100000001_100000001);
    }

    #[test]
    fn calculate_necessary_tiles_with_quadruple() {
        let bingpai = TileCounts::from_code("1188m288p55s1111z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles(&shoupai);
        assert_eq!(replacement_number, 3);
        assert_eq!(necessary_tiles, 0b1111110_111101111_101111111_101111110);
    }

    #[test]
    fn calculate_necessary_tiles_with_triplet() {
        let bingpai = TileCounts::from_code("1188m2388p55s111z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles(&shoupai);
        assert_eq!(replacement_number, 2);
        assert_eq!(necessary_tiles, 0b0000000_000000000_000000110_000000000);
    }

    #[test]
    fn calculate_necessary_tiles_with_2_triplets() {
        let bingpai = TileCounts::from_code("1188m288p555s111z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles(&shoupai);
        assert_eq!(replacement_number, 3);
        assert_eq!(necessary_tiles, 0b1111110_111101111_101111111_101111110);
    }

    #[test]
    fn calculate_necessary_tiles_tenpai() {
        let bingpai = TileCounts::from_code("1188m288p55s1177z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles(&shoupai);
        assert_eq!(replacement_number, 1);
        assert_eq!(necessary_tiles, 0b0000000_000000000_000000010_000000000);
    }

    #[test]
    fn calculate_necessary_tiles_win() {
        let bingpai = TileCounts::from_code("1188m2288p55s1177z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles(&shoupai);
        assert_eq!(replacement_number, 0);
        assert_eq!(necessary_tiles, 0b0000000_000000000_000000000_000000000);
    }

    #[test]
    fn calculate_necessary_tiles_incomplete_hand() {
        let bingpai = TileCounts::from_code("1188m55s1122z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles(&shoupai);
        assert_eq!(replacement_number, u8::MAX);
        assert_eq!(necessary_tiles, 0b0000000_000000000_000000000_000000000);
    }

    #[test]
    fn calculate_necessary_tiles_3_player_with_quadruple() {
        let bingpai = TileCounts::from_code("1199m288p55s1111z");
        let shoupai = Shoupai3Player::new(&bingpai, None).unwrap();
        let (replacement_number, necessary_tiles) = calculate_necessary_tiles_3_player(&shoupai);
        assert_eq!(replacement_number, 3);
        assert_eq!(necessary_tiles, 0b1111110_111101111_101111111_000000000);
    }
}
