// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use xiangting::{PlayerCount, calculate_necessary_tiles, calculate_unnecessary_tiles};

fn main() {
    // 199m146779p12s246z
    let hand: [u8; 34] = [
        1, 0, 0, 0, 0, 0, 0, 0, 2, // m
        1, 0, 0, 1, 0, 1, 2, 0, 1, // p
        1, 1, 0, 0, 0, 0, 0, 0, 0, // s
        0, 1, 0, 1, 0, 1, 0, // z
    ];

    let (replacement_number1, necessary_tiles) =
        calculate_necessary_tiles(&hand, &PlayerCount::Four).unwrap();
    let (replacement_number2, unnecessary_tiles) =
        calculate_unnecessary_tiles(&hand, &PlayerCount::Four).unwrap();

    assert_eq!(replacement_number1, 5);
    assert_eq!(replacement_number1, replacement_number2);
    assert_eq!(necessary_tiles, 0b1111111_100000111_111111111_100000111); // 1239m123456789p1239s1234567z
    assert_eq!(unnecessary_tiles, 0b0101010_000000011_101101001_000000001); // 1m14679p12s246z
}
