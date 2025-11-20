// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use xiangting::{PlayerCount, calculate_replacement_number};

fn main() {
    // 1111m111122233z
    let hand: [u8; 34] = [
        4, 0, 0, 0, 0, 0, 0, 0, 0, // m
        0, 0, 0, 0, 0, 0, 0, 0, 0, // p
        0, 0, 0, 0, 0, 0, 0, 0, 0, // s
        4, 3, 2, 0, 0, 0, 0, // z
    ];

    let replacement_number_4p = calculate_replacement_number(&hand, &PlayerCount::Four).unwrap();
    assert_eq!(replacement_number_4p, 2u8);

    let replacement_number_3p = calculate_replacement_number(&hand, &PlayerCount::Three).unwrap();
    assert_eq!(replacement_number_3p, 3u8);
}
