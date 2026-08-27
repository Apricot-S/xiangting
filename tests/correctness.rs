// SPDX-FileCopyrightText: 2024 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

#![cfg(feature = "correctness")]

mod common;
mod hand_generator;

use crate::common::{
    NecessaryTiles, Player, ReplacementNumber, UnnecessaryTiles, VerificationTarget,
};
use crate::hand_generator::{NUM_HANDS, build_table, decode};

fn verify_correctness<const N: usize, T: VerificationTarget>() -> bool {
    let table = build_table::<N>();

    crate::common::verify_correctness::<N, T, Player<false>, _>(NUM_HANDS[N - 1], move |hash| {
        decode(hash, &table)
    })
}

macro_rules! define_correctness_tests {
        ($(($replacement:ident, $necessary:ident, $unnecessary:ident, $n:literal)),+ $(,)?) => {
            $(
                #[test]
                #[ignore = "exhaustive correctness check"]
                fn $replacement() {
                    assert!(
                        verify_correctness::<$n, ReplacementNumber>(),
                        "There were mismatches."
                    );
                }

                #[test]
                #[ignore = "exhaustive correctness check"]
                fn $necessary() {
                    assert!(
                        verify_correctness::<$n, NecessaryTiles>(),
                        "There were mismatches."
                    );
                }

                #[test]
                #[ignore = "exhaustive correctness check"]
                fn $unnecessary() {
                    assert!(
                        verify_correctness::<$n, UnnecessaryTiles>(),
                        "There were mismatches."
                    );
                }
            )+
        };
    }

define_correctness_tests!(
    (
        verify_replacement_number_01,
        verify_necessary_tiles_01,
        verify_unnecessary_tiles_01,
        2
    ),
    (
        verify_replacement_number_02,
        verify_necessary_tiles_02,
        verify_unnecessary_tiles_02,
        3
    ),
    (
        verify_replacement_number_04,
        verify_necessary_tiles_04,
        verify_unnecessary_tiles_04,
        5
    ),
    (
        verify_replacement_number_05,
        verify_necessary_tiles_05,
        verify_unnecessary_tiles_05,
        6
    ),
    (
        verify_replacement_number_07,
        verify_necessary_tiles_07,
        verify_unnecessary_tiles_07,
        8
    ),
    (
        verify_replacement_number_08,
        verify_necessary_tiles_08,
        verify_unnecessary_tiles_08,
        9
    ),
    (
        verify_replacement_number_10,
        verify_necessary_tiles_10,
        verify_unnecessary_tiles_10,
        11
    ),
    (
        verify_replacement_number_11,
        verify_necessary_tiles_11,
        verify_unnecessary_tiles_11,
        12
    ),
    (
        verify_replacement_number_13,
        verify_necessary_tiles_13,
        verify_unnecessary_tiles_13,
        14
    ),
    (
        verify_replacement_number_14,
        verify_necessary_tiles_14,
        verify_unnecessary_tiles_14,
        15
    ),
);
