// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use xiangting::{PlayerCount, calculate_replacement_number};

fn main() {
    // 123m456p789s11222z
    let hand: [u8; 34] = [
        1, 1, 1, 0, 0, 0, 0, 0, 0, // m
        0, 0, 0, 1, 1, 1, 0, 0, 0, // p
        0, 0, 0, 0, 0, 0, 1, 1, 1, // s
        2, 3, 0, 0, 0, 0, 0, // z
    ];

    let replacement_number = calculate_replacement_number(&hand, &PlayerCount::Four);
    assert_eq!(replacement_number.unwrap(), 0u8);
}
