// SPDX-FileCopyrightText: 2026 Apricot S.
// SPDX-License-Identifier: MIT
// This file is part of https://github.com/Apricot-S/xiangting

#![cfg(not(feature = "build-map"))]

use rand::SeedableRng;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64Mcg;
use shanten_dp::{Mode, calc_shanten, calc_shanten2, make_tile_limits};
use xiangting::{
    PlayerCount, calculate_necessary_tiles, calculate_replacement_number,
    calculate_unnecessary_tiles,
};

#[test]
fn all_apis_match_independent_solver_for_both_rules_and_all_lengths() {
    let mut rng = Pcg64Mcg::seed_from_u64(42);
    for three in [false, true] {
        let rule = if three {
            PlayerCount::Three
        } else {
            PlayerCount::Four
        };
        let limits = make_tile_limits(three);
        for flush in [false, true] {
            let mut wall: Vec<usize> = (0..34)
                .filter(|&i| (!three || !(1..=7).contains(&i)) && (!flush || (9..18).contains(&i)))
                .flat_map(|i| [i; 4])
                .collect();
            for length in [1, 2, 4, 5, 7, 8, 10, 11, 13, 14] {
                for _ in 0..250 {
                    wall.shuffle(&mut rng);
                    let mut hand = [0; 34];
                    for &tile in &wall[..length] {
                        hand[tile] += 1;
                    }
                    let expected =
                        calc_shanten2(&hand, &limits, length / 3, Mode::all(), false, false)
                            .unwrap()
                            .unwrap();
                    let number = u8::try_from(expected.shanten + 1).unwrap();
                    assert_eq!(
                        calculate_replacement_number(&hand, rule).unwrap(),
                        number,
                        "{rule:?} {hand:?}"
                    );
                    assert_eq!(
                        calculate_necessary_tiles(&hand, rule).unwrap(),
                        (number, expected.waits),
                        "{rule:?} {hand:?}"
                    );
                    // Use scalar distances with a FIXED target meld count. The
                    // external solver's orphans discard mask omits simple tiles.
                    let mut discards = 0;
                    for tile in 0..34 {
                        if hand[tile] == 0 {
                            continue;
                        }
                        hand[tile] -= 1;
                        let distance =
                            calc_shanten(&hand, &limits, length / 3, Mode::all(), false, false)
                                .unwrap()
                                .unwrap()
                                + 1;
                        hand[tile] += 1;
                        if distance == i8::try_from(number).unwrap() {
                            discards |= 1 << tile;
                        }
                    }
                    assert_eq!(
                        calculate_unnecessary_tiles(&hand, rule).unwrap(),
                        (number, discards),
                        "{rule:?} {hand:?}"
                    );
                }
            }
        }
    }
}
