// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use xiangting::{PlayerCount, calculate_necessary_tiles, calculate_unnecessary_tiles};

fn main() {
    // 1111m111122233z
    let hand: [u8; 34] = [
        4, 0, 0, 0, 0, 0, 0, 0, 0, // m
        0, 0, 0, 0, 0, 0, 0, 0, 0, // p
        0, 0, 0, 0, 0, 0, 0, 0, 0, // s
        4, 3, 2, 0, 0, 0, 0, // z
    ];

    let (rn_4p, nt_4p) = calculate_necessary_tiles(&hand, &PlayerCount::Four).unwrap();
    let (_, ut_4p) = calculate_unnecessary_tiles(&hand, &PlayerCount::Four).unwrap();
    assert_eq!(rn_4p, 2u8);
    assert_eq!(nt_4p, 0b0000000_000000000_000000000_000000110); // 23m
    assert_eq!(ut_4p, 0b0000001_000000000_000000000_000000000); // 1z

    let (rn_3p, nt_3p) = calculate_necessary_tiles(&hand, &PlayerCount::Three).unwrap();
    let (_, ut_3p) = calculate_unnecessary_tiles(&hand, &PlayerCount::Three).unwrap();
    assert_eq!(rn_3p, 3u8);
    assert_eq!(nt_3p, 0b1111100_111111111_111111111_100000000); // 9m123456789p123456789s34567z
    assert_eq!(ut_3p, 0b0000001_000000000_000000000_000000001); // 1m1z
}
