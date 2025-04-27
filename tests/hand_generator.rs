// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

//! Hand generation by decoding the hash value of the N-tile version of Nyanten.
//!
//! Reference:
//!
//! https://gist.github.com/tomohxx/fc0b72e3fe68744a6ac56a64a41ab8d7

use std::cmp::min;
use xiangting::Bingpai;

const MAX_NUM_SAME_TILE: usize = 4;
const NUM_TILE_INDEX: usize = 3 * 9 + 4 + 3;

pub const NUM_HANDS: [u64; 15] = [
    1,
    34,
    595,
    7_140,
    66_045,
    501_908,
    3_261_467,
    18_623_330,
    95_305_485,
    443_646_280,
    1_900_269_316,
    7_558_429_024,
    28_126_474_500,
    98_521_596_000,
    326_520_504_500,
];

// N is the number of tiles in the hand + 1.
// table[i][n]
// i = 0, 1, ..., 34
// n = 0, 1, ..., N (N = 1, ..., 14)
type Table<const N: usize> = [[u64; N]; NUM_TILE_INDEX + 1];

pub fn build_table<const N: usize>() -> Table<N> {
    debug_assert!((1..=15).contains(&N));

    let mut table: Table<N> = [[0; N]; NUM_TILE_INDEX + 1];
    table[NUM_TILE_INDEX][N - 1] = 1;

    for i in (0..NUM_TILE_INDEX).rev() {
        for n in 0..=min(MAX_NUM_SAME_TILE * i, N - 1) {
            for c in 0..=min(N - 1 - n, MAX_NUM_SAME_TILE) {
                table[i][n] += table[i + 1][n + c];
            }
        }
    }

    debug_assert!(table[0][0] == NUM_HANDS[N - 1]);

    table
}

pub fn decode<const N: usize>(hash: u64, table: &Table<N>) -> Bingpai {
    let mut hand: Bingpai = [0; NUM_TILE_INDEX];
    let mut h = 0;
    let mut n = 0;

    for i in 0..NUM_TILE_INDEX {
        for c in 0..=4 {
            let hh = table[i + 1][n];

            if h + hh <= hash {
                h += hh;
                n += 1;
            } else {
                hand[i] = c;
                break;
            }
        }
    }

    debug_assert_eq!(hand.iter().sum::<u8>() as usize, N - 1);

    hand
}

#[cfg(test)]
mod tests {
    use super::*;

    #[inline]
    fn to_count34(hand: &[usize]) -> Bingpai {
        hand.iter().fold([0u8; NUM_TILE_INDEX], |mut counts, &t| {
            counts[t] += 1;
            counts
        })
    }

    fn generate_hands<const N: usize>(expected_first: Bingpai, expected_last: Bingpai) {
        let table = build_table::<N>();

        let first_hand = decode(NUM_HANDS[N - 1] - 1, &table);
        let last_hand = decode(0, &table);

        assert_eq!(first_hand, expected_first);
        assert_eq!(last_hand, expected_last);
    }

    #[test]
    fn generate_hands_01() {
        let expected_first = to_count34(&[0]);
        let expected_last = to_count34(&[33]);
        generate_hands::<2>(expected_first, expected_last);
    }

    #[test]
    fn generate_hands_02() {
        let expected_first = to_count34(&[0, 0]);
        let expected_last = to_count34(&[33, 33]);
        generate_hands::<3>(expected_first, expected_last);
    }

    #[test]
    fn generate_hands_03() {
        let expected_first = to_count34(&[0, 0, 0]);
        let expected_last = to_count34(&[33, 33, 33]);
        generate_hands::<4>(expected_first, expected_last);
    }

    #[test]
    fn generate_hands_04() {
        let expected_first = to_count34(&[0, 0, 0, 0]);
        let expected_last = to_count34(&[33, 33, 33, 33]);
        generate_hands::<5>(expected_first, expected_last);
    }

    #[test]
    fn generate_hands_05() {
        let expected_first = to_count34(&[0, 0, 0, 0, 1]);
        let expected_last = to_count34(&[32, 33, 33, 33, 33]);
        generate_hands::<6>(expected_first, expected_last);
    }

    #[test]
    fn generate_hands_06() {
        let expected_first = to_count34(&[0, 0, 0, 0, 1, 1]);
        let expected_last = to_count34(&[32, 32, 33, 33, 33, 33]);
        generate_hands::<7>(expected_first, expected_last);
    }

    #[test]
    fn generate_hands_07() {
        let expected_first = to_count34(&[0, 0, 0, 0, 1, 1, 1]);
        let expected_last = to_count34(&[32, 32, 32, 33, 33, 33, 33]);
        generate_hands::<8>(expected_first, expected_last);
    }

    #[test]
    fn generate_hands_08() {
        let expected_first = to_count34(&[0, 0, 0, 0, 1, 1, 1, 1]);
        let expected_last = to_count34(&[32, 32, 32, 32, 33, 33, 33, 33]);
        generate_hands::<9>(expected_first, expected_last);
    }

    #[test]
    fn generate_hands_09() {
        let expected_first = to_count34(&[0, 0, 0, 0, 1, 1, 1, 1, 2]);
        let expected_last = to_count34(&[31, 32, 32, 32, 32, 33, 33, 33, 33]);
        generate_hands::<10>(expected_first, expected_last);
    }

    #[test]
    fn generate_hands_10() {
        let expected_first = to_count34(&[0, 0, 0, 0, 1, 1, 1, 1, 2, 2]);
        let expected_last = to_count34(&[31, 31, 32, 32, 32, 32, 33, 33, 33, 33]);
        generate_hands::<11>(expected_first, expected_last);
    }

    #[test]
    fn generate_hands_11() {
        let expected_first = to_count34(&[0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2]);
        let expected_last = to_count34(&[31, 31, 31, 32, 32, 32, 32, 33, 33, 33, 33]);
        generate_hands::<12>(expected_first, expected_last);
    }

    #[test]
    fn generate_hands_12() {
        let expected_first = to_count34(&[0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2]);
        let expected_last = to_count34(&[31, 31, 31, 31, 32, 32, 32, 32, 33, 33, 33, 33]);
        generate_hands::<13>(expected_first, expected_last);
    }

    #[test]
    fn generate_hands_13() {
        let expected_first = to_count34(&[0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3]);
        let expected_last = to_count34(&[30, 31, 31, 31, 31, 32, 32, 32, 32, 33, 33, 33, 33]);
        generate_hands::<14>(expected_first, expected_last);
    }

    #[test]
    fn generate_hands_14() {
        let expected_first = to_count34(&[0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3]);
        let expected_last = to_count34(&[30, 30, 31, 31, 31, 31, 32, 32, 32, 32, 33, 33, 33, 33]);
        generate_hands::<15>(expected_first, expected_last);
    }
}
