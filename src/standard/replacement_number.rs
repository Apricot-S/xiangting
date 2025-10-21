// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::hash::{hash_19m, hash_shupai, hash_zipai};
use super::shupai_map::{SHUPAI_NECESSARY_TILES_MAP, SHUPAI_REPLACEMENT_NUMBER_MAP};
use super::unpack::{
    UnpackedNumbers, UnpackedTiles, unpack_necessary_tiles, unpack_replacement_number,
};
use super::wanzi_19_map::{WANZI_19_NECESSARY_TILES_MAP, WANZI_19_REPLACEMENT_NUMBER_MAP};
use super::zipai_map::{ZIPAI_NECESSARY_TILES_MAP, ZIPAI_REPLACEMENT_NUMBER_MAP};
use crate::shoupai::{Shoupai, Shoupai3Player};
use crate::tile::TileCounts;
use std::cmp::min;

#[inline]
fn split_flags(all_color: u64) -> (u16, u16, u16, u16) {
    let m = (all_color & 0b111111111) as u16;
    let p = ((all_color >> 9) & 0b111111111) as u16;
    let s = ((all_color >> 18) & 0b111111111) as u16;
    let z = ((all_color >> 27) & 0b1111111) as u16;
    (m, p, s, z)
}

fn count_4_tiles_in_shoupai(shoupai: &TileCounts) -> u64 {
    shoupai
        .iter()
        .enumerate()
        .filter(|&(_, &count)| count == 4)
        .map(|(i, _)| 1 << i)
        .fold(0, |acc, bit| acc | bit)
}

fn modify_number(replacement_number: u8, necessary_tiles: u16, four_tiles: u16) -> u8 {
    const MAX_REPLACEMENT_NUMBER: u8 = 14;
    let remaining_necessary_tiles = necessary_tiles & !four_tiles;

    if replacement_number != 0 && remaining_necessary_tiles == 0 {
        MAX_REPLACEMENT_NUMBER
    } else {
        replacement_number
    }
}

fn modify_numbers(
    numbers: UnpackedNumbers,
    tiles: UnpackedTiles,
    four_tiles: u16,
) -> UnpackedNumbers {
    std::array::from_fn(|i| modify_number(numbers[i], tiles[i], four_tiles))
}

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
        // let mut r = lhs[i] + rhs[0];
        // ```
        // However, since rhs[0] is always 0, the calculation can be omitted.
        let mut r = lhs[i];
        for j in 0..i {
            r = min(r, lhs[j] + rhs[i - j]);
        }
        lhs[i] = r;
    }
}

pub(in super::super) fn calculate_replacement_number(shoupai: &Shoupai) -> u8 {
    let hash_m = hash_shupai(&shoupai.bingpai()[0..9]);
    let hash_p = hash_shupai(&shoupai.bingpai()[9..18]);
    let hash_s = hash_shupai(&shoupai.bingpai()[18..27]);
    let hash_z = hash_zipai(&shoupai.bingpai()[27..34]);

    let packed_rn_m = &SHUPAI_REPLACEMENT_NUMBER_MAP[hash_m];
    let packed_rn_p = &SHUPAI_REPLACEMENT_NUMBER_MAP[hash_p];
    let packed_rn_s = &SHUPAI_REPLACEMENT_NUMBER_MAP[hash_s];
    let packed_rn_z = &ZIPAI_REPLACEMENT_NUMBER_MAP[hash_z];

    let replacement_number_m = unpack_replacement_number(packed_rn_m);
    let replacement_number_p = unpack_replacement_number(packed_rn_p);
    let replacement_number_s = unpack_replacement_number(packed_rn_s);
    let replacement_number_z = unpack_replacement_number(packed_rn_z);

    let (mut entry0, entry1, entry2, entry3) = match shoupai.tile_counts() {
        None => (
            replacement_number_m,
            replacement_number_p,
            replacement_number_s,
            replacement_number_z,
        ),
        Some(s) => {
            let packed_nt_m = &SHUPAI_NECESSARY_TILES_MAP[hash_m];
            let packed_nt_p = &SHUPAI_NECESSARY_TILES_MAP[hash_p];
            let packed_nt_s = &SHUPAI_NECESSARY_TILES_MAP[hash_s];
            let packed_nt_z = &ZIPAI_NECESSARY_TILES_MAP[hash_z];

            let necessary_tiles_m = unpack_necessary_tiles(packed_nt_m);
            let necessary_tiles_p = unpack_necessary_tiles(packed_nt_p);
            let necessary_tiles_s = unpack_necessary_tiles(packed_nt_s);
            let necessary_tiles_z = unpack_necessary_tiles(packed_nt_z);

            let four_tiles = count_4_tiles_in_shoupai(s);
            let (four_tiles_m, four_tiles_p, four_tiles_s, four_tiles_z) = split_flags(four_tiles);

            (
                modify_numbers(replacement_number_m, necessary_tiles_m, four_tiles_m),
                modify_numbers(replacement_number_p, necessary_tiles_p, four_tiles_p),
                modify_numbers(replacement_number_s, necessary_tiles_s, four_tiles_s),
                modify_numbers(replacement_number_z, necessary_tiles_z, four_tiles_z),
            )
        }
    };

    update_dp(&mut entry0, &entry1);
    update_dp(&mut entry0, &entry2);
    update_dp(&mut entry0, &entry3);

    entry0[5 + shoupai.num_required_bingpai_mianzi() as usize]
}

pub(in super::super) fn calculate_replacement_number_3_player(shoupai: &Shoupai3Player) -> u8 {
    let hash_m = hash_19m(&shoupai.bingpai()[0..9]);
    let hash_p = hash_shupai(&shoupai.bingpai()[9..18]);
    let hash_s = hash_shupai(&shoupai.bingpai()[18..27]);
    let hash_z = hash_zipai(&shoupai.bingpai()[27..34]);

    let packed_rn_m = &WANZI_19_REPLACEMENT_NUMBER_MAP[hash_m];
    let packed_rn_p = &SHUPAI_REPLACEMENT_NUMBER_MAP[hash_p];
    let packed_rn_s = &SHUPAI_REPLACEMENT_NUMBER_MAP[hash_s];
    let packed_rn_z = &ZIPAI_REPLACEMENT_NUMBER_MAP[hash_z];

    let replacement_number_m = unpack_replacement_number(packed_rn_m);
    let replacement_number_p = unpack_replacement_number(packed_rn_p);
    let replacement_number_s = unpack_replacement_number(packed_rn_s);
    let replacement_number_z = unpack_replacement_number(packed_rn_z);

    let (mut entry0, entry1, entry2, entry3) = match shoupai.tile_counts() {
        None => (
            replacement_number_m,
            replacement_number_p,
            replacement_number_s,
            replacement_number_z,
        ),
        Some(s) => {
            let packed_nt_m = &WANZI_19_NECESSARY_TILES_MAP[hash_m];
            let packed_nt_p = &SHUPAI_NECESSARY_TILES_MAP[hash_p];
            let packed_nt_s = &SHUPAI_NECESSARY_TILES_MAP[hash_s];
            let packed_nt_z = &ZIPAI_NECESSARY_TILES_MAP[hash_z];

            let necessary_tiles_m = unpack_necessary_tiles(packed_nt_m);
            let necessary_tiles_p = unpack_necessary_tiles(packed_nt_p);
            let necessary_tiles_s = unpack_necessary_tiles(packed_nt_s);
            let necessary_tiles_z = unpack_necessary_tiles(packed_nt_z);

            let four_tiles = count_4_tiles_in_shoupai(s);
            let (four_tiles_m, four_tiles_p, four_tiles_s, four_tiles_z) = split_flags(four_tiles);

            (
                modify_numbers(replacement_number_m, necessary_tiles_m, four_tiles_m),
                modify_numbers(replacement_number_p, necessary_tiles_p, four_tiles_p),
                modify_numbers(replacement_number_s, necessary_tiles_s, four_tiles_s),
                modify_numbers(replacement_number_z, necessary_tiles_z, four_tiles_z),
            )
        }
    };

    update_dp(&mut entry0, &entry1);
    update_dp(&mut entry0, &entry2);
    update_dp(&mut entry0, &entry3);

    entry0[5 + shoupai.num_required_bingpai_mianzi() as usize]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::FromTileCode;
    use crate::{ClaimedTilePosition, FuluMianzi};

    #[test]
    fn calculate_replacement_number_shisanyao_13() {
        let bingpai = TileCounts::from_code("19m19p19s1234567z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 9);
    }

    #[test]
    fn calculate_replacement_number_shisanyao_14() {
        let bingpai = TileCounts::from_code("119m19p19s1234567z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 8);
    }

    #[test]
    fn calculate_replacement_number_tenpai() {
        let bingpai = TileCounts::from_code("123m456p789s1122z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 1);
    }

    #[test]
    fn calculate_replacement_number_win() {
        let bingpai = TileCounts::from_code("123m456p789s11222z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 0);
    }

    #[test]
    fn calculate_replacement_number_with_meld_exclude() {
        let bingpai = TileCounts::from_code("123m456p789s2z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 1);
    }

    #[test]
    fn calculate_replacement_number_with_meld_include() {
        let bingpai = TileCounts::from_code("123m456p789s2z");
        let fulu_mianzi_list = [FuluMianzi::Kezi(27)];
        let shoupai = Shoupai::new(&bingpai, Some(&fulu_mianzi_list)).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 1);
    }

    #[test]
    fn calculate_replacement_number_without_pair() {
        // Source: https://blog.kobalab.net/entry/20151216/1450191666 雀頭がない場合
        let bingpai = TileCounts::from_code("12389m456p12789s1z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_too_many_meld_candidates() {
        // Source: https://blog.kobalab.net/entry/20151216/1450191666 搭子過多の場合
        let bingpai = TileCounts::from_code("12389m456p1289s11z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_not_enough_meld_candidates() {
        // Source: https://blog.kobalab.net/entry/20151216/1450191666 搭子不足の場合
        let bingpai = TileCounts::from_code("133345568m23677z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_incomplete_melds() {
        let bingpai = TileCounts::from_code("123m1z");

        let fulu_mianzi_list = [
            FuluMianzi::Shunzi(12, ClaimedTilePosition::Low),
            FuluMianzi::Gangzi(24),
        ];

        let shoupai = Shoupai::new(&bingpai, Some(&fulu_mianzi_list)).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 1);
    }

    #[test]
    fn calculate_replacement_number_triplet_sequence() {
        let bingpai = TileCounts::from_code("222345p1234567z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 5);
    }

    #[test]
    fn calculate_replacement_number_sequence_isolated_sequence() {
        let bingpai = TileCounts::from_code("2344456p123456z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 5);
    }

    #[test]
    fn calculate_replacement_number_pair_triplet_sequence() {
        let bingpai = TileCounts::from_code("11222345p12345z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 4);
    }

    #[test]
    fn calculate_replacement_number_pair_sequence_sequence_pair() {
        let bingpai = TileCounts::from_code("2234556788p123z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_prioritize_meld_candidates() {
        // Source: https://blog.kobalab.net/entry/2022/04/17/174206 面子の分け方
        let bingpai = TileCounts::from_code("133345568s11567z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_1() {
        // Source: https://blog.kobalab.net/entry/2022/04/17/174206 5枚目の牌を待つ形
        let bingpai = TileCounts::from_code("1111m123p112233s");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_2() {
        let bingpai = TileCounts::from_code("1111234444m1111p");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_3() {
        // Source: http://cmj3.web.fc2.com/#syanten
        let bingpai = TileCounts::from_code("11112222333444z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_4() {
        // Pair wait for a tile already called as a pon
        let bingpai = TileCounts::from_code("123m456p789s1z");
        let fulu_mianzi_list = [FuluMianzi::Kezi(27)];
        let shoupai = Shoupai::new(&bingpai, Some(&fulu_mianzi_list)).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_5() {
        // Middle wait for a tile already called as a kan
        let bingpai = TileCounts::from_code("13m456p789s11z");
        let fulu_mianzi_list = [FuluMianzi::Gangzi(1)];
        let shoupai = Shoupai::new(&bingpai, Some(&fulu_mianzi_list)).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_6() {
        // Edge wait for a tile already called as a kan (12-3)
        let bingpai = TileCounts::from_code("123m12p789s11z");
        let fulu_mianzi_list = [FuluMianzi::Gangzi(11)];
        let shoupai = Shoupai::new(&bingpai, Some(&fulu_mianzi_list)).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_7() {
        // Edge wait for a tile already called as a kan (7-89)
        let bingpai = TileCounts::from_code("123m123p89s11z");
        let fulu_mianzi_list = [FuluMianzi::Gangzi(24)];
        let shoupai = Shoupai::new(&bingpai, Some(&fulu_mianzi_list)).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_8() {
        // Open wait for tiles already called as kans
        let bingpai = TileCounts::from_code("23m456p11z");
        let fulu_mianzi_list = [FuluMianzi::Gangzi(0), FuluMianzi::Gangzi(3)];
        let shoupai = Shoupai::new(&bingpai, Some(&fulu_mianzi_list)).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_9() {
        // Middle wait for a tile already called as a kan with a isolated 4th tile
        let bingpai = TileCounts::from_code("13333m11z");
        let fulu_mianzi_list = [FuluMianzi::Gangzi(1), FuluMianzi::Gangzi(3)];
        let shoupai = Shoupai::new(&bingpai, Some(&fulu_mianzi_list)).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_10() {
        // Edge wait for a tile already called as a kan with a isolated 4th tile (12-3)
        let bingpai = TileCounts::from_code("12222m11z");
        let fulu_mianzi_list = [FuluMianzi::Gangzi(2), FuluMianzi::Gangzi(3)];
        let shoupai = Shoupai::new(&bingpai, Some(&fulu_mianzi_list)).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_11() {
        // Edge wait for a tile already called as a kan with a isolated 4th tile (7-89)
        let bingpai = TileCounts::from_code("88889m11z");
        let fulu_mianzi_list = [FuluMianzi::Gangzi(5), FuluMianzi::Gangzi(6)];
        let shoupai = Shoupai::new(&bingpai, Some(&fulu_mianzi_list)).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_12() {
        // Open wait for a tile already called as a kan with a isolated 4th tile
        let bingpai = TileCounts::from_code("23333m11z");
        let fulu_mianzi_list = [FuluMianzi::Gangzi(0), FuluMianzi::Gangzi(3)];
        let shoupai = Shoupai::new(&bingpai, Some(&fulu_mianzi_list)).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_13() {
        // Edge wait for a tile already called as a kan with a isolated 4th tile (12-3)
        // and tiles of meld candidates is 4th tile
        let bingpai = TileCounts::from_code("12p11z");
        let fulu_mianzi_list = [
            FuluMianzi::Kezi(9),
            FuluMianzi::Kezi(10),
            FuluMianzi::Gangzi(11),
        ];
        let shoupai = Shoupai::new(&bingpai, Some(&fulu_mianzi_list)).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_14() {
        let bingpai = TileCounts::from_code("2233p111z");
        let fulu_mianzi_list = [
            FuluMianzi::Shunzi(9, ClaimedTilePosition::Low),
            FuluMianzi::Shunzi(9, ClaimedTilePosition::Low),
        ];
        let shoupai = Shoupai::new(&bingpai, Some(&fulu_mianzi_list)).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_15() {
        let bingpai = TileCounts::from_code("13m13p1s11z");
        let fulu_mianzi_list = [FuluMianzi::Gangzi(1), FuluMianzi::Gangzi(10)];
        let shoupai = Shoupai::new(&bingpai, Some(&fulu_mianzi_list)).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 4);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_16() {
        // A form that can be interpreted as either open wait or edge wait
        let bingpai = TileCounts::from_code("12345m22z");
        let fulu_mianzi_list = [FuluMianzi::Kezi(2), FuluMianzi::Gangzi(5)];
        let shoupai = Shoupai::new(&bingpai, Some(&fulu_mianzi_list)).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_17() {
        // A form that can be interpreted as either middle wait or edge wait
        let bingpai = TileCounts::from_code("12234m22z");
        let fulu_mianzi_list = [FuluMianzi::Kezi(2), FuluMianzi::Gangzi(5)];
        let shoupai = Shoupai::new(&bingpai, Some(&fulu_mianzi_list)).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_waiting_for_the_5th_tile_18() {
        // A suit that cannot be a pair or a joint
        let bingpai = TileCounts::from_code("1111m");
        let fulu_mianzi_list = [
            FuluMianzi::Gangzi(1),
            FuluMianzi::Gangzi(2),
            FuluMianzi::Gangzi(4),
        ];
        let shoupai = Shoupai::new(&bingpai, Some(&fulu_mianzi_list)).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_2_isolated_4_tiles_1() {
        let bingpai = TileCounts::from_code("1111247777m");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_2_isolated_4_tiles_2() {
        let bingpai = TileCounts::from_code("1111247777m1112z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_2_isolated_4_tiles_3() {
        let bingpai = TileCounts::from_code("11114444m");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_2_isolated_4_tiles_4() {
        let bingpai = TileCounts::from_code("111124m1111z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_2_isolated_4_tiles_5() {
        let bingpai = TileCounts::from_code("1111444478m");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_3_isolated_4_tiles() {
        let bingpai = TileCounts::from_code("1111247777m1111z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_4_honors_1() {
        let bingpai = TileCounts::from_code("1111z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_4_honors_2() {
        let bingpai = TileCounts::from_code("123m1111z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_4_honors_3() {
        let bingpai = TileCounts::from_code("11112222z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_4_honors_4() {
        let bingpai = TileCounts::from_code("123m11p11112222z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_different_3_player_and_4_player() {
        let bingpai = TileCounts::from_code("1111m111122233z");
        let shoupai = Shoupai::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_3_player_different_3_player_and_4_player() {
        let bingpai = TileCounts::from_code("1111m111122233z");
        let shoupai = Shoupai3Player::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number_3_player(&shoupai);
        assert_eq!(replacement_number, 3);
    }

    #[test]
    fn calculate_replacement_number_3_player_4_19m_1() {
        let bingpai = TileCounts::from_code("1111m");
        let shoupai = Shoupai3Player::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number_3_player(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_3_player_4_19m_2() {
        let bingpai = TileCounts::from_code("1111m123p");
        let shoupai = Shoupai3Player::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number_3_player(&shoupai);
        assert_eq!(replacement_number, 2);
    }

    #[test]
    fn calculate_replacement_number_3_player_4_19m_3() {
        let bingpai = TileCounts::from_code("11119999m");
        let shoupai = Shoupai3Player::new(&bingpai, None).unwrap();
        let replacement_number = calculate_replacement_number_3_player(&shoupai);
        assert_eq!(replacement_number, 2);
    }
}
