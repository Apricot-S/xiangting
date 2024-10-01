// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::core::{
    to_flag, ShupaiBlockCountImpl, ShupaiBlockCountPatternImpl, SingleColorTileFlag,
};
use xiangting::standard::core::ShupaiBlockCountPattern;

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

fn count_shupai_block_impl(
    single_color_bingpai: &mut [u8; 9],
    n: usize,
) -> ShupaiBlockCountPatternImpl {
    debug_assert!(n <= 9);

    if n > 8 {
        let num_gulipai = single_color_bingpai.iter().sum();
        let gulipai = to_flag(single_color_bingpai);

        return ShupaiBlockCountPatternImpl {
            a: ShupaiBlockCountImpl {
                num_mianzi: 0,
                num_dazi: 0,
                num_duizi: 0,
                num_gulipai,
                gulipai,
                liangmian_ting: SingleColorTileFlag::ZERO,
                biankanzhang_ting: SingleColorTileFlag::ZERO,
                shuangpeng_ting: SingleColorTileFlag::ZERO,
                danqi_ting: gulipai,
            },
            b: ShupaiBlockCountImpl {
                num_mianzi: 0,
                num_dazi: 0,
                num_duizi: 0,
                num_gulipai,
                gulipai,
                liangmian_ting: SingleColorTileFlag::ZERO,
                biankanzhang_ting: SingleColorTileFlag::ZERO,
                shuangpeng_ting: SingleColorTileFlag::ZERO,
                danqi_ting: gulipai,
            },
        };
    }

    let mut max = count_shupai_block_impl(single_color_bingpai, n + 1);

    #[inline]
    fn update_max(max: &mut ShupaiBlockCountPatternImpl, r: ShupaiBlockCountPatternImpl) {
        if (r.a.num_gulipai < max.a.num_gulipai)
            || (r.a.num_gulipai == max.a.num_gulipai)
                && ((r.a.num_dazi + r.a.num_duizi) < (max.a.num_dazi + max.a.num_duizi))
        {
            max.a = r.a;
        }

        if (r.b.num_mianzi > max.b.num_mianzi)
            || ((r.b.num_mianzi == max.b.num_mianzi)
                && ((r.b.num_dazi + r.b.num_duizi) > (max.b.num_dazi + max.b.num_duizi)))
        {
            max.b = r.b;
        }
    }

    if (n <= 6) && single_color_bingpai.has_shunzi(n) {
        single_color_bingpai.remove_shunzi(n);
        let mut r = count_shupai_block_impl(single_color_bingpai, n);
        single_color_bingpai.restore_shunzi(n);

        r.a.num_mianzi += 1;
        r.b.num_mianzi += 1;

        update_max(&mut max, r);
    }

    if single_color_bingpai.has_kezi(n) {
        single_color_bingpai.remove_kezi(n);
        let mut r = count_shupai_block_impl(single_color_bingpai, n);
        single_color_bingpai.restore_kezi(n);

        r.a.num_mianzi += 1;
        r.b.num_mianzi += 1;

        update_max(&mut max, r);
    }

    if (n <= 6) && single_color_bingpai.has_qianzhang_dazi(n) {
        single_color_bingpai.remove_qianzhang_dazi(n);
        let mut r = count_shupai_block_impl(single_color_bingpai, n);
        single_color_bingpai.restore_qianzhang_dazi(n);

        r.a.num_dazi += 1;
        r.b.num_dazi += 1;

        r.a.biankanzhang_ting.set(n + 1, true);
        r.b.biankanzhang_ting.set(n + 1, true);

        update_max(&mut max, r);
    }

    // Edge wait (12-3)
    if (n == 0) && single_color_bingpai.has_liangmen_dazi(n) {
        single_color_bingpai.remove_liangmen_dazi(n);
        let mut r = count_shupai_block_impl(single_color_bingpai, n);
        single_color_bingpai.restore_liangmen_dazi(n);

        r.a.num_dazi += 1;
        r.b.num_dazi += 1;

        r.a.biankanzhang_ting.set(n + 2, true);
        r.b.biankanzhang_ting.set(n + 2, true);

        update_max(&mut max, r);
    }

    // Edge wait (7-89)
    if (n == 7) && single_color_bingpai.has_liangmen_dazi(n) {
        single_color_bingpai.remove_liangmen_dazi(n);
        let mut r = count_shupai_block_impl(single_color_bingpai, n);
        single_color_bingpai.restore_liangmen_dazi(n);

        r.a.num_dazi += 1;
        r.b.num_dazi += 1;

        r.a.biankanzhang_ting.set(n - 1, true);
        r.b.biankanzhang_ting.set(n - 1, true);

        update_max(&mut max, r);
    }

    // Open wait
    if (1 <= n && n <= 6) && single_color_bingpai.has_liangmen_dazi(n) {
        single_color_bingpai.remove_liangmen_dazi(n);
        let mut r = count_shupai_block_impl(single_color_bingpai, n);
        single_color_bingpai.restore_liangmen_dazi(n);

        r.a.num_dazi += 1;
        r.b.num_dazi += 1;

        // Record only the smaller one. e.g., 1 for 23
        r.a.liangmian_ting.set(n - 1, true);
        r.b.liangmian_ting.set(n - 1, true);

        update_max(&mut max, r);
    }

    // There is a possibility of extracting a pair twice from the four tiles,
    // but since the replacement number is greater than the pattern of
    // a triplet and an isolated tile, it is not practically an issue.
    if single_color_bingpai.has_duizi(n) {
        single_color_bingpai.remove_duizi(n);
        let mut r = count_shupai_block_impl(single_color_bingpai, n);
        single_color_bingpai.restore_duizi(n);

        r.a.num_duizi += 1;
        r.b.num_duizi += 1;

        r.a.shuangpeng_ting.set(n, true);
        r.b.shuangpeng_ting.set(n, true);

        update_max(&mut max, r);
    }

    max
}

pub(in super::super) fn count_shupai_block(
    single_color_bingpai: &[u8; 9],
) -> ShupaiBlockCountPattern {
    count_shupai_block_impl(&mut single_color_bingpai.clone(), 0).to_entry()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_shupai_block_works() {
        let mut single_color_bingpai = [1, 0, 3, 1, 2, 1, 0, 1, 0];
        let r = count_shupai_block(&mut single_color_bingpai);
        assert_eq!(r[0].0, 1);
        assert_eq!(r[0].1, 3);
        assert_eq!(r[0].2, 0);
        assert_eq!(r[0].3, 0);
        assert_eq!(r[0].4, 0b000000000);
        assert_eq!(r[0].5, 0b000000000);
        assert_eq!(r[0].6, 0b001001010);
        assert_eq!(r[0].7, 0b000000000);
        assert_eq!(r[0].8, 0b000000000);

        assert_eq!(r[1].0, 2);
        assert_eq!(r[1].1, 0);
        assert_eq!(r[1].2, 0);
        assert_eq!(r[1].3, 3);
        assert_eq!(r[1].4, 0b010010001);
        assert_eq!(r[1].5, 0b000000000);
        assert_eq!(r[1].6, 0b000000000);
        assert_eq!(r[1].7, 0b000000000);
        assert_eq!(r[1].8, 0b010010001);
    }

    #[test]
    fn count_shupai_block_empty() {
        let mut shupai_bingpai = [0, 0, 0, 0, 0, 0, 0, 0, 0];
        let r = count_shupai_block(&mut shupai_bingpai);
        assert_eq!(r[0].0, 0);
        assert_eq!(r[0].1, 0);
        assert_eq!(r[0].2, 0);
        assert_eq!(r[0].3, 0);
        assert_eq!(r[0].4, 0b000000000);
        assert_eq!(r[0].5, 0b000000000);
        assert_eq!(r[0].6, 0b000000000);
        assert_eq!(r[0].7, 0b000000000);
        assert_eq!(r[0].8, 0b000000000);

        assert_eq!(r[1].0, 0);
        assert_eq!(r[1].1, 0);
        assert_eq!(r[1].2, 0);
        assert_eq!(r[1].3, 0);
        assert_eq!(r[1].4, 0b000000000);
        assert_eq!(r[1].5, 0b000000000);
        assert_eq!(r[1].6, 0b000000000);
        assert_eq!(r[1].7, 0b000000000);
        assert_eq!(r[1].8, 0b000000000);
    }
}
