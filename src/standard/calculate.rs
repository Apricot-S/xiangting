// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::core::MapValue;
use super::hash::{hash_19m, hash_shupai, hash_zipai};
use super::shupai_map::SHUPAI_MAP;
use super::wanzi_19_map::WANZI_19_MAP;
use super::zipai_map::ZIPAI_MAP;
use crate::bingpai::Bingpai;
use crate::constants::NUM_TILE_INDEX;
use crate::shoupai::{FuluMianziList, FuluMianziListExt};
use bitvec::prelude::*;
use std::cmp::min;

type AllTileFlag = BitArr!(for NUM_TILE_INDEX, in u64);

#[inline]
fn merge_flags(m: u16, p: u16, s: u16, z: u16) -> AllTileFlag {
    let mut all_color: u64 = 0;
    all_color |= m as u64;
    all_color |= (p as u64) << 9;
    all_color |= (s as u64) << 18;
    all_color |= (z as u64) << 27;
    AllTileFlag::from([all_color; 1])
}

fn count_4_tiles_in_shoupai(
    bingpai: &Bingpai,
    fulu_mianzi_list: &Option<FuluMianziList>,
) -> AllTileFlag {
    match fulu_mianzi_list {
        None => {
            bingpai
                .iter()
                .enumerate()
                .fold(AllTileFlag::ZERO, |mut acc, (i, &num_tile_bingpai)| {
                    if num_tile_bingpai == 4 {
                        acc.set(i, true);
                    }
                    acc
                })
        }
        Some(f) => {
            let fulupai = f.to_fulupai();
            bingpai.iter().zip(fulupai.iter()).enumerate().fold(
                AllTileFlag::ZERO,
                |mut acc, (i, (&num_tile_bingpai, &num_tile_fulupai))| {
                    if (num_tile_bingpai + num_tile_fulupai) == 4 {
                        acc.set(i, true);
                    }
                    acc
                },
            )
        }
    }
}

fn add(lhs: &mut MapValue, rhs: &MapValue) {
    for i in (6..10).rev() {
        let mut r = min(lhs[i] + rhs[0], lhs[0] + rhs[i]);
        for j in 5..i {
            r = min(r, min(lhs[j] + rhs[i - j], lhs[i - j] + rhs[j]));
        }
        lhs[i] = r;
    }

    for i in (1..6).rev() {
        let mut r = lhs[i] + rhs[0];
        for j in 0..i {
            r = min(r, lhs[j] + rhs[i - j]);
        }
        lhs[i] = r;
    }
}

pub(in super::super) fn calculate_replacement_number(
    bingpai: Bingpai,
    fulu_mianzi_list: &Option<FuluMianziList>,
    num_bingpai: u8,
) -> u8 {
    let num_required_melds = num_bingpai / 3;
    debug_assert!(
        (4 - num_required_melds)
            >= fulu_mianzi_list
                .as_ref()
                .map_or(0, |f| f.iter().flatten().count()) as u8
    );

    // let four_tiles = count_4_tiles_in_shoupai(&bingpai, fulu_mianzi_list);

    let h0 = hash_shupai(&bingpai[0..9]);
    let mut pack0 = SHUPAI_MAP[h0];

    let h1 = hash_shupai(&bingpai[9..18]);
    let pack1 = SHUPAI_MAP[h1];
    add(&mut pack0, &pack1);

    let h2 = hash_shupai(&bingpai[18..27]);
    let pack2 = SHUPAI_MAP[h2];
    add(&mut pack0, &pack2);

    let h3 = hash_zipai(&bingpai[27..34]);
    let pack3 = ZIPAI_MAP[h3];
    add(&mut pack0, &pack3);

    pack0[5 + num_required_melds as usize]
}

pub(in super::super) fn calculate_replacement_number_3_player(
    bingpai: Bingpai,
    fulu_mianzi_list: &Option<FuluMianziList>,
    num_bingpai: u8,
) -> u8 {
    let num_required_melds = num_bingpai / 3;
    debug_assert!(
        (4 - num_required_melds)
            >= fulu_mianzi_list
                .as_ref()
                .map_or(0, |f| f.iter().flatten().count()) as u8
    );

    //let four_tiles = count_4_tiles_in_shoupai(&bingpai, fulu_mianzi_list);

    let h0 = hash_19m(&bingpai[0..9]);
    let mut pack0 = WANZI_19_MAP[h0];

    let h1 = hash_shupai(&bingpai[9..18]);
    let pack1 = SHUPAI_MAP[h1];
    add(&mut pack0, &pack1);

    let h2 = hash_shupai(&bingpai[18..27]);
    let pack2 = SHUPAI_MAP[h2];
    add(&mut pack0, &pack2);

    let h3 = hash_zipai(&bingpai[27..34]);
    let pack3 = ZIPAI_MAP[h3];
    add(&mut pack0, &pack3);

    pack0[5 + num_required_melds as usize]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ClaimedTilePosition, FuluMianzi};

    #[test]
    fn calculate_replacement_number_empty() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 2); // 1 pairs, 0 melds => min 2
    }

    #[test]
    #[should_panic]
    fn calculate_replacement_number_overdraw() {
        let bingpai: Bingpai = [
            4, 4, 4, 4, 3, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        calculate_replacement_number(bingpai, &None, num_bingpai);
    }

    #[test]
    fn calculate_replacement_number_shisanyao() {
        let bingpai_14: Bingpai = [
            2, 0, 0, 0, 0, 0, 0, 0, 1, // m
            1, 0, 0, 0, 0, 0, 0, 0, 1, // p
            1, 0, 0, 0, 0, 0, 0, 0, 1, // s
            1, 1, 1, 1, 1, 1, 1, // z
        ];
        let num_bingpai_1: u8 = bingpai_14.iter().sum();
        let replacement_number_1 = calculate_replacement_number(bingpai_14, &None, num_bingpai_1);
        assert_eq!(replacement_number_1, 8);

        let bingpai_13: Bingpai = [
            1, 0, 0, 0, 0, 0, 0, 0, 1, // m
            1, 0, 0, 0, 0, 0, 0, 0, 1, // p
            1, 0, 0, 0, 0, 0, 0, 0, 1, // s
            1, 1, 1, 1, 1, 1, 1, // z
        ];
        let num_bingpai_2: u8 = bingpai_13.iter().sum();
        let replacement_number_2 = calculate_replacement_number(bingpai_13, &None, num_bingpai_2);
        assert_eq!(replacement_number_2, 9);
    }

    #[test]
    fn calculate_replacement_number_tenpai() {
        let bingpai: Bingpai = [
            1, 1, 1, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 1, 1, 1, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 1, 1, 1, // s
            2, 2, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 1);
    }

    #[test]
    fn calculate_replacement_number_win() {
        let bingpai: Bingpai = [
            1, 1, 1, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 1, 1, 1, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 1, 1, 1, // s
            2, 3, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 0);
    }

    #[test]
    fn calculate_replacement_number_with_meld() {
        let bingpai: Bingpai = [
            1, 1, 1, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 1, 1, 1, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 1, 1, 1, // s
            0, 1, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number_1 = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number_1, 1);

        let fulu_mianzi_list = Some([Some(FuluMianzi::Kezi(27)), None, None, None]);
        let replacement_number_2 =
            calculate_replacement_number(bingpai, &fulu_mianzi_list, num_bingpai);
        assert_eq!(replacement_number_2, 1);
    }

    #[test]
    fn calculate_replacement_number_without_pair() {
        // Source: https://blog.kobalab.net/entry/20151216/1450191666 雀頭がない場合
        let bingpai: Bingpai = [
            1, 1, 1, 0, 0, 0, 0, 1, 1, // m
            0, 0, 0, 1, 1, 1, 0, 0, 0, // p
            1, 1, 0, 0, 0, 0, 1, 1, 1, // s
            1, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_too_many_meld_candidates() {
        // Source: https://blog.kobalab.net/entry/20151216/1450191666 搭子過多の場合
        let bingpai: Bingpai = [
            1, 1, 1, 0, 0, 0, 0, 1, 1, // m
            0, 0, 0, 1, 1, 1, 0, 0, 0, // p
            1, 1, 0, 0, 0, 0, 0, 1, 1, // s
            2, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_not_enough_meld_candidates() {
        // Source: https://blog.kobalab.net/entry/20151216/1450191666 搭子不足の場合
        let bingpai: Bingpai = [
            1, 0, 3, 1, 2, 1, 0, 1, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 1, 1, 0, 0, 1, 2, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_incomplete_hand_4_melds_without_a_pair() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 1, 1, 1, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 1, 1, 1, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number_1 = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number_1, 2);

        let fulu_mianzi_list = Some([
            Some(FuluMianzi::Kezi(1)),
            Some(FuluMianzi::Shunzi(13, ClaimedTilePosition::Low)),
            None,
            None,
        ]);
        let replacement_number_2 =
            calculate_replacement_number(bingpai, &fulu_mianzi_list, num_bingpai);
        assert_eq!(replacement_number_2, 2);
    }

    #[test]
    fn calculate_replacement_number_incomplete_melds() {
        let bingpai: Bingpai = [
            1, 1, 1, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 0, 0, 0, 0, 0, 0, // z
        ];

        let fulu_mianzi_list = Some([
            Some(FuluMianzi::Shunzi(12, ClaimedTilePosition::Low)),
            Some(FuluMianzi::Gangzi(24)),
            None,
            None,
        ]);

        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number =
            calculate_replacement_number(bingpai, &fulu_mianzi_list, num_bingpai);
        assert_eq!(replacement_number, 1);
    }

    #[test]
    fn calculate_replacement_number_triplet_sequence() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 3, 1, 1, 1, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 1, 1, 1, 1, 1, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 5);
    }

    #[test]
    fn calculate_replacement_number_sequence_isolated_sequence() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 1, 1, 3, 1, 1, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 1, 1, 1, 1, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 5);
    }

    #[test]
    fn calculate_replacement_number_pair_triplet_sequence() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            2, 3, 1, 1, 1, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 1, 1, 1, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 4);
    }

    #[test]
    fn calculate_replacement_number_pair_sequence_sequence_pair() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 2, 1, 1, 2, 1, 1, 2, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            1, 1, 1, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_prioritize_meld_candidates() {
        // Source: https://blog.kobalab.net/entry/2022/04/17/174206 面子の分け方
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            1, 0, 3, 1, 2, 1, 0, 1, 0, // s
            2, 0, 0, 0, 1, 1, 1, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_1() {
        // Source: https://blog.kobalab.net/entry/2022/04/17/174206 5枚目の牌を待つ形
        let bingpai: Bingpai = [
            4, 0, 0, 0, 0, 0, 0, 0, 0, // m
            1, 1, 1, 0, 0, 0, 0, 0, 0, // p
            2, 2, 2, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_2() {
        let bingpai: Bingpai = [
            4, 1, 1, 4, 0, 0, 0, 0, 0, // m
            4, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_3() {
        // Source: http://cmj3.web.fc2.com/#syanten
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            4, 4, 3, 3, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_4() {
        // Pair wait for a tile already called as a pon
        let bingpai: Bingpai = [
            1, 1, 1, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 1, 1, 1, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 1, 1, 1, // s
            1, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number_1 = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number_1, 1);

        let fulu_mianzi_list = Some([Some(FuluMianzi::Kezi(27)), None, None, None]);
        let replacement_number_2 =
            calculate_replacement_number(bingpai, &fulu_mianzi_list, num_bingpai);
        assert_eq!(replacement_number_2, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_5() {
        // Middle wait for a tile already called as a kan
        let bingpai: Bingpai = [
            1, 0, 1, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 1, 1, 1, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 1, 1, 1, // s
            2, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number_1 = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number_1, 1);

        let fulu_mianzi_list = Some([Some(FuluMianzi::Gangzi(1)), None, None, None]);
        let replacement_number_2 =
            calculate_replacement_number(bingpai, &fulu_mianzi_list, num_bingpai);
        assert_eq!(replacement_number_2, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_6() {
        // Edge wait for a tile already called as a kan (12-3)
        let bingpai: Bingpai = [
            1, 1, 1, 0, 0, 0, 0, 0, 0, // m
            1, 1, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 1, 1, 1, // s
            2, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number_1 = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number_1, 1);

        let fulu_mianzi_list = Some([Some(FuluMianzi::Gangzi(11)), None, None, None]);
        let replacement_number_2 =
            calculate_replacement_number(bingpai, &fulu_mianzi_list, num_bingpai);
        assert_eq!(replacement_number_2, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_7() {
        // Edge wait for a tile already called as a kan (7-89)
        let bingpai: Bingpai = [
            1, 1, 1, 0, 0, 0, 0, 0, 0, // m
            1, 1, 1, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 1, 1, // s
            2, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number_1 = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number_1, 1);

        let fulu_mianzi_list = Some([Some(FuluMianzi::Gangzi(24)), None, None, None]);
        let replacement_number_2 =
            calculate_replacement_number(bingpai, &fulu_mianzi_list, num_bingpai);
        assert_eq!(replacement_number_2, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_8() {
        // Open wait for tiles already called as kans
        let bingpai: Bingpai = [
            0, 1, 1, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 1, 1, 1, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            2, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number_1 = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number_1, 1);

        let fulu_mianzi_list = Some([
            Some(FuluMianzi::Gangzi(0)),
            Some(FuluMianzi::Gangzi(3)),
            None,
            None,
        ]);
        let replacement_number_2 =
            calculate_replacement_number(bingpai, &fulu_mianzi_list, num_bingpai);
        assert_eq!(replacement_number_2, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_9() {
        // Middle wait for a tile already called as a kan with a isolated 4th tile
        let bingpai: Bingpai = [
            1, 0, 4, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            2, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number_1 = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number_1, 1);

        let fulu_mianzi_list = Some([
            Some(FuluMianzi::Gangzi(1)),
            Some(FuluMianzi::Gangzi(3)),
            None,
            None,
        ]);
        let replacement_number_2 =
            calculate_replacement_number(bingpai, &fulu_mianzi_list, num_bingpai);
        assert_eq!(replacement_number_2, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_10() {
        // Edge wait for a tile already called as a kan with a isolated 4th tile (12-3)
        let bingpai: Bingpai = [
            1, 4, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            2, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number_1 = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number_1, 1);

        let fulu_mianzi_list = Some([
            Some(FuluMianzi::Gangzi(2)),
            Some(FuluMianzi::Gangzi(3)),
            None,
            None,
        ]);
        let replacement_number_2 =
            calculate_replacement_number(bingpai, &fulu_mianzi_list, num_bingpai);
        assert_eq!(replacement_number_2, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_11() {
        // Edge wait for a tile already called as a kan with a isolated 4th tile (7-89)
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 4, 1, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            2, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number_1 = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number_1, 1);

        let fulu_mianzi_list = Some([
            Some(FuluMianzi::Gangzi(5)),
            Some(FuluMianzi::Gangzi(6)),
            None,
            None,
        ]);
        let replacement_number_2 =
            calculate_replacement_number(bingpai, &fulu_mianzi_list, num_bingpai);
        assert_eq!(replacement_number_2, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_12() {
        // Open wait for a tile already called as a kan with a isolated 4th tile
        let bingpai: Bingpai = [
            0, 1, 4, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            2, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number_1 = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number_1, 1);

        let fulu_mianzi_list = Some([
            Some(FuluMianzi::Gangzi(0)),
            Some(FuluMianzi::Gangzi(3)),
            None,
            None,
        ]);
        let replacement_number_2 =
            calculate_replacement_number(bingpai, &fulu_mianzi_list, num_bingpai);
        assert_eq!(replacement_number_2, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_13() {
        // Edge wait for a tile already called as a kan with a isolated 4th tile (12-3)
        // and tiles of meld candidates is 4th tile
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            1, 1, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            2, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number_1 = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number_1, 1);

        let fulu_mianzi_list = Some([
            Some(FuluMianzi::Kezi(9)),
            Some(FuluMianzi::Kezi(10)),
            Some(FuluMianzi::Gangzi(11)),
            None,
        ]);
        let replacement_number_2 =
            calculate_replacement_number(bingpai, &fulu_mianzi_list, num_bingpai);
        assert_eq!(replacement_number_2, 3);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_14() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 2, 2, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            3, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number_1 = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number_1, 1);

        let fulu_mianzi_list = Some([
            Some(FuluMianzi::Shunzi(9, ClaimedTilePosition::Low)),
            Some(FuluMianzi::Shunzi(9, ClaimedTilePosition::Low)),
            None,
            None,
        ]);
        let replacement_number_2 =
            calculate_replacement_number(bingpai, &fulu_mianzi_list, num_bingpai);
        assert_eq!(replacement_number_2, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_15() {
        let bingpai: Bingpai = [
            1, 0, 1, 0, 0, 0, 0, 0, 0, // m
            1, 0, 1, 0, 0, 0, 0, 0, 0, // p
            1, 0, 0, 0, 0, 0, 0, 0, 0, // s
            2, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number_1 = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number_1, 2);

        let fulu_mianzi_list = Some([
            Some(FuluMianzi::Gangzi(1)),
            Some(FuluMianzi::Gangzi(10)),
            None,
            None,
        ]);
        let replacement_number_2 =
            calculate_replacement_number(bingpai, &fulu_mianzi_list, num_bingpai);
        assert_eq!(replacement_number_2, 4);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_16() {
        // A form that can be interpreted as either open wait or edge wait
        let bingpai: Bingpai = [
            1, 1, 1, 1, 1, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 2, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number_1 = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number_1, 1);

        let fulu_mianzi_list = Some([
            Some(FuluMianzi::Kezi(2)),
            Some(FuluMianzi::Gangzi(5)),
            None,
            None,
        ]);
        let replacement_number_2 =
            calculate_replacement_number(bingpai, &fulu_mianzi_list, num_bingpai);
        assert_eq!(replacement_number_2, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_17() {
        // A form that can be interpreted as either middle wait or edge wait
        let bingpai: Bingpai = [
            1, 2, 1, 1, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 2, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number_1 = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number_1, 1);

        let fulu_mianzi_list = Some([
            Some(FuluMianzi::Kezi(2)),
            Some(FuluMianzi::Gangzi(5)),
            None,
            None,
        ]);
        let replacement_number_2 =
            calculate_replacement_number(bingpai, &fulu_mianzi_list, num_bingpai);
        assert_eq!(replacement_number_2, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_18() {
        // A suit that cannot be a pair or a joint
        let bingpai: Bingpai = [
            4, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number_1 = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number_1, 2);

        let fulu_mianzi_list = Some([
            Some(FuluMianzi::Gangzi(1)),
            Some(FuluMianzi::Gangzi(2)),
            Some(FuluMianzi::Gangzi(4)),
            None,
        ]);
        let replacement_number_2 =
            calculate_replacement_number(bingpai, &fulu_mianzi_list, num_bingpai);
        assert_eq!(replacement_number_2, 2);
    }

    #[test]
    fn calculate_replacement_number_2_isolated_4_tiles_1() {
        let bingpai: Bingpai = [
            4, 1, 0, 1, 0, 0, 4, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_2_isolated_4_tiles_2() {
        let bingpai: Bingpai = [
            4, 1, 0, 1, 0, 0, 4, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            3, 1, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_2_isolated_4_tiles_3() {
        let bingpai: Bingpai = [
            4, 0, 0, 4, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_2_isolated_4_tiles_4() {
        let bingpai: Bingpai = [
            4, 1, 0, 1, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            4, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_2_isolated_4_tiles_5() {
        let bingpai: Bingpai = [
            4, 0, 0, 4, 0, 0, 1, 1, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_3_isolated_4_tiles() {
        let bingpai: Bingpai = [
            4, 1, 0, 1, 0, 0, 4, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            4, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_4_honors_1() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            4, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_4_honors_2() {
        let bingpai: Bingpai = [
            1, 1, 1, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            4, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_4_honors_3() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            4, 4, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_4_honors_4() {
        let bingpai: Bingpai = [
            1, 1, 1, 0, 0, 0, 0, 0, 0, // m
            2, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            4, 4, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_different_3_player_and_4_player() {
        let bingpai: Bingpai = [
            4, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            4, 3, 2, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number_4p = calculate_replacement_number(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number_4p, 2);
    }

    #[test]
    fn calculate_replacement_number_3_player_different_3_player_and_4_player() {
        let bingpai: Bingpai = [
            4, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            4, 3, 2, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number_3p =
            calculate_replacement_number_3_player(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number_3p, 3);
    }

    #[test]
    fn calculate_replacement_number_3_player_4_19m_1() {
        let bingpai: Bingpai = [
            4, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number_3_player(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_3_player_4_19m_2() {
        let bingpai: Bingpai = [
            4, 0, 0, 0, 0, 0, 0, 0, 0, // m
            1, 1, 1, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number_3_player(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_3_player_4_19m_3() {
        let bingpai: Bingpai = [
            4, 0, 0, 0, 0, 0, 0, 0, 4, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        let replacement_number = calculate_replacement_number_3_player(bingpai, &None, num_bingpai);
        assert_eq!(replacement_number, 2);
    }
}
