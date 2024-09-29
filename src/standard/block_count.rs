// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::tile_flag::{to_flag, SingleColorTileFlag};
use bitvec::prelude::*;

pub(super) struct BlockCount {
    pub(super) num_mianzi: u8,
    pub(super) num_dazi: u8,
    pub(super) num_duizi: u8,
    pub(super) num_gulipai: u8,
    pub(super) gulipai: SingleColorTileFlag,
    pub(super) four_tiles_gulipai: SingleColorTileFlag,
}

pub(super) struct BlockCountPattern {
    pub(super) a: BlockCount, // Pattern with the minimum number of isolated tiles
    pub(super) b: BlockCount, // Pattern with the maximum number of melds
}

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

pub(super) fn count_shupai_block(
    single_color_bingpai: &mut [u8],
    n: usize,
    jiangpai: Option<usize>,
    single_color_four_tiles: &BitSlice,
) -> BlockCountPattern {
    if n > 8 {
        let num_gulipai = single_color_bingpai.iter().sum();
        let gulipai = to_flag(single_color_bingpai);
        let mut four_tiles_gulipai = SingleColorTileFlag::ZERO;
        four_tiles_gulipai[0..9].copy_from_bitslice(&single_color_four_tiles[0..9]);
        four_tiles_gulipai &= gulipai;

        return BlockCountPattern {
            a: BlockCount {
                num_mianzi: 0,
                num_dazi: 0,
                num_duizi: 0,
                num_gulipai,
                gulipai,
                four_tiles_gulipai,
            },
            b: BlockCount {
                num_mianzi: 0,
                num_dazi: 0,
                num_duizi: 0,
                num_gulipai,
                gulipai,
                four_tiles_gulipai,
            },
        };
    }

    let mut max = count_shupai_block(
        single_color_bingpai,
        n + 1,
        jiangpai,
        single_color_four_tiles,
    );

    #[inline]
    fn update_max(max: &mut BlockCountPattern, r: BlockCountPattern) {
        if (r.a.num_gulipai < max.a.num_gulipai)
            || (r.a.num_gulipai == max.a.num_gulipai)
                && ((r.a.num_dazi + r.a.num_duizi) < (max.a.num_dazi + max.a.num_duizi))
        {
            max.a = r.a;
        }

        if (r.b.num_mianzi > max.b.num_mianzi)
            || ((r.b.num_mianzi == max.b.num_mianzi)
                && ((r.b.num_dazi + r.b.num_duizi) > (max.b.num_dazi + max.b.num_duizi)))
            || ((r.b.num_mianzi == max.b.num_mianzi)
                && ((r.b.num_dazi + r.b.num_duizi) == (max.b.num_dazi + max.b.num_duizi))
                && (r.b.four_tiles_gulipai.count_ones() < max.b.four_tiles_gulipai.count_ones()))
        {
            max.b = r.b;
        }
    }

    if (n <= 6) && single_color_bingpai.has_shunzi(n) {
        single_color_bingpai.remove_shunzi(n);
        let mut r = count_shupai_block(single_color_bingpai, n, jiangpai, single_color_four_tiles);
        single_color_bingpai.restore_shunzi(n);

        r.a.num_mianzi += 1;
        r.b.num_mianzi += 1;

        update_max(&mut max, r);
    }

    if single_color_bingpai.has_kezi(n) {
        single_color_bingpai.remove_kezi(n);
        let mut r = count_shupai_block(single_color_bingpai, n, jiangpai, single_color_four_tiles);
        single_color_bingpai.restore_kezi(n);

        r.a.num_mianzi += 1;
        r.b.num_mianzi += 1;

        update_max(&mut max, r);
    }

    if (n <= 6) && single_color_bingpai.has_qianzhang_dazi(n) && !single_color_four_tiles[n + 1] {
        single_color_bingpai.remove_qianzhang_dazi(n);
        let mut r = count_shupai_block(single_color_bingpai, n, jiangpai, single_color_four_tiles);
        single_color_bingpai.restore_qianzhang_dazi(n);

        r.a.num_dazi += 1;
        r.b.num_dazi += 1;

        update_max(&mut max, r);
    }

    if (n <= 7) && single_color_bingpai.has_liangmen_dazi(n) {
        let is_wait_consumed_by_hand = match n {
            0 => single_color_four_tiles[2],
            1..=6 => single_color_four_tiles[n - 1] && single_color_four_tiles[n + 2],
            7 => single_color_four_tiles[6],
            _ => unreachable!("Invalid rank"),
        };

        if !is_wait_consumed_by_hand {
            single_color_bingpai.remove_liangmen_dazi(n);
            let mut r =
                count_shupai_block(single_color_bingpai, n, jiangpai, single_color_four_tiles);
            single_color_bingpai.restore_liangmen_dazi(n);

            r.a.num_dazi += 1;
            r.b.num_dazi += 1;

            update_max(&mut max, r);
        }
    }

    // There is a possibility of extracting a pair twice from the four tiles,
    // but since the replacement number is greater than the pattern of
    // a triplet and an isolated tile, it is not practically an issue.
    if single_color_bingpai.has_duizi(n) && Some(n) != jiangpai {
        single_color_bingpai.remove_duizi(n);
        let mut r = count_shupai_block(single_color_bingpai, n, jiangpai, single_color_four_tiles);
        single_color_bingpai.restore_duizi(n);

        r.a.num_duizi += 1;
        r.b.num_duizi += 1;

        update_max(&mut max, r);
    }

    max
}

pub(super) fn count_zipai_block(
    zipai_bingpai: &[u8],
    jiangpai: Option<usize>,
    zipai_four_tiles: &BitSlice,
) -> BlockCount {
    zipai_bingpai.iter().enumerate().fold(
        BlockCount {
            num_mianzi: 0,
            num_dazi: 0,
            num_duizi: 0,
            num_gulipai: 0,
            gulipai: SingleColorTileFlag::ZERO,
            four_tiles_gulipai: SingleColorTileFlag::ZERO,
        },
        |mut acc, (i, &n)| {
            match n {
                4 => {
                    acc.num_mianzi += 1;
                    acc.num_gulipai += 1;
                    acc.gulipai.set(i, true);
                    debug_assert!(zipai_four_tiles[i]);
                    acc.four_tiles_gulipai.set(i, true);
                }
                3 => {
                    debug_assert!(!zipai_four_tiles[i]);
                    acc.num_mianzi += 1;
                }
                2 => {
                    if Some(i) != jiangpai {
                        debug_assert!(!zipai_four_tiles[i]);
                        acc.num_duizi += 1;
                    }
                }
                1 => {
                    acc.num_gulipai += 1;
                    acc.gulipai.set(i, true);

                    if zipai_four_tiles[i] {
                        acc.four_tiles_gulipai.set(i, true);
                    }
                }
                0 => (),
                _ => panic!("There are 5 or more of the same tiles: {} tiles", n),
            }

            acc
        },
    )
}

pub(super) fn count_19m_block(
    wanzi_bingpai: &[u8],
    jiangpai: Option<usize>,
    wanzi_four_tiles: &BitSlice,
) -> BlockCount {
    wanzi_bingpai.iter().enumerate().fold(
        BlockCount {
            num_mianzi: 0,
            num_dazi: 0,
            num_duizi: 0,
            num_gulipai: 0,
            gulipai: SingleColorTileFlag::ZERO,
            four_tiles_gulipai: SingleColorTileFlag::ZERO,
        },
        |mut acc, (i, &n)| {
            if i == 0 || i == 8 {
                match n {
                    4 => {
                        acc.num_mianzi += 1;
                        acc.num_gulipai += 1;
                        acc.gulipai.set(i, true);
                        debug_assert!(wanzi_four_tiles[i]);
                        acc.four_tiles_gulipai.set(i, true);
                    }
                    3 => {
                        debug_assert!(!wanzi_four_tiles[i]);
                        acc.num_mianzi += 1;
                    }
                    2 => {
                        if Some(i) != jiangpai {
                            debug_assert!(!wanzi_four_tiles[i]);
                            acc.num_duizi += 1;
                        }
                    }
                    1 => {
                        acc.num_gulipai += 1;
                        acc.gulipai.set(i, true);

                        if wanzi_four_tiles[i] {
                            acc.four_tiles_gulipai.set(i, true);
                        }
                    }
                    0 => (),
                    _ => panic!("There are 5 or more of the same tiles: {} tiles", n),
                }
            }
            acc
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_shupai_block_works() {
        let mut single_color_bingpai = [1, 0, 3, 1, 2, 1, 0, 1, 0];
        let single_color_four_tiles = SingleColorTileFlag::ZERO;
        let r = count_shupai_block(
            &mut single_color_bingpai,
            0,
            None,
            &single_color_four_tiles[0..9],
        );
        assert_eq!(r.a.num_mianzi, 1);
        assert_eq!(r.a.num_dazi, 3);
        assert_eq!(r.a.num_duizi, 0);
        assert_eq!(r.a.num_gulipai, 0);
        assert_eq!(r.b.num_mianzi, 2);
        assert_eq!(r.b.num_dazi, 0);
        assert_eq!(r.b.num_duizi, 0);
        assert_eq!(r.b.num_gulipai, 3);
    }
}
