// SPDX-FileCopyrightText: 2025 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

use xiangting::{ClaimedTilePosition, FuluMianzi, calculate_replacement_number};

fn main() {
    // 123m1z
    let hand: [u8; 34] = [
        1, 1, 1, 0, 0, 0, 0, 0, 0, // m
        0, 0, 0, 0, 0, 0, 0, 0, 0, // p
        0, 0, 0, 0, 0, 0, 0, 0, 0, // s
        1, 0, 0, 0, 0, 0, 0, // z
    ];

    // 456p 7777s 111z
    let melds = [
        FuluMianzi::Shunzi(12, ClaimedTilePosition::Low),
        FuluMianzi::Gangzi(24),
        FuluMianzi::Kezi(27),
    ];

    let replacement_number_wo_melds = calculate_replacement_number(&hand, None);
    assert_eq!(replacement_number_wo_melds.unwrap(), 1u8);

    let replacement_number_w_melds = calculate_replacement_number(&hand, Some(&melds));
    assert_eq!(replacement_number_w_melds.unwrap(), 2u8);
}
