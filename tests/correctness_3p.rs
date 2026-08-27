// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

#![cfg(feature = "correctness")]

mod common;
mod hand_generator_3p;

mod tests {
    use crate::common::{
        NecessaryTiles, Player, ReplacementNumber, UnnecessaryTiles, VerificationTarget,
    };
    use crate::hand_generator_3p::{NUM_HANDS_3P, build_table_3p, decode_3p};

    fn verify_correctness_3p<const N: usize, T: VerificationTarget>() -> bool {
        let table = build_table_3p::<N>();

        crate::common::verify_correctness::<N, T, Player<true>, _>(
            NUM_HANDS_3P[N - 1],
            move |hash| decode_3p(hash, &table),
        )
    }

    macro_rules! define_correctness_tests {
        ($(($replacement:ident, $necessary:ident, $unnecessary:ident, $n:literal)),+ $(,)?) => {
            $(
                #[test]
                #[ignore = "exhaustive correctness check"]
                fn $replacement() {
                    assert!(
                        verify_correctness_3p::<$n, ReplacementNumber>(),
                        "There were mismatches."
                    );
                }

                #[test]
                #[ignore = "exhaustive correctness check"]
                fn $necessary() {
                    assert!(
                        verify_correctness_3p::<$n, NecessaryTiles>(),
                        "There were mismatches."
                    );
                }

                #[test]
                #[ignore = "exhaustive correctness check"]
                fn $unnecessary() {
                    assert!(
                        verify_correctness_3p::<$n, UnnecessaryTiles>(),
                        "There were mismatches."
                    );
                }
            )+
        };
    }

    define_correctness_tests!(
        (
            verify_replacement_number_3p_01,
            verify_necessary_tiles_3p_01,
            verify_unnecessary_tiles_3p_01,
            2
        ),
        (
            verify_replacement_number_3p_02,
            verify_necessary_tiles_3p_02,
            verify_unnecessary_tiles_3p_02,
            3
        ),
        (
            verify_replacement_number_3p_04,
            verify_necessary_tiles_3p_04,
            verify_unnecessary_tiles_3p_04,
            5
        ),
        (
            verify_replacement_number_3p_05,
            verify_necessary_tiles_3p_05,
            verify_unnecessary_tiles_3p_05,
            6
        ),
        (
            verify_replacement_number_3p_07,
            verify_necessary_tiles_3p_07,
            verify_unnecessary_tiles_3p_07,
            8
        ),
        (
            verify_replacement_number_3p_08,
            verify_necessary_tiles_3p_08,
            verify_unnecessary_tiles_3p_08,
            9
        ),
        (
            verify_replacement_number_3p_10,
            verify_necessary_tiles_3p_10,
            verify_unnecessary_tiles_3p_10,
            11
        ),
        (
            verify_replacement_number_3p_11,
            verify_necessary_tiles_3p_11,
            verify_unnecessary_tiles_3p_11,
            12
        ),
        (
            verify_replacement_number_3p_13,
            verify_necessary_tiles_3p_13,
            verify_unnecessary_tiles_3p_13,
            14
        ),
        (
            verify_replacement_number_3p_14,
            verify_necessary_tiles_3p_14,
            verify_unnecessary_tiles_3p_14,
            15
        ),
    );
}
