// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::bingpai::Bingpai;
use super::constants::NUM_TILE_INDEX;
use super::shoupai::{count_fulupai, FuluMianziList};
use bitvec::prelude::*;

trait BingpaiExt {
    fn has_liangmen_dazi(&self, n: usize) -> bool;
    fn remove_liangmen_dazi(&mut self, n: usize);
    fn restore_liangmen_dazi(&mut self, n: usize);

    fn has_qianzhang_dazi(&self, n: usize) -> bool;
    fn remove_qianzhang_dazi(&mut self, n: usize);
    fn restore_qianzhang_dazi(&mut self, n: usize);

    fn has_shunzi(&self, n: usize) -> bool;
    fn remove_shunzi(&mut self, n: usize);
    fn restore_shunzi(&mut self, n: usize);

    fn has_duizi(&self, n: usize) -> bool;
    fn remove_duizi(&mut self, n: usize);
    fn restore_duizi(&mut self, n: usize);

    fn has_kezi(&self, n: usize) -> bool;
    fn remove_kezi(&mut self, n: usize);
    fn restore_kezi(&mut self, n: usize);
}

impl BingpaiExt for [u8] {
    #[inline]
    fn has_liangmen_dazi(&self, n: usize) -> bool {
        (self[n] > 0) && (self[n + 1] > 0)
    }

    #[inline]
    fn remove_liangmen_dazi(&mut self, n: usize) {
        self[n] -= 1;
        self[n + 1] -= 1;
    }

    #[inline]
    fn restore_liangmen_dazi(&mut self, n: usize) {
        self[n] += 1;
        self[n + 1] += 1;
    }

    #[inline]
    fn has_qianzhang_dazi(&self, n: usize) -> bool {
        (self[n] > 0) && (self[n + 2] > 0)
    }

    #[inline]
    fn remove_qianzhang_dazi(&mut self, n: usize) {
        self[n] -= 1;
        self[n + 2] -= 1;
    }

    #[inline]
    fn restore_qianzhang_dazi(&mut self, n: usize) {
        self[n] += 1;
        self[n + 2] += 1;
    }

    #[inline]
    fn has_shunzi(&self, n: usize) -> bool {
        (self[n] > 0) && (self[n + 1] > 0) && (self[n + 2] > 0)
    }

    #[inline]
    fn remove_shunzi(&mut self, n: usize) {
        self[n] -= 1;
        self[n + 1] -= 1;
        self[n + 2] -= 1;
    }

    #[inline]
    fn restore_shunzi(&mut self, n: usize) {
        self[n] += 1;
        self[n + 1] += 1;
        self[n + 2] += 1;
    }

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

    #[inline]
    fn has_kezi(&self, n: usize) -> bool {
        self[n] >= 3
    }

    #[inline]
    fn remove_kezi(&mut self, n: usize) {
        self[n] -= 3;
    }

    #[inline]
    fn restore_kezi(&mut self, n: usize) {
        self[n] += 3;
    }
}

type SingleColorTileFlag = BitArr!(for 9);

fn to_flag(single_color_bingpai: &[u8]) -> SingleColorTileFlag {
    single_color_bingpai.iter().enumerate().fold(
        SingleColorTileFlag::ZERO,
        |mut flag, (i, &count)| {
            flag.set(i, count > 0);
            flag
        },
    )
}

type AllTileFlag = BitArr!(for NUM_TILE_INDEX);

fn merge_flags(
    m: SingleColorTileFlag,
    p: SingleColorTileFlag,
    s: SingleColorTileFlag,
    z: SingleColorTileFlag,
) -> AllTileFlag {
    let mut all_tiles = AllTileFlag::ZERO;

    all_tiles[0..9].store(m.load::<u16>());
    all_tiles[9..18].store(p.load::<u16>());
    all_tiles[18..27].store(s.load::<u16>());
    all_tiles[27..34].store(z.load::<u8>());

    all_tiles
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
            let fulupai = count_fulupai(f);
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

// Reference: https://blog.kobalab.net/entry/20170917/1505601161
fn calculate_replacement_number_formula(
    num_mianzi: u8,
    mut num_mianzi_candidate: u8,
    mut num_gulipai: u8,
    has_jiangpai: bool,
) -> u8 {
    debug_assert!(num_mianzi <= 4);

    // If there is no pair, 5 blocks are needed
    let num_required_block: u8 = if has_jiangpai { 4 } else { 5 };

    // Adjust for excess meld candidates
    if (num_mianzi + num_mianzi_candidate) > 4 {
        num_gulipai += num_mianzi + num_mianzi_candidate - 4;
        num_mianzi_candidate = 4 - num_mianzi;
    }

    // Adjust for excess isolated tiles
    if (num_mianzi + num_mianzi_candidate + num_gulipai) > num_required_block {
        num_gulipai = num_required_block - num_mianzi - num_mianzi_candidate;
    }

    // Count the pair as a meld candidate if it exists
    if has_jiangpai {
        num_mianzi_candidate += 1;
    }

    14 - num_mianzi * 3 - num_mianzi_candidate * 2 - num_gulipai
}

struct TileGroupCount {
    num_mianzi: u8,
    num_dazi: u8,
    num_duizi: u8,
    num_gulipai: u8,
    gulipai: SingleColorTileFlag,
}

struct TileGroupCountPattern {
    a: TileGroupCount, // Pattern with the minimum number of isolated tiles
    b: TileGroupCount, // Pattern with the maximum number of melds
}

fn count_shupai_tile_group(
    single_color_bingpai: &mut [u8],
    n: usize,
    jiangpai: Option<usize>,
    four_tiles: &BitSlice,
) -> TileGroupCountPattern {
    if n > 8 {
        return TileGroupCountPattern {
            a: TileGroupCount {
                num_mianzi: 0,
                num_dazi: 0,
                num_duizi: 0,
                num_gulipai: single_color_bingpai.iter().sum(),
                gulipai: to_flag(single_color_bingpai),
            },
            b: TileGroupCount {
                num_mianzi: 0,
                num_dazi: 0,
                num_duizi: 0,
                num_gulipai: single_color_bingpai.iter().sum(),
                gulipai: to_flag(single_color_bingpai),
            },
        };
    }

    let mut max = count_shupai_tile_group(single_color_bingpai, n + 1, jiangpai, four_tiles);

    #[inline]
    fn update_max(max: &mut TileGroupCountPattern, r: TileGroupCountPattern) {
        if (r.a.num_gulipai < max.a.num_gulipai)
            || (r.a.num_gulipai == max.a.num_gulipai)
                && ((r.a.num_dazi + r.a.num_duizi) < (max.a.num_dazi + max.a.num_duizi))
        {
            max.a = r.a;
        }
        if (r.b.num_mianzi > max.b.num_mianzi)
            || (r.b.num_mianzi == max.b.num_mianzi)
                && ((r.b.num_dazi + r.b.num_duizi) > (max.b.num_dazi + max.b.num_duizi))
        {
            max.b = r.b;
        }
    }

    if (n <= 6) && single_color_bingpai.has_shunzi(n) {
        single_color_bingpai.remove_shunzi(n);
        let mut r = count_shupai_tile_group(single_color_bingpai, n, jiangpai, four_tiles);
        single_color_bingpai.restore_shunzi(n);

        r.a.num_mianzi += 1;
        r.b.num_mianzi += 1;

        update_max(&mut max, r);
    }

    if single_color_bingpai.has_kezi(n) {
        single_color_bingpai.remove_kezi(n);
        let mut r = count_shupai_tile_group(single_color_bingpai, n, jiangpai, four_tiles);
        single_color_bingpai.restore_kezi(n);

        r.a.num_mianzi += 1;
        r.b.num_mianzi += 1;

        update_max(&mut max, r);
    }

    if (n <= 6) && single_color_bingpai.has_qianzhang_dazi(n) {
        single_color_bingpai.remove_qianzhang_dazi(n);
        let mut r = count_shupai_tile_group(single_color_bingpai, n, jiangpai, four_tiles);
        single_color_bingpai.restore_qianzhang_dazi(n);

        if !four_tiles[n + 1] {
            r.a.num_dazi += 1;
            r.b.num_dazi += 1;
        }

        update_max(&mut max, r);
    }

    if (n <= 7) && single_color_bingpai.has_liangmen_dazi(n) {
        single_color_bingpai.remove_liangmen_dazi(n);
        let mut r = count_shupai_tile_group(single_color_bingpai, n, jiangpai, four_tiles);
        single_color_bingpai.restore_liangmen_dazi(n);

        let is_wait_consumed_in_hand = match n {
            0 => four_tiles[2],
            1..=6 => four_tiles[n - 1] && four_tiles[n + 2],
            7 => four_tiles[6],
            _ => panic!("Invalid rank"),
        };

        if !is_wait_consumed_in_hand {
            r.a.num_dazi += 1;
            r.b.num_dazi += 1;
        }

        update_max(&mut max, r);
    }

    if single_color_bingpai.has_duizi(n) {
        single_color_bingpai.remove_duizi(n);
        let mut r = count_shupai_tile_group(single_color_bingpai, n, jiangpai, four_tiles);
        single_color_bingpai.restore_duizi(n);

        if Some(n) != jiangpai {
            r.a.num_duizi += 1;
            r.b.num_duizi += 1;
        }

        update_max(&mut max, r);
    }

    max
}

fn count_zipai_tile_group(zipai_bingpai: &[u8], jiangpai: Option<usize>) -> TileGroupCount {
    zipai_bingpai.iter().enumerate().fold(
        TileGroupCount {
            num_mianzi: 0,
            num_dazi: 0,
            num_duizi: 0,
            num_gulipai: 0,
            gulipai: SingleColorTileFlag::ZERO,
        },
        |mut acc, (i, &n)| {
            match n {
                4 => {
                    acc.num_mianzi += 1;
                    acc.num_gulipai += 1;
                    acc.gulipai.set(i, true);
                }
                3 => acc.num_mianzi += 1,
                2 => {
                    if Some(i) != jiangpai {
                        acc.num_duizi += 1;
                    }
                }
                1 => {
                    acc.num_gulipai += 1;
                    acc.gulipai.set(i, true);
                }
                0 => (),
                _ => panic!("There are 5 or more of the same tiles: {} tiles", n),
            }
            acc
        },
    )
}

fn count_19m_tile_group(wanzi_bingpai: &[u8], jiangpai: Option<usize>) -> TileGroupCount {
    wanzi_bingpai.iter().enumerate().fold(
        TileGroupCount {
            num_mianzi: 0,
            num_dazi: 0,
            num_duizi: 0,
            num_gulipai: 0,
            gulipai: SingleColorTileFlag::ZERO,
        },
        |mut acc, (i, &n)| {
            if i == 0 || i == 8 {
                match n {
                    4 => {
                        acc.num_mianzi += 1;
                        acc.num_gulipai += 1;
                        acc.gulipai.set(i, true);
                    }
                    3 => acc.num_mianzi += 1,
                    2 => {
                        if Some(i) != jiangpai {
                            acc.num_duizi += 1;
                        }
                    }
                    1 => {
                        acc.num_gulipai += 1;
                        acc.gulipai.set(i, true);
                    }
                    0 => (),
                    _ => panic!("There are 5 or more of the same tiles: {} tiles", n),
                }
            }
            acc
        },
    )
}

#[inline]
fn offset_jiangpai(jiangpai: Option<usize>, start: usize, upper: usize) -> Option<usize> {
    jiangpai
        .filter(|&value| value >= start && value < upper)
        .map(|value| value - start)
}

fn calculate_replacement_number_inner(
    bingpai: &mut Bingpai,
    num_fulu: u8,
    four_tiles: AllTileFlag,
    jiangpai: Option<usize>,
) -> u8 {
    let has_jiangpai = jiangpai.is_some();
    let jiangpai_m = offset_jiangpai(jiangpai, 0, 9);
    let jiangpai_p = offset_jiangpai(jiangpai, 9, 18);
    let jiangpai_s = offset_jiangpai(jiangpai, 18, 27);
    let jiangpai_z = offset_jiangpai(jiangpai, 27, 34);

    let z = count_zipai_tile_group(&bingpai[27..34], jiangpai_z);
    let pattern_m = count_shupai_tile_group(&mut bingpai[0..9], 0, jiangpai_m, &four_tiles[0..9]);
    let pattern_p = count_shupai_tile_group(&mut bingpai[9..18], 0, jiangpai_p, &four_tiles[9..18]);
    let pattern_s =
        count_shupai_tile_group(&mut bingpai[18..27], 0, jiangpai_s, &four_tiles[18..27]);

    let mut min = 14;

    for m in [&pattern_m.a, &pattern_m.b] {
        for p in [&pattern_p.a, &pattern_p.b] {
            for s in [&pattern_s.a, &pattern_s.b] {
                let num_mianzi =
                    num_fulu + m.num_mianzi + p.num_mianzi + s.num_mianzi + z.num_mianzi;
                let num_dazi = m.num_dazi + p.num_dazi + s.num_dazi + z.num_dazi;
                let num_duizi = m.num_duizi + p.num_duizi + s.num_duizi + z.num_duizi;
                let num_mianzi_candidate = num_dazi + num_duizi;
                let mut num_gulipai = m.num_gulipai + p.num_gulipai + s.num_gulipai + z.num_gulipai;

                if four_tiles.any() {
                    let gulipai = merge_flags(m.gulipai, p.gulipai, s.gulipai, z.gulipai);
                    if gulipai.any() {
                        let four_tiles_gulipai = four_tiles & gulipai;
                        if four_tiles_gulipai.any() {
                            // A tile that is held in a quantity of four
                            // cannot become a pair even if it is isolated.
                            let mut num_four_tiles_gulipai_shupai =
                                four_tiles_gulipai[0..27].count_ones() as u8;

                            if num_mianzi < 4 && num_four_tiles_gulipai_shupai >= 2 {
                                // One of the isolated suits can become a sequence candidate.
                                num_four_tiles_gulipai_shupai -= 1;
                            }

                            num_gulipai -= num_four_tiles_gulipai_shupai;
                            num_gulipai -= four_tiles_gulipai[27..34].count_ones() as u8;
                        }
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
    let jiangpai_m = offset_jiangpai(jiangpai, 0, 9);
    let jiangpai_p = offset_jiangpai(jiangpai, 9, 18);
    let jiangpai_s = offset_jiangpai(jiangpai, 18, 27);
    let jiangpai_z = offset_jiangpai(jiangpai, 27, 34);

    let z = count_zipai_tile_group(&bingpai[27..34], jiangpai_z);
    let m = count_19m_tile_group(&bingpai[0..9], jiangpai_m);
    let pattern_p = count_shupai_tile_group(&mut bingpai[9..18], 0, jiangpai_p, &four_tiles[9..18]);
    let pattern_s =
        count_shupai_tile_group(&mut bingpai[18..27], 0, jiangpai_s, &four_tiles[18..27]);

    let mut min = 14;

    for p in [&pattern_p.a, &pattern_p.b] {
        for s in [&pattern_s.a, &pattern_s.b] {
            let num_mianzi = num_fulu + m.num_mianzi + p.num_mianzi + s.num_mianzi + z.num_mianzi;
            let num_dazi = m.num_dazi + p.num_dazi + s.num_dazi + z.num_dazi;
            let num_duizi = m.num_duizi + p.num_duizi + s.num_duizi + z.num_duizi;
            let num_mianzi_candidate = num_dazi + num_duizi;
            let mut num_gulipai = m.num_gulipai + p.num_gulipai + s.num_gulipai + z.num_gulipai;

            if four_tiles.any() {
                let gulipai = merge_flags(m.gulipai, p.gulipai, s.gulipai, z.gulipai);
                if gulipai.any() {
                    let four_tiles_gulipai = four_tiles & gulipai;
                    if four_tiles_gulipai.any() {
                        // A tile that is held in a quantity of four
                        // cannot become a pair even if it is isolated.
                        let mut num_four_tiles_gulipai_shupai =
                            four_tiles_gulipai[9..27].count_ones() as u8;

                        if num_mianzi < 4 && num_four_tiles_gulipai_shupai >= 2 {
                            // One of the isolated suits can become a sequence candidate.
                            num_four_tiles_gulipai_shupai -= 1;
                        }

                        num_gulipai -= num_four_tiles_gulipai_shupai;
                        num_gulipai -= four_tiles_gulipai[0] as u8;
                        num_gulipai -= four_tiles_gulipai[8] as u8;
                        num_gulipai -= four_tiles_gulipai[27..34].count_ones() as u8;
                    }
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

#[inline]
fn calculate_num_fulu(num_bingpai: u8, fulu_mianzi_list: &Option<FuluMianziList>) -> u8 {
    match fulu_mianzi_list {
        None => match num_bingpai {
            12..=14 => 0,
            9..=11 => 1,
            6..=8 => 2,
            3..=5 => 3,
            1..=2 => 4,
            _ => panic!("Invalid hand: Total tile count exceeds 14."),
        },
        Some(f) => f.iter().flatten().count() as u8,
    }
}

pub(crate) fn calculate_replacement_number(
    mut bingpai: Bingpai,
    fulu_mianzi_list: &Option<FuluMianziList>,
    num_bingpai: u8,
) -> u8 {
    let num_fulu = calculate_num_fulu(num_bingpai, fulu_mianzi_list);
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

pub(crate) fn calculate_replacement_number_3_player(
    mut bingpai: Bingpai,
    fulu_mianzi_list: &Option<FuluMianziList>,
    num_bingpai: u8,
) -> u8 {
    let num_fulu = calculate_num_fulu(num_bingpai, fulu_mianzi_list);
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
    fn count_shupai_tile_group_works() {
        let mut single_color_bingpai = [1, 0, 3, 1, 2, 1, 0, 1, 0];
        let r =
            count_shupai_tile_group(&mut single_color_bingpai, 0, None, &AllTileFlag::ZERO[0..9]);
        assert_eq!(r.a.num_mianzi, 1);
        assert_eq!(r.a.num_dazi, 3);
        assert_eq!(r.a.num_duizi, 0);
        assert_eq!(r.a.num_gulipai, 0);
        assert_eq!(r.b.num_mianzi, 2);
        assert_eq!(r.b.num_dazi, 0);
        assert_eq!(r.b.num_duizi, 0);
        assert_eq!(r.b.num_gulipai, 3);
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

        let bingpai_12: Bingpai = [
            0, 0, 0, 0, 0, 0, 0, 0, 1, // m
            1, 0, 0, 0, 0, 0, 0, 0, 1, // p
            1, 0, 0, 0, 0, 0, 0, 0, 1, // s
            1, 1, 1, 1, 1, 1, 1, // z
        ];
        let num_bingpai_3: u8 = bingpai_12.iter().sum();
        let replacement_number_3 = calculate_replacement_number(bingpai_13, &None, num_bingpai_3);
        assert_eq!(replacement_number_3, 9);
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
