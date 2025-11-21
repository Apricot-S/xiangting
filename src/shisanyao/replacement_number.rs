// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::common::YAOJIUPAI_INDICES;
use crate::bingpai::Bingpai;

pub(in super::super) fn calculate_replacement_number(bingpai: &Bingpai) -> u8 {
    if bingpai.num_required_bingpai_mianzi() < 4 {
        return u8::MAX;
    }

    let (num_kinds, has_jiangpai) = YAOJIUPAI_INDICES
        .iter()
        .map(|&i| &bingpai.tile_counts()[i])
        .filter(|&&count| count > 0)
        .fold((0, false), |(num_kinds, has_jiangpai), &count| {
            (num_kinds + 1, has_jiangpai || count >= 2)
        });

    14 - num_kinds - (has_jiangpai as u8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::FromTileCode;
    use crate::tile::TileCounts;

    #[test]
    fn calculate_replacement_number_no_terminals_and_honors() {
        let tile_counts = TileCounts::from_code("23455m345p45678s");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 14);
    }

    #[test]
    fn calculate_replacement_number_without_pair() {
        let tile_counts = TileCounts::from_code("189m12p249s12345z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 5);
    }

    #[test]
    fn calculate_replacement_number_with_pair() {
        let tile_counts = TileCounts::from_code("119m12p299s12345z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 4);
    }

    #[test]
    fn calculate_replacement_number_tenpai() {
        let tile_counts = TileCounts::from_code("11m19p19s1234567z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 1);
    }

    #[test]
    fn calculate_replacement_number_tenpai_13_wait() {
        let tile_counts = TileCounts::from_code("19m19p19s1234567z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 1);
    }

    #[test]
    fn calculate_replacement_number_win() {
        let tile_counts = TileCounts::from_code("119m19p19s1234567z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 0);
    }

    #[test]
    fn calculate_replacement_number_incomplete_hand() {
        let tile_counts = TileCounts::from_code("19p19s1234567z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, u8::MAX);
    }
}
