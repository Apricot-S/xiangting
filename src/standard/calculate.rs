// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::core::{ShupaiBlockCountExt, Wanzi19BlockCountExt, ZipaiBlockCountExt};
use super::count_block::{count_19m_block, count_shupai_block, count_zipai_block};
use crate::bingpai::Bingpai;
use crate::constants::{MAX_NUM_SHOUPAI, NUM_TILE_INDEX};
use crate::shoupai::{FuluMianziList, FuluMianziListExt};
use bitvec::prelude::*;

// Reference: https://blog.kobalab.net/entry/20170917/1505601161
fn calculate_replacement_number_formula(
    num_mianzi: u8,
    mut num_mianzi_candidate: u8,
    mut num_gulipai: u8,
    has_jiangpai: bool,
) -> u8 {
    debug_assert!(num_mianzi <= (MAX_NUM_SHOUPAI / 3));
    debug_assert!(num_mianzi_candidate <= (MAX_NUM_SHOUPAI / 2));
    debug_assert!(num_gulipai <= MAX_NUM_SHOUPAI);

    // Adjust for excess meld candidates
    if (num_mianzi + num_mianzi_candidate) > 4 {
        num_gulipai += num_mianzi + num_mianzi_candidate - 4;
        num_mianzi_candidate = 4 - num_mianzi;
    }

    // Count the pair as a meld candidate if it exists
    if has_jiangpai {
        num_mianzi_candidate += 1;
    }

    // Adjust for excess isolated tiles
    if (num_mianzi + num_mianzi_candidate + num_gulipai) > 5 {
        num_gulipai = 5 - num_mianzi - num_mianzi_candidate;
    }

    14 - num_mianzi * 3 - num_mianzi_candidate * 2 - num_gulipai
}

type AllTileFlag = BitArr!(for NUM_TILE_INDEX, in u64);

#[inline]
fn merge_flags(m: u16, p: u16, s: u16, z: u8) -> AllTileFlag {
    let mut all_color: u64 = 0;
    all_color |= m as u64;
    all_color |= (p as u64) << 9;
    all_color |= (s as u64) << 18;
    all_color |= (z as u64) << 27;
    AllTileFlag::from([all_color; 1])
}

fn calculate_replacement_number_inner(
    bingpai: &mut Bingpai,
    num_fulu: u8,
    four_tiles: AllTileFlag,
    jiangpai: Option<usize>,
) -> u8 {
    let has_jiangpai = jiangpai.is_some();

    let z = count_zipai_block(&bingpai[27..34]);
    let pattern_m = count_shupai_block(&bingpai[0..9]);
    let pattern_p = count_shupai_block(&bingpai[9..18]);
    let pattern_s = count_shupai_block(&bingpai[18..27]);

    let mut min = 14;

    for m in pattern_m {
        for p in pattern_p {
            for s in pattern_s {
                let num_mianzi =
                    num_fulu + m.num_mianzi() + p.num_mianzi() + s.num_mianzi() + z.num_mianzi();
                let mut num_mianzi_candidate = m.num_mianzi_candidate()
                    + p.num_mianzi_candidate()
                    + s.num_mianzi_candidate()
                    + z.num_duizi();
                let mut num_gulipai =
                    m.num_gulipai() + p.num_gulipai() + s.num_gulipai() + z.num_gulipai();

                if four_tiles.any() {
                    let danqi_ting =
                        merge_flags(m.danqi_ting(), p.danqi_ting(), s.danqi_ting(), z.gulipai());

                    // When all pair waits are included in the four tiles
                    if (four_tiles | danqi_ting) == four_tiles {
                        // A tile that is held in a quantity of four cannot become a pair.
                        let four_tiles_danqi_ting = four_tiles & danqi_ting;
                        let num_four_tiles_danqi_ting = four_tiles_danqi_ting.count_ones() as u8;

                        // e.g., 1111234444p1111s: num_gulipai = 2, num_four_tiles_danqi_ting = 3
                        if num_gulipai > num_four_tiles_danqi_ting {
                            num_gulipai -= num_four_tiles_danqi_ting;
                        } else {
                            num_gulipai = 0;
                        }

                        let num_duizi =
                            m.num_duizi() + p.num_duizi() + s.num_duizi() + z.num_duizi();

                        if (has_jiangpai || num_duizi != 0) && four_tiles_danqi_ting[0..27].any() {
                            // One of the isolated suits can become a sequence candidate.
                            num_gulipai += 1;
                        }
                    }

                    let liangmian_ting = merge_flags(
                        m.liangmian_ting(),
                        p.liangmian_ting(),
                        s.liangmian_ting(),
                        0,
                    );
                    let four_tiles_liangmian_ting = four_tiles & liangmian_ting;

                    if four_tiles_liangmian_ting.any() {
                        let num_four_tiles_liangmian_ting =
                            four_tiles_liangmian_ting.count_ones() as u8;
                        num_mianzi_candidate -= num_four_tiles_liangmian_ting;
                        num_gulipai += num_four_tiles_liangmian_ting * 2;
                    }

                    let biankanzhang_ting = merge_flags(
                        m.biankanzhang_ting(),
                        p.biankanzhang_ting(),
                        s.biankanzhang_ting(),
                        0,
                    );
                    let four_tiles_biankanzhang_ting = four_tiles & biankanzhang_ting;

                    if four_tiles_biankanzhang_ting.any() {
                        let num_four_tiles_biankanzhang_ting =
                            four_tiles_biankanzhang_ting.count_ones() as u8;
                        num_mianzi_candidate -= num_four_tiles_biankanzhang_ting;
                        num_gulipai += num_four_tiles_biankanzhang_ting * 2;
                    }

                    let shuangpeng_ting = merge_flags(
                        m.shuangpeng_ting(),
                        p.shuangpeng_ting(),
                        s.shuangpeng_ting(),
                        z.shuangpeng_ting(),
                    );
                    let four_tiles_shuangpeng_ting = four_tiles & shuangpeng_ting;

                    if four_tiles_shuangpeng_ting.any() {
                        // Since the pair has already been removed from the hand, it is not an issue
                        // if this process converts all pairs (triplet candidates) in the hand
                        // into two isolated tiles.
                        let num_four_tiles_shuangpeng_ting =
                            four_tiles_shuangpeng_ting.count_ones() as u8;
                        num_mianzi_candidate -= num_four_tiles_shuangpeng_ting;
                        num_gulipai += num_four_tiles_shuangpeng_ting * 2;
                    }
                }

                let temp = calculate_replacement_number_formula(
                    num_mianzi,
                    num_mianzi_candidate,
                    num_gulipai,
                    has_jiangpai,
                );

                if temp == 0 {
                    return 0;
                }

                if temp < min {
                    min = temp;
                }
            }
        }
    }

    min
}

fn calculate_replacement_number_inner_3_player(
    bingpai: &mut Bingpai,
    num_fulu: u8,
    four_tiles: AllTileFlag,
    jiangpai: Option<usize>,
) -> u8 {
    let has_jiangpai = jiangpai.is_some();

    let z = count_zipai_block(&bingpai[27..34]);
    let m = count_19m_block(&bingpai[0..9]);
    let pattern_p = count_shupai_block(&bingpai[9..18]);
    let pattern_s = count_shupai_block(&bingpai[18..27]);

    let mut min = 14;

    for p in pattern_p {
        for s in pattern_s {
            let num_mianzi =
                num_fulu + m.num_mianzi() + p.num_mianzi() + s.num_mianzi() + z.num_mianzi();
            let mut num_mianzi_candidate =
                m.num_duizi() + p.num_mianzi_candidate() + s.num_mianzi_candidate() + z.num_duizi();
            let mut num_gulipai =
                m.num_gulipai() + p.num_gulipai() + s.num_gulipai() + z.num_gulipai();

            if four_tiles.any() {
                let danqi_ting =
                    merge_flags(m.gulipai(), p.danqi_ting(), s.danqi_ting(), z.gulipai());

                // When all pair waits are included in the four tiles
                if (four_tiles | danqi_ting) == four_tiles {
                    // A tile that is held in a quantity of four cannot become a pair.
                    let four_tiles_danqi_ting = four_tiles & danqi_ting;
                    let num_four_tiles_danqi_ting = four_tiles_danqi_ting.count_ones() as u8;

                    // e.g., 1111234444p1111s: num_gulipai = 2, num_four_tiles_danqi_ting = 3
                    if num_gulipai > num_four_tiles_danqi_ting {
                        num_gulipai -= num_four_tiles_danqi_ting;
                    } else {
                        num_gulipai = 0;
                    }

                    let num_duizi = m.num_duizi() + p.num_duizi() + s.num_duizi() + z.num_duizi();

                    if (has_jiangpai || num_duizi != 0) && four_tiles_danqi_ting[9..27].any() {
                        // One of the isolated suits can become a sequence candidate.
                        num_gulipai += 1;
                    }
                }

                let liangmian_ting = merge_flags(0, p.liangmian_ting(), s.liangmian_ting(), 0);
                let four_tiles_liangmian_ting = four_tiles & liangmian_ting;

                if four_tiles_liangmian_ting.any() {
                    let num_four_tiles_liangmian_ting =
                        four_tiles_liangmian_ting.count_ones() as u8;
                    num_mianzi_candidate -= num_four_tiles_liangmian_ting;
                    num_gulipai += num_four_tiles_liangmian_ting * 2;
                }

                let biankanzhang_ting =
                    merge_flags(0, p.biankanzhang_ting(), s.biankanzhang_ting(), 0);
                let four_tiles_biankanzhang_ting = four_tiles & biankanzhang_ting;

                if four_tiles_biankanzhang_ting.any() {
                    let num_four_tiles_biankanzhang_ting =
                        four_tiles_biankanzhang_ting.count_ones() as u8;
                    num_mianzi_candidate -= num_four_tiles_biankanzhang_ting;
                    num_gulipai += num_four_tiles_biankanzhang_ting * 2;
                }

                let shuangpeng_ting = merge_flags(
                    m.shuangpeng_ting(),
                    p.shuangpeng_ting(),
                    s.shuangpeng_ting(),
                    z.shuangpeng_ting(),
                );
                let four_tiles_shuangpeng_ting = four_tiles & shuangpeng_ting;

                if four_tiles_shuangpeng_ting.any() {
                    // Since the pair has already been removed from the hand, it is not an issue
                    // if this process converts all pairs (triplet candidates) in the hand
                    // into two isolated tiles.
                    let num_four_tiles_shuangpeng_ting =
                        four_tiles_shuangpeng_ting.count_ones() as u8;
                    num_mianzi_candidate -= num_four_tiles_shuangpeng_ting;
                    num_gulipai += num_four_tiles_shuangpeng_ting * 2;
                }
            }

            let temp = calculate_replacement_number_formula(
                num_mianzi,
                num_mianzi_candidate,
                num_gulipai,
                has_jiangpai,
            );

            if temp == 0 {
                return 0;
            }

            if temp < min {
                min = temp;
            }
        }
    }

    min
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

#[inline]
fn calculate_num_fulu(num_bingpai: u8) -> u8 {
    match num_bingpai {
        13 | 14 => 0,
        10 | 11 => 1,
        7 | 8 => 2,
        4 | 5 => 3,
        1 | 2 => 4,
        _ => panic!("Invalid hand"),
    }
}

trait BingpaiExt {
    fn has_duizi(&self, n: usize) -> bool;
    fn remove_duizi(&mut self, n: usize);
    fn restore_duizi(&mut self, n: usize);
}

impl BingpaiExt for Bingpai {
    #[inline]
    fn has_duizi(&self, n: usize) -> bool {
        self[n] >= 2
    }

    #[inline]
    fn remove_duizi(&mut self, n: usize) {
        self[n] -= 2;
    }

    #[inline]
    fn restore_duizi(&mut self, n: usize) {
        self[n] += 2;
    }
}

pub(in super::super) fn calculate_replacement_number(
    mut bingpai: Bingpai,
    fulu_mianzi_list: &Option<FuluMianziList>,
    num_bingpai: u8,
) -> u8 {
    let num_fulu = calculate_num_fulu(num_bingpai);
    debug_assert!(
        num_fulu
            >= fulu_mianzi_list
                .as_ref()
                .map_or(0, |f| f.iter().flatten().count()) as u8
    );

    let four_tiles = count_4_tiles_in_shoupai(&bingpai, fulu_mianzi_list);

    // Calculate the replacement number without a pair
    let mut min = calculate_replacement_number_inner(&mut bingpai, num_fulu, four_tiles, None);
    if min == 0 {
        return 0;
    }

    // Remove a possible pair and calculate the replacement number with a pair
    for n in 0..NUM_TILE_INDEX {
        if bingpai.has_duizi(n) {
            bingpai.remove_duizi(n);
            let temp =
                calculate_replacement_number_inner(&mut bingpai, num_fulu, four_tiles, Some(n));
            bingpai.restore_duizi(n);

            if temp == 0 {
                return 0;
            }

            if temp < min {
                min = temp;
            }
        }
    }

    min
}

pub(in super::super) fn calculate_replacement_number_3_player(
    mut bingpai: Bingpai,
    fulu_mianzi_list: &Option<FuluMianziList>,
    num_bingpai: u8,
) -> u8 {
    let num_fulu = calculate_num_fulu(num_bingpai);
    debug_assert!(
        num_fulu
            >= fulu_mianzi_list
                .as_ref()
                .map_or(0, |f| f.iter().flatten().count()) as u8
    );

    let four_tiles = count_4_tiles_in_shoupai(&bingpai, fulu_mianzi_list);

    // Calculate the replacement number without a pair
    let mut min =
        calculate_replacement_number_inner_3_player(&mut bingpai, num_fulu, four_tiles, None);
    if min == 0 {
        return 0;
    }

    // Remove a possible pair and calculate the replacement number with a pair
    for n in 0..NUM_TILE_INDEX {
        if bingpai.has_duizi(n) {
            bingpai.remove_duizi(n);
            let temp = calculate_replacement_number_inner_3_player(
                &mut bingpai,
                num_fulu,
                four_tiles,
                Some(n),
            );
            bingpai.restore_duizi(n);

            if temp == 0 {
                return 0;
            }

            if temp < min {
                min = temp;
            }
        }
    }

    min
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ClaimedTilePosition, FuluMianzi};

    #[test]
    fn calculate_replacement_number_formula_works() {
        assert_eq!(calculate_replacement_number_formula(0, 0, 14, false), 9);
        assert_eq!(calculate_replacement_number_formula(4, 0, 0, true), 0);
        assert_eq!(calculate_replacement_number_formula(3, 1, 0, true), 1);
        assert_eq!(calculate_replacement_number_formula(4, 1, 0, false), 1);
        assert_eq!(calculate_replacement_number_formula(4, 0, 2, false), 1);
    }

    #[test]
    #[should_panic]
    fn calculate_replacement_number_empty() {
        let bingpai: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, // m
            0, 0, 0, 0, 0, 0, 0, 0, 0, // p
            0, 0, 0, 0, 0, 0, 0, 0, 0, // s
            0, 0, 0, 0, 0, 0, 0, // z
        ];
        let num_bingpai: u8 = bingpai.iter().sum();
        calculate_replacement_number(bingpai, &None, num_bingpai);
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
    #[should_panic]
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
