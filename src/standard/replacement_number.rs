// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::forward::Forward;
use crate::bingpai::{Bingpai, PlayerRule};

pub(in super::super) fn calculate_replacement_number<R: PlayerRule>(bingpai: &Bingpai<R>) -> u8 {
    Forward::new(bingpai).number
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bingpai::{FourPlayer, ThreePlayer};
    use crate::test_utils::FromTileCode;
    use crate::tile::TileCounts;

    #[test]
    fn calculate_replacement_number_shisanyao_13() {
        let tile_counts = TileCounts::from_code("19m19p19s1234567z");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 9);
    }

    #[test]
    fn calculate_replacement_number_shisanyao_14() {
        let tile_counts = TileCounts::from_code("119m19p19s1234567z");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 8);
    }

    #[test]
    fn calculate_replacement_number_tenpai() {
        let tile_counts = TileCounts::from_code("123m456p789s1122z");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 1);
    }

    #[test]
    fn calculate_replacement_number_win() {
        let tile_counts = TileCounts::from_code("123m456p789s11222z");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 0);
    }

    #[test]
    fn calculate_replacement_number_with_meld_exclude() {
        let tile_counts = TileCounts::from_code("123m456p789s2z");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 1);
    }

    #[test]
    fn calculate_replacement_number_without_pair() {
        // Source: https://blog.kobalab.net/entry/20151216/1450191666 雀頭がない場合
        let tile_counts = TileCounts::from_code("12389m456p12789s1z");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_too_many_meld_candidates() {
        // Source: https://blog.kobalab.net/entry/20151216/1450191666 搭子過多の場合
        let tile_counts = TileCounts::from_code("12389m456p1289s11z");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_not_enough_meld_candidates() {
        // Source: https://blog.kobalab.net/entry/20151216/1450191666 搭子不足の場合
        let tile_counts = TileCounts::from_code("133345568m23677z");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_triplet_sequence() {
        let tile_counts = TileCounts::from_code("222345p1234567z");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 5);
    }

    #[test]
    fn calculate_replacement_number_sequence_isolated_sequence() {
        let tile_counts = TileCounts::from_code("2344456p123456z");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 5);
    }

    #[test]
    fn calculate_replacement_number_pair_triplet_sequence() {
        let tile_counts = TileCounts::from_code("11222345p12345z");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 4);
    }

    #[test]
    fn calculate_replacement_number_pair_sequence_sequence_pair() {
        let tile_counts = TileCounts::from_code("2234556788p123z");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_prioritize_meld_candidates() {
        // Source: https://blog.kobalab.net/entry/2022/04/17/174206 面子の分け方
        let tile_counts = TileCounts::from_code("133345568s11567z");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_1() {
        // Source: https://blog.kobalab.net/entry/2022/04/17/174206 5枚目の牌を待つ形
        let tile_counts = TileCounts::from_code("1111m123p112233s");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_2() {
        let tile_counts = TileCounts::from_code("1111234444m1111p");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_3() {
        // Source: http://cmj3.web.fc2.com/#syanten
        let tile_counts = TileCounts::from_code("11112222333444z");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_2_isolated_4_tiles_1() {
        let tile_counts = TileCounts::from_code("1111247777m");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_2_isolated_4_tiles_2() {
        let tile_counts = TileCounts::from_code("1111247777m1112z");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_2_isolated_4_tiles_3() {
        let tile_counts = TileCounts::from_code("11114444m");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_2_isolated_4_tiles_4() {
        let tile_counts = TileCounts::from_code("111124m1111z");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_2_isolated_4_tiles_5() {
        let tile_counts = TileCounts::from_code("1111444478m");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_3_isolated_4_tiles() {
        let tile_counts = TileCounts::from_code("1111247777m1111z");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_4_honors_1() {
        let tile_counts = TileCounts::from_code("1111z");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_4_honors_2() {
        let tile_counts = TileCounts::from_code("123m1111z");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_4_honors_3() {
        let tile_counts = TileCounts::from_code("11112222z");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_4_honors_4() {
        let tile_counts = TileCounts::from_code("123m11p11112222z");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_different_3p_and_4p() {
        let tile_counts = TileCounts::from_code("1111m111122233z");
        let bingpai = Bingpai::<FourPlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_3p_different_3p_and_4p() {
        let tile_counts = TileCounts::from_code("1111m111122233z");
        let bingpai = Bingpai::<ThreePlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_3p_4_19m_1() {
        let tile_counts = TileCounts::from_code("1111m");
        let bingpai = Bingpai::<ThreePlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_3p_4_19m_2() {
        let tile_counts = TileCounts::from_code("1111m123p");
        let bingpai = Bingpai::<ThreePlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_3p_4_19m_3() {
        let tile_counts = TileCounts::from_code("11119999m");
        let bingpai = Bingpai::<ThreePlayer>::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }
}
