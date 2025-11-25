// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::hash::{hash_19m, hash_shupai, hash_zipai};
use super::shupai_map::SHUPAI_REPLACEMENT_NUMBER_MAP;
use super::unpack::{UnpackedNumbers, unpack_replacement_number};
use super::wanzi_19_map::WANZI_19_REPLACEMENT_NUMBER_MAP;
use super::zipai_map::ZIPAI_REPLACEMENT_NUMBER_MAP;
use crate::bingpai::{Bingpai, Bingpai3p};
use std::cmp::min;

fn update_dp(lhs: &mut UnpackedNumbers, rhs: &UnpackedNumbers) {
    for i in (5..10).rev() {
        // The original expression is
        // ```
        // let mut r = min(lhs[i] + rhs[0], lhs[0] + rhs[i]);
        // ```
        // However, since lhs[0] and rhs[0] are always 0, the calculation can be omitted.
        let mut r = min(lhs[i], rhs[i]);
        for j in 5..i {
            r = [r, lhs[j] + rhs[i - j], lhs[i - j] + rhs[j]]
                .into_iter()
                .min()
                .unwrap();
        }
        lhs[i] = r;
    }

    // Skip the case when i = 0, as the inner loop would not run, leading to redundant assignments.
    for i in (1..5).rev() {
        // The original expression is
        // ```
        // let mut r = min(lhs[i] + rhs[0], lhs[0] + rhs[i]);
        // ```
        // However, since lhs[0] and rhs[0] are always 0, the calculation can be omitted.
        let mut r = min(lhs[i], rhs[i]);
        for j in 1..i {
            r = min(r, lhs[j] + rhs[i - j]);
        }
        lhs[i] = r;
    }
}

fn update_dp_final(lhs: &mut UnpackedNumbers, rhs: &UnpackedNumbers) {
    for i in (5..10).rev() {
        // The original expression is
        // ```
        // let mut r = min(lhs[i] + rhs[0], lhs[0] + rhs[i]);
        // ```
        // However, since lhs[0] and rhs[0] are always 0, the calculation can be omitted.
        let mut r = min(lhs[i], rhs[i]);
        for j in 5..i {
            r = [r, lhs[j] + rhs[i - j], lhs[i - j] + rhs[j]]
                .into_iter()
                .min()
                .unwrap();
        }
        lhs[i] = r;
    }
}

pub(in super::super) fn calculate_replacement_number(bingpai: &Bingpai) -> u8 {
    let hash_m = hash_shupai(&bingpai.tile_counts()[0..9]);
    let hash_p = hash_shupai(&bingpai.tile_counts()[9..18]);
    let hash_s = hash_shupai(&bingpai.tile_counts()[18..27]);
    let hash_z = hash_zipai(&bingpai.tile_counts()[27..34]);

    let packed_rn_m = &SHUPAI_REPLACEMENT_NUMBER_MAP[hash_m];
    let packed_rn_p = &SHUPAI_REPLACEMENT_NUMBER_MAP[hash_p];
    let packed_rn_s = &SHUPAI_REPLACEMENT_NUMBER_MAP[hash_s];
    let packed_rn_z = &ZIPAI_REPLACEMENT_NUMBER_MAP[hash_z];

    let mut entry0 = unpack_replacement_number(packed_rn_m);
    let entry1 = unpack_replacement_number(packed_rn_p);
    let entry2 = unpack_replacement_number(packed_rn_s);
    let entry3 = unpack_replacement_number(packed_rn_z);

    update_dp(&mut entry0, &entry1);
    update_dp(&mut entry0, &entry2);
    update_dp_final(&mut entry0, &entry3);

    entry0[5 + bingpai.num_required_bingpai_mianzi() as usize] as u8
}

pub(in super::super) fn calculate_replacement_number_3p(bingpai: &Bingpai3p) -> u8 {
    let hash_m = hash_19m(&bingpai.tile_counts()[0..9]);
    let hash_p = hash_shupai(&bingpai.tile_counts()[9..18]);
    let hash_s = hash_shupai(&bingpai.tile_counts()[18..27]);
    let hash_z = hash_zipai(&bingpai.tile_counts()[27..34]);

    let packed_rn_m = &WANZI_19_REPLACEMENT_NUMBER_MAP[hash_m];
    let packed_rn_p = &SHUPAI_REPLACEMENT_NUMBER_MAP[hash_p];
    let packed_rn_s = &SHUPAI_REPLACEMENT_NUMBER_MAP[hash_s];
    let packed_rn_z = &ZIPAI_REPLACEMENT_NUMBER_MAP[hash_z];

    let mut entry0 = unpack_replacement_number(packed_rn_m);
    let entry1 = unpack_replacement_number(packed_rn_p);
    let entry2 = unpack_replacement_number(packed_rn_s);
    let entry3 = unpack_replacement_number(packed_rn_z);

    update_dp(&mut entry0, &entry1);
    update_dp(&mut entry0, &entry2);
    update_dp_final(&mut entry0, &entry3);

    entry0[5 + bingpai.num_required_bingpai_mianzi() as usize] as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::FromTileCode;
    use crate::tile::TileCounts;

    #[test]
    fn calculate_replacement_number_shisanyao_13() {
        let tile_counts = TileCounts::from_code("19m19p19s1234567z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 9);
    }

    #[test]
    fn calculate_replacement_number_shisanyao_14() {
        let tile_counts = TileCounts::from_code("119m19p19s1234567z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 8);
    }

    #[test]
    fn calculate_replacement_number_tenpai() {
        let tile_counts = TileCounts::from_code("123m456p789s1122z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 1);
    }

    #[test]
    fn calculate_replacement_number_win() {
        let tile_counts = TileCounts::from_code("123m456p789s11222z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 0);
    }

    #[test]
    fn calculate_replacement_number_with_meld_exclude() {
        let tile_counts = TileCounts::from_code("123m456p789s2z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 1);
    }

    #[test]
    fn calculate_replacement_number_without_pair() {
        // Source: https://blog.kobalab.net/entry/20151216/1450191666 雀頭がない場合
        let tile_counts = TileCounts::from_code("12389m456p12789s1z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_too_many_meld_candidates() {
        // Source: https://blog.kobalab.net/entry/20151216/1450191666 搭子過多の場合
        let tile_counts = TileCounts::from_code("12389m456p1289s11z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_not_enough_meld_candidates() {
        // Source: https://blog.kobalab.net/entry/20151216/1450191666 搭子不足の場合
        let tile_counts = TileCounts::from_code("133345568m23677z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_triplet_sequence() {
        let tile_counts = TileCounts::from_code("222345p1234567z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 5);
    }

    #[test]
    fn calculate_replacement_number_sequence_isolated_sequence() {
        let tile_counts = TileCounts::from_code("2344456p123456z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 5);
    }

    #[test]
    fn calculate_replacement_number_pair_triplet_sequence() {
        let tile_counts = TileCounts::from_code("11222345p12345z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 4);
    }

    #[test]
    fn calculate_replacement_number_pair_sequence_sequence_pair() {
        let tile_counts = TileCounts::from_code("2234556788p123z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_prioritize_meld_candidates() {
        // Source: https://blog.kobalab.net/entry/2022/04/17/174206 面子の分け方
        let tile_counts = TileCounts::from_code("133345568s11567z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_1() {
        // Source: https://blog.kobalab.net/entry/2022/04/17/174206 5枚目の牌を待つ形
        let tile_counts = TileCounts::from_code("1111m123p112233s");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_2() {
        let tile_counts = TileCounts::from_code("1111234444m1111p");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_3() {
        // Source: http://cmj3.web.fc2.com/#syanten
        let tile_counts = TileCounts::from_code("11112222333444z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_2_isolated_4_tiles_1() {
        let tile_counts = TileCounts::from_code("1111247777m");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_2_isolated_4_tiles_2() {
        let tile_counts = TileCounts::from_code("1111247777m1112z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_2_isolated_4_tiles_3() {
        let tile_counts = TileCounts::from_code("11114444m");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_2_isolated_4_tiles_4() {
        let tile_counts = TileCounts::from_code("111124m1111z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_2_isolated_4_tiles_5() {
        let tile_counts = TileCounts::from_code("1111444478m");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_3_isolated_4_tiles() {
        let tile_counts = TileCounts::from_code("1111247777m1111z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_4_honors_1() {
        let tile_counts = TileCounts::from_code("1111z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_4_honors_2() {
        let tile_counts = TileCounts::from_code("123m1111z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_4_honors_3() {
        let tile_counts = TileCounts::from_code("11112222z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_4_honors_4() {
        let tile_counts = TileCounts::from_code("123m11p11112222z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_different_3p_and_4p() {
        let tile_counts = TileCounts::from_code("1111m111122233z");
        let bingpai = Bingpai::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_3p_different_3p_and_4p() {
        let tile_counts = TileCounts::from_code("1111m111122233z");
        let bingpai = Bingpai3p::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number_3p(&bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_3p_4_19m_1() {
        let tile_counts = TileCounts::from_code("1111m");
        let bingpai = Bingpai3p::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number_3p(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_3p_4_19m_2() {
        let tile_counts = TileCounts::from_code("1111m123p");
        let bingpai = Bingpai3p::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number_3p(&bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_3p_4_19m_3() {
        let tile_counts = TileCounts::from_code("11119999m");
        let bingpai = Bingpai3p::new(&tile_counts).unwrap();
        let replacement_number = calculate_replacement_number_3p(&bingpai);
        assert_eq!(replacement_number, 2);
    }
}
