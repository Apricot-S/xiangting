// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

//! Hand generation for three-player mahjong.
//!
//! Reference:
//!
//! <https://gist.github.com/tomohxx/fc0b72e3fe68744a6ac56a64a41ab8d7>
//! <https://gist.github.com/Apricot-S/f59903a44909206ad9e79390665f7253>

use std::cmp::min;

use xiangting::TileCounts;

const MAX_NUM_SAME_TILE: usize = 4;
const NUM_TILE_INDEX_3P: usize = 27;

pub const NUM_HANDS_3P: [u64; 15] = [
    1,
    27,
    378,
    3_654,
    27_405,
    169_884,
    905_463,
    4_261_842,
    18_057_546,
    69_867_525,
    249_599_610,
    830_534_445,
    2_592_262_530,
    7_633_490_490,
    21_310_147_575,
];

// N is the number of tiles in the hand + 1.
// table[i][n]
// i = 0, 1, ..., 27
// n = 0, 1, ..., N (N = 1, ..., 14)
type Table<const N: usize> = [[u64; N]; NUM_TILE_INDEX_3P + 1];

#[must_use]
pub fn build_table_3p<const N: usize>() -> Table<N> {
    debug_assert!((1..=15).contains(&N));

    let mut table: Table<N> = [[0; N]; NUM_TILE_INDEX_3P + 1];
    table[NUM_TILE_INDEX_3P][N - 1] = 1;

    for i in (0..NUM_TILE_INDEX_3P).rev() {
        for n in 0..=min(MAX_NUM_SAME_TILE * i, N - 1) {
            for c in 0..=min(N - 1 - n, MAX_NUM_SAME_TILE) {
                table[i][n] += table[i + 1][n + c];
            }
        }
    }

    debug_assert_eq!(table[0][0], NUM_HANDS_3P[N - 1]);

    table
}

#[must_use]
pub fn decode_3p<const N: usize>(hash: u64, table: &Table<N>) -> TileCounts {
    let mut hand_3p = [0u8; NUM_TILE_INDEX_3P];
    let mut h = 0;
    let mut n = 0;

    for i in 0..NUM_TILE_INDEX_3P {
        for c in 0..=4 {
            let hh = table[i + 1][n];

            if h + hh <= hash {
                h += hh;
                n += 1;
            } else {
                hand_3p[i] = c;
                break;
            }
        }
    }

    let hand: TileCounts = std::array::from_fn(|i| match i {
        0 => hand_3p[0],
        1..=7 => 0,
        8..=33 => hand_3p[i - 7],
        _ => unreachable!(),
    });

    debug_assert_eq!(hand.iter().sum::<u8>() as usize, N - 1);

    hand
}

#[inline]
fn to_count34(hand: &[usize]) -> TileCounts {
    hand.iter().fold([0u8; 34], |mut counts, &t| {
        counts[t] += 1;
        counts
    })
}

fn generate_hands_3p<const N: usize>(expected_first: TileCounts, expected_last: TileCounts) {
    let table = build_table_3p::<N>();

    let first_hand = decode_3p(NUM_HANDS_3P[N - 1] - 1, &table);
    let last_hand = decode_3p(0, &table);

    assert_eq!(first_hand, expected_first);
    assert_eq!(last_hand, expected_last);
}

#[test]
fn generate_hands_3p_01() {
    let expected_first = to_count34(&[0]);
    let expected_last = to_count34(&[33]);
    generate_hands_3p::<2>(expected_first, expected_last);
}

#[test]
fn generate_hands_3p_02() {
    let expected_first = to_count34(&[0, 0]);
    let expected_last = to_count34(&[33, 33]);
    generate_hands_3p::<3>(expected_first, expected_last);
}

#[test]
fn generate_hands_3p_03() {
    let expected_first = to_count34(&[0, 0, 0]);
    let expected_last = to_count34(&[33, 33, 33]);
    generate_hands_3p::<4>(expected_first, expected_last);
}

#[test]
fn generate_hands_3p_04() {
    let expected_first = to_count34(&[0, 0, 0, 0]);
    let expected_last = to_count34(&[33, 33, 33, 33]);
    generate_hands_3p::<5>(expected_first, expected_last);
}

#[test]
fn generate_hands_3p_05() {
    let expected_first = to_count34(&[0, 0, 0, 0, 8]);
    let expected_last = to_count34(&[32, 33, 33, 33, 33]);
    generate_hands_3p::<6>(expected_first, expected_last);
}

#[test]
fn generate_hands_3p_06() {
    let expected_first = to_count34(&[0, 0, 0, 0, 8, 8]);
    let expected_last = to_count34(&[32, 32, 33, 33, 33, 33]);
    generate_hands_3p::<7>(expected_first, expected_last);
}

#[test]
fn generate_hands_3p_07() {
    let expected_first = to_count34(&[0, 0, 0, 0, 8, 8, 8]);
    let expected_last = to_count34(&[32, 32, 32, 33, 33, 33, 33]);
    generate_hands_3p::<8>(expected_first, expected_last);
}

#[test]
fn generate_hands_3p_08() {
    let expected_first = to_count34(&[0, 0, 0, 0, 8, 8, 8, 8]);
    let expected_last = to_count34(&[32, 32, 32, 32, 33, 33, 33, 33]);
    generate_hands_3p::<9>(expected_first, expected_last);
}

#[test]
fn generate_hands_3p_09() {
    let expected_first = to_count34(&[0, 0, 0, 0, 8, 8, 8, 8, 9]);
    let expected_last = to_count34(&[31, 32, 32, 32, 32, 33, 33, 33, 33]);
    generate_hands_3p::<10>(expected_first, expected_last);
}

#[test]
fn generate_hands_3p_10() {
    let expected_first = to_count34(&[0, 0, 0, 0, 8, 8, 8, 8, 9, 9]);
    let expected_last = to_count34(&[31, 31, 32, 32, 32, 32, 33, 33, 33, 33]);
    generate_hands_3p::<11>(expected_first, expected_last);
}

#[test]
fn generate_hands_3p_11() {
    let expected_first = to_count34(&[0, 0, 0, 0, 8, 8, 8, 8, 9, 9, 9]);
    let expected_last = to_count34(&[31, 31, 31, 32, 32, 32, 32, 33, 33, 33, 33]);
    generate_hands_3p::<12>(expected_first, expected_last);
}

#[test]
fn generate_hands_3p_12() {
    let expected_first = to_count34(&[0, 0, 0, 0, 8, 8, 8, 8, 9, 9, 9, 9]);
    let expected_last = to_count34(&[31, 31, 31, 31, 32, 32, 32, 32, 33, 33, 33, 33]);
    generate_hands_3p::<13>(expected_first, expected_last);
}

#[test]
fn generate_hands_3p_13() {
    let expected_first = to_count34(&[0, 0, 0, 0, 8, 8, 8, 8, 9, 9, 9, 9, 10]);
    let expected_last = to_count34(&[30, 31, 31, 31, 31, 32, 32, 32, 32, 33, 33, 33, 33]);
    generate_hands_3p::<14>(expected_first, expected_last);
}

#[test]
fn generate_hands_3p_14() {
    let expected_first = to_count34(&[0, 0, 0, 0, 8, 8, 8, 8, 9, 9, 9, 9, 10, 10]);
    let expected_last = to_count34(&[30, 30, 31, 31, 31, 31, 32, 32, 32, 32, 33, 33, 33, 33]);
    generate_hands_3p::<15>(expected_first, expected_last);
}
