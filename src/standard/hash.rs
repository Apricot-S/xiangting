// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use super::shupai_table::SHUPAI_TABLE;
use super::wanzi_19_table::WANZI_19_TABLE;
use super::zipai_table::ZIPAI_TABLE;

pub fn hash_shupai(single_color_bingpai: &[u8]) -> usize {
    let (hash, _) = single_color_bingpai
        .iter()
        .enumerate()
        .fold((0, 0), |(h, n), (i, &c)| {
            debug_assert!(i < 9);
            debug_assert!(c <= 4);
            debug_assert!(n + c <= 14);
            let updated_n = n + c;
            let updated_h = h + SHUPAI_TABLE[i][updated_n as usize][c as usize];
            (updated_h, updated_n)
        });
    hash
}

pub fn hash_zipai(zipai_bingpai: &[u8]) -> usize {
    let (hash, _) = zipai_bingpai
        .iter()
        .enumerate()
        .fold((0, 0), |(h, n), (i, &c)| {
            debug_assert!(i < 7);
            debug_assert!(c <= 4);
            debug_assert!(n + c <= 14);
            let updated_n = n + c;
            let updated_h = h + ZIPAI_TABLE[i][updated_n as usize][c as usize];
            (updated_h, updated_n)
        });
    hash
}

pub fn hash_19m(wanzi_bingpai: &[u8]) -> usize {
    let (hash, _) = wanzi_bingpai
        .iter()
        .step_by(8)
        .enumerate()
        .fold((0, 0), |(h, n), (i, &c)| {
            debug_assert!(i < 2);
            debug_assert!(c <= 4);
            debug_assert!(n + c <= 8);
            let updated_n = n + c;
            let updated_h = h + WANZI_19_TABLE[i][updated_n as usize][c as usize];
            (updated_h, updated_n)
        });
    hash
}

#[cfg(test)]
mod tests {
    use super::super::shupai_table::SHUPAI_SIZE;
    use super::super::wanzi_19_table::WANZI_19_SIZE;
    use super::super::zipai_table::ZIPAI_SIZE;
    use super::*;

    fn test_hash<const N: usize>(hand: &[u8; N], check: &mut [u8]) {
        let h = match N {
            9 => {
                let h = hash_shupai(hand);
                assert!(h < SHUPAI_SIZE, "Out of range.");
                h
            }
            7 => {
                let h = hash_zipai(hand);
                assert!(h < ZIPAI_SIZE, "Out of range.");
                h
            }
            2 => {
                let full_hand = [hand[0], 0, 0, 0, 0, 0, 0, 0, hand[1]];
                let h = hash_19m(&full_hand);
                assert!(h < WANZI_19_SIZE, "Out of range.");
                h
            }
            _ => unreachable!(),
        };

        assert!(check[h] == 0, "Collision.");
        check[h] += 1;
    }

    fn build_hand<const N: usize>(i: usize, hand: &mut [u8; N], n: u8, check: &mut [u8]) {
        assert!([9, 7, 2].contains(&N));
        assert!(i <= N);
        assert!(n <= 14);

        if i == N {
            test_hash(hand, check);
            return;
        }

        assert!(hand[i] == 0);

        for c in 0..=4 {
            if n + c > 14 {
                break;
            }
            hand[i] = c;
            build_hand(i + 1, hand, n + c, check);
            hand[i] = 0;
        }
    }

    #[test]
    fn test_hash_shupai() {
        let mut hand = [0u8; 9];
        let mut check = [0u8; SHUPAI_SIZE];
        build_hand(0, &mut hand, 0, &mut check);
        assert!(check.iter().all(|&c| c == 1), "A logic error.");
    }

    #[test]
    fn test_hash_zipai() {
        let mut hand = [0u8; 7];
        let mut check = [0u8; ZIPAI_SIZE];
        build_hand(0, &mut hand, 0, &mut check);
        assert!(check.iter().all(|&c| c == 1), "A logic error.");
    }

    #[test]
    fn test_hash_19m() {
        let mut hand = [0u8; 2];
        let mut check = [0u8; WANZI_19_SIZE];
        build_hand(0, &mut hand, 0, &mut check);
        assert!(check.iter().all(|&c| c == 1), "A logic error.");
    }
}
